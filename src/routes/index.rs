use actix_web::{get, HttpResponse, Responder};

#[get("/")]
pub async fn root() -> impl Responder {
    HttpResponse::Ok().body("INDEX=>HIT_SUCCESS")
}

#[cfg(test)]
mod tests {
    use super::*;
    use actix_web::{test, App};

    #[actix_web::test]
    async fn test_route_root() {
        let app = test::init_service(App::new().service(root)).await;
        let req = test::TestRequest::get().uri("/").to_request();
        let res = test::call_service(&app, req).await;

        assert!(res.status().is_success())
    }
}
