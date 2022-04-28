use std::env;

use actix_cors::Cors;
use actix_web::{http, web, App, HttpServer};
use entity::sea_orm::{Database, DatabaseConnection};

mod handlers;

#[derive(Debug, Clone)]
pub struct AppState {
    pub db_connection: DatabaseConnection,
}

impl AppState {
    pub fn new(db_connection: DatabaseConnection) -> Self {
        Self { db_connection }
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv::dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set in .env file");
    let db_connection = Database::connect(&db_url)
        .await
        .expect("Failed to connect to database");

    let app_state = AppState::new(db_connection);

    // std::env::set_var("RUST_LOG", "actix-web=debug");
    HttpServer::new(move || {
        let cors = Cors::default()
            .allowed_origin("http://localhost:3000")
            .allowed_methods(vec!["GET", "POST", "PUT", "PATCH", "DELETE"])
            .allowed_headers(vec![http::header::AUTHORIZATION, http::header::ACCEPT])
            .allowed_header(http::header::CONTENT_TYPE);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(app_state.clone()))
        // .service(web::scope("/api").configure(handlers::config::config))
    })
    .bind(("localhost", 8080))?
    .run()
    .await
}
