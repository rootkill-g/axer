mod database;
mod dtos;

use actix_web::{get, middleware::Logger, web, App, HttpResponse, HttpServer, Responder};

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
    let cpool = database::axerdb::AxerDBPool::new().await;
    let axerdb = web::Data::new(cpool);

    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::clone(&axerdb))
            .wrap(Logger::default())
            .service(index)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
