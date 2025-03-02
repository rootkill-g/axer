use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok().body("INDEX=>HIT_SUCCESS")
}
