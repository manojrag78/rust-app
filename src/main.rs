// src/main.rs
use actix_cors::Cors;
use actix_web::{App, HttpServer};
use dotenvy::dotenv;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    println!("🚀 Running at http://localhost:3001");
    dotenv().ok(); // Load .env

    HttpServer::new(|| {
        App::new()
            .wrap(
                Cors::default()
                    .allow_any_origin()
                    .allow_any_method()
                    .allow_any_header(),
            )
            .configure(routes::stream::config) // mount the /stream endpoint
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
