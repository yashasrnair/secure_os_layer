use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, body::EitherBody,
};
use futures_util::future::{LocalBoxFuture, Ready, ready};
use sqlx::SqlitePool;
use std::sync::Arc;

/// A middleware that validates the X-App-ID header against the registered apps in the database.
pub struct PermissionMiddleware;

impl<S, B> Transform<S, ServiceRequest> for PermissionMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    // Specify the associated type Response for the transformed service.
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = PermissionMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        // Wrap the service in an Arc for cloneability.
        ready(Ok(PermissionMiddlewareMiddleware {
            service: Arc::new(service),
        }))
    }
}

pub struct PermissionMiddlewareMiddleware<S> {
    service: Arc<S>,
}

impl<S, B> Service<ServiceRequest> for PermissionMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    // Our future must be 'static.
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // --- Do header extraction outside the async block ---
        let app_id_opt = req
            .headers()
            .get("X-App-ID")
            .and_then(|val| val.to_str().ok())
            .map(|s| s.trim().to_string());

        if let Some(ref app_id) = app_id_opt {
            if app_id.is_empty() {
                let (req, _payload) = req.into_parts();
                let res = HttpResponse::Unauthorized().body("Missing or invalid X-App-ID");
                return Box::pin(async move {
                    Ok(ServiceResponse::new(req, res.map_into_right_body()))
                });
            }
        } else {
            let (req, _payload) = req.into_parts();
            let res = HttpResponse::Unauthorized().body("Missing or invalid X-App-ID");
            return Box::pin(async move {
                Ok(ServiceResponse::new(req, res.map_into_right_body()))
            });
        }

        // Now that we have a valid, owned app_id, unwrap it.
        let app_id = app_id_opt.unwrap();

        // Clone the pool from app data.
        let pool = req
            .app_data::<actix_web::web::Data<SqlitePool>>()
            .cloned();

        // Clone the inner service from the Arc.
        let svc = self.service.clone();

        // Now we can move everything into the async block.
        Box::pin(async move {
            // Validate the app_id using the database.
            if let Some(pool) = pool {
                let query = "SELECT COUNT(*) as count FROM registered_apps WHERE app_id = ?";
                let result = sqlx::query_scalar::<_, i64>(query)
                    .bind(&app_id)
                    .fetch_one(pool.get_ref())
                    .await;

                match result {
                    Ok(count) if count > 0 => {
                        // Valid app ID; continue processing.
                    }
                    _ => {
                        let (req, _payload) = req.into_parts();
                        let res = HttpResponse::Unauthorized().body("Unregistered App ID");
                        return Ok(ServiceResponse::new(req, res.map_into_right_body()));
                    }
                }
            } else {
                let (req, _payload) = req.into_parts();
                let res = HttpResponse::InternalServerError().body("Missing DB pool");
                return Ok(ServiceResponse::new(req, res.map_into_right_body()));
            }

            // All checks passed; now call the cloned inner service.
            let res = svc.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}
