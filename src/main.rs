use actix_web::{HttpServer, App, web};
use sqlx::postgres::PgPoolOptions;
use std::io::Result;
use std::env;
use dotenvy::dotenv; 

mod api;

#[actix_web::main]
async fn main() -> Result<()> {
    // Load database url from .env file
    dotenv().ok(); 
    let database_url = env::var("DATABASE_URL")
        .expect("DATABASE_URL must be set in your environment variables.");

    
    // get a connection pool
    let pool = PgPoolOptions::new()
        .max_connections(5)
        .connect(&database_url)
        .await
        .expect("Failed to create SQLx pool.");

    println!("Server running at http://127.0.0.1:8080");

    // start the server
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(pool.clone()))
            .configure(api::config)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}