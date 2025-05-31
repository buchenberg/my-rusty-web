mod config;
mod db;
mod errors;
mod models;
mod handlers;

use actix_web::{web, App, HttpServer};
use db::Database;
use crate::config::config::Config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let config = Config::default();

    // Initialize database
    let database = Database::new(&config.database_url)
        .expect("Failed to create database connection");
    database.init().expect("Failed to initialize database");

    let database = web::Data::new(database);

    println!("Server running at http://{}", config.server_addr);

    HttpServer::new(move || {
        App::new()
            .app_data(database.clone())
            .route("/routes", web::post().to(handlers::handlers::create_route))
            .route("/routes", web::get().to(handlers::handlers::get_all_routes))
            .route("/routes/{id}", web::get().to(handlers::handlers::get_route))
            .route("/routes/{id}", web::put().to(handlers::handlers::update_route))
            .route("/routes/{id}", web::delete().to(handlers::handlers::delete_route))
    })
        .bind(&config.server_addr)?
        .run()
        .await
}