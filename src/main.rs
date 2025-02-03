use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use sqlx::sqlite::SqlitePoolOptions;
use uuid::Uuid;

// Import the custom permission middleware.
mod permission_middleware;
use permission_middleware::PermissionMiddleware;

#[derive(Serialize)]
struct StatusResponse {
    status: String,
    message: String,
}

// API model for user data.
#[derive(Deserialize, Serialize)]
struct UserData {
    id: Uuid,
    key: String,
    value: String,
}

// Database model for user data.
#[derive(sqlx::FromRow)]
struct DBUserData {
    id: String,
    key: String,
    value: String,
}

// Database model for a registered app.
#[derive(sqlx::FromRow, Serialize)]
struct RegisteredApp {
    app_id: String,
    app_name: String,
    allowed_permissions: Option<String>,
}

// New database model for an installed app.
#[derive(sqlx::FromRow, Serialize)]
struct InstalledApp {
    install_id: i64,
    app_id: String,
    install_date: String,
}

async fn status() -> impl Responder {
    HttpResponse::Ok().json(StatusResponse {
        status: "ok".into(),
        message: "Secure OS Layer is running".into(),
    })
}

async fn add_data(
    data: web::Json<UserData>,
    pool: web::Data<sqlx::SqlitePool>,
) -> impl Responder {
    let query = "INSERT INTO user_data (id, key, value) VALUES (?, ?, ?)";
    match sqlx::query(query)
        .bind(data.id.to_string())
        .bind(&data.key)
        .bind(&data.value)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().json("Data added"),
        Err(e) => {
            eprintln!("Error adding data: {:?}", e);
            HttpResponse::InternalServerError().finish()
        }
    }
}

async fn get_data(
    path: web::Path<String>,
    pool: web::Data<sqlx::SqlitePool>,
) -> impl Responder {
    let id = path.into_inner();
    let query = "SELECT id, key, value FROM user_data WHERE id = ?";
    match sqlx::query_as::<_, DBUserData>(query)
        .bind(&id)
        .fetch_one(pool.get_ref())
        .await
    {
        Ok(db_user_data) => {
            match Uuid::parse_str(&db_user_data.id) {
                Ok(uuid) => {
                    let user_data = UserData {
                        id: uuid,
                        key: db_user_data.key,
                        value: db_user_data.value,
                    };
                    HttpResponse::Ok().json(user_data)
                }
                Err(e) => {
                    eprintln!("Error parsing UUID: {:?}", e);
                    HttpResponse::InternalServerError().body("Invalid UUID in DB")
                }
            }
        }
        Err(e) => {
            eprintln!("Error fetching data: {:?}", e);
            HttpResponse::NotFound().body("Data not found")
        }
    }
}

// New endpoint: List registered apps.
async fn get_apps(pool: web::Data<sqlx::SqlitePool>) -> impl Responder {
    let query = "SELECT app_id, app_name, allowed_permissions FROM registered_apps";
    match sqlx::query_as::<_, RegisteredApp>(query)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(apps) => HttpResponse::Ok().json(apps),
        Err(e) => {
            eprintln!("Error fetching apps: {:?}", e);
            HttpResponse::InternalServerError().body("Error fetching apps")
        }
    }
}

// New endpoint: Install an app.
#[derive(Deserialize)]
struct InstallRequest {
    app_id: String,
}

async fn install_app(
    data: web::Json<InstallRequest>,
    pool: web::Data<sqlx::SqlitePool>,
) -> impl Responder {
    // Insert an installation record.
    let query = "INSERT INTO installed_apps (app_id) VALUES (?)";
    match sqlx::query(query)
        .bind(&data.app_id)
        .execute(pool.get_ref())
        .await
    {
        Ok(_) => HttpResponse::Ok().json("App installed"),
        Err(e) => {
            eprintln!("Error installing app: {:?}", e);
            HttpResponse::InternalServerError().body("Failed to install app")
        }
    }
}

// Optional: Endpoint to list installed apps.
async fn get_installed_apps(pool: web::Data<sqlx::SqlitePool>) -> impl Responder {
    let query = "SELECT install_id, app_id, install_date FROM installed_apps";
    match sqlx::query_as::<_, InstalledApp>(query)
        .fetch_all(pool.get_ref())
        .await
    {
        Ok(installed) => HttpResponse::Ok().json(installed),
        Err(e) => {
            eprintln!("Error fetching installed apps: {:?}", e);
            HttpResponse::InternalServerError().body("Error fetching installed apps")
        }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Create the SQLite connection pool.
    let pool = SqlitePoolOptions::new()
        .max_connections(5)
        .connect("sqlite://secure_os_layer.db")
        .await
        .expect("Failed to connect to the database");

    // Insert a test registered app into the 'registered_apps' table if it doesn't exist.
    let test_app_id = "my-app-id";
    let insert_query = "INSERT OR IGNORE INTO registered_apps (app_id, app_name, allowed_permissions) VALUES (?, ?, ?)";
    sqlx::query(insert_query)
        .bind(test_app_id)
        .bind("Test App")
        .bind("all")
        .execute(&pool)
        .await
        .expect("Failed to insert test app");

    HttpServer::new(move || {
        App::new()
            // Make the SQLite pool available as global state.
            .app_data(web::Data::new(pool.clone()))
            // Apply the permission middleware globally.
            .wrap(PermissionMiddleware)
            .route("/status", web::get().to(status))
            .route("/data", web::post().to(add_data))
            .route("/data/{id}", web::get().to(get_data))
            .route("/apps", web::get().to(get_apps))
            .route("/install", web::post().to(install_app))
            .route("/installed", web::get().to(get_installed_apps))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
