mod storage;

use actix_web::{get, middleware::Logger, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct Addition {
    func: String,
    x: i32,
    y: i32,
}

#[derive(Debug, Serialize, Deserialize)]
struct FunctionRunner {
    module_index: i32,
    function_name: String,
    args: Vec<String>,
}

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
