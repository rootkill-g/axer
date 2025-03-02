mod database;
mod dtos;
mod routes;

use actix_web::{middleware::Logger, web, App, HttpServer};
use routes::{register_wasm_module, root};

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
            .service(root)
            .service(register_wasm_module)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
