use actix_web::{post, web, HttpResponse, Responder};
use ulid::Ulid;

use crate::{
    database::axerdb::AxerDBPool,
    dtos::{WasmModule, WasmModuleRegisterRequest},
};

#[post("/wasm")]
pub async fn register_wasm_module(
    payload: web::Json<WasmModuleRegisterRequest>,
    axerdb: web::Data<AxerDBPool>,
) -> impl Responder {
    tracing::info!("Module to register: {:?}", payload);
    // For the borrow checker
    let payload = payload.into_inner();
    let wasm_module = WasmModule {
        id: Ulid::new().to_string(),
        name: payload.name,
        wasm: payload.wasm,
    };

    match axerdb.write(wasm_module).await {
        Ok(result) => HttpResponse::Created().json(result),
        Err(err) => {
            tracing::error!("ERROR OCCURED={:?}", err);

            HttpResponse::InternalServerError().finish()
        }
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::init_axerdb_test_pool;

    use super::*;
    use actix_web::{
        http::{header::ContentType, StatusCode},
        test, App,
    };
    use serde_json::json;

    #[actix_web::test]
    async fn test_route_register_wasm_module() {
        let axerdb = web::Data::new(init_axerdb_test_pool().await);
        let app = test::init_service(
            App::new()
                .app_data(web::Data::clone(&axerdb))
                .service(register_wasm_module),
        )
        .await;
        let wasm_module_register_request = WasmModuleRegisterRequest {
            name: String::from("Test Wasm Module"),
            wasm: b"(module (func (export \"test\") (param i32) (result i32) local.get 0))"
                .to_vec(),
        };
        let payload = json!(&wasm_module_register_request);
        let req = test::TestRequest::post()
            .uri("/wasm")
            .insert_header(ContentType::json())
            .set_json(&payload)
            .to_request();
        let res = test::call_service(&app, req).await;

        assert_eq!(res.status(), StatusCode::CREATED)
    }
}
