mod dtos;
mod storage;

use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("INDEX=>HIT_SUCCESS")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // Initialize tracing subscriber for logging
    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        .init();

    tracing::info!("Initializing database...");

    HttpServer::new(|| App::new().wrap(Logger::default()).service(index))
        .bind(("127.0.0.1", 8080))?
        .run()
        .await
}
