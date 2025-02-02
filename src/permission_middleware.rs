use actix_web::{
    dev::{forward_ready, Service, ServiceRequest, ServiceResponse, Transform},
    Error, HttpResponse, body::EitherBody,
};
use futures_util::future::{LocalBoxFuture, Ready, ready};
use sqlx::SqlitePool;

/// A middleware that validates the X-App-ID header against the registered apps in the database.
pub struct PermissionMiddleware;

impl<S, B> Transform<S, ServiceRequest> for PermissionMiddleware
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type InitError = ();
    type Transform = PermissionMiddlewareMiddleware<S>;
    type Future = Ready<Result<Self::Transform, Self::InitError>>;

    fn new_transform(&self, service: S) -> Self::Future {
        ready(Ok(PermissionMiddlewareMiddleware { service }))
    }
}

pub struct PermissionMiddlewareMiddleware<S> {
    service: S,
}

impl<S, B> Service<ServiceRequest> for PermissionMiddlewareMiddleware<S>
where
    S: Service<ServiceRequest, Response = ServiceResponse<B>, Error = Error> + 'static,
    B: 'static,
{
    type Response = ServiceResponse<EitherBody<B>>;
    type Error = Error;
    type Future = LocalBoxFuture<'static, Result<Self::Response, Self::Error>>;

    forward_ready!(service);

    fn call(&self, req: ServiceRequest) -> Self::Future {
        // Extract the X-App-ID header.
        let app_id = req
            .headers()
            .get("X-App-ID")
            .and_then(|val| val.to_str().ok())
            .map(|s| s.trim().to_string());

        // Get the connection pool from the app data.
        let pool = req.app_data::<actix_web::web::Data<SqlitePool>>().cloned();

        Box::pin(async move {
            // Check that we have a non-empty app_id.
            let app_id = match app_id {
                Some(a) if !a.is_empty() => a,
                _ => {
                    let (req, _payload) = req.into_parts();
                    let res = HttpResponse::Unauthorized().body("Missing or invalid X-App-ID");
                    return Ok(ServiceResponse::new(req, res.map_into_right_body()));
                }
            };

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
                // If no pool is provided, return an error.
                let (req, _payload) = req.into_parts();
                let res = HttpResponse::InternalServerError().body("Missing DB pool");
                return Ok(ServiceResponse::new(req, res.map_into_right_body()));
            }

            // Continue to the next service/middleware.
            let res = self.service.call(req).await?;
            Ok(res.map_into_left_body())
        })
    }
}
