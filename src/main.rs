mod twasm;
mod utils;

use actix_multipart::form::{tempfile::TempFile, MultipartForm};
use actix_web::{get, middleware::Logger, post, web, App, HttpResponse, HttpServer, Responder};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use twasm::{run_wasm, run_wasm_function};
use utils::get_index;
use wasmtime::{Config, Engine, Linker, Module, Store};
use wasmtime_wasi::{preview1::WasiP1Ctx, WasiCtxBuilder};

#[derive(Debug, Serialize, Deserialize)]
struct Addition {
    func: String,
    x: i32,
    y: i32,
}

#[derive(MultipartForm, Debug)]
struct WasmModule {
    module: TempFile,
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

#[get("/wasm_exports/{index}")]
async fn wasm_idx(idx: web::Path<String>) -> impl Responder {
    let idx = idx.into_inner();
    let indx = format!("wasm/{}.wasm", idx);
    let exps: std::collections::HashMap<String, (Vec<wasmtime::ValType>, Vec<wasmtime::ValType>)> =
        match run_wasm(indx.as_str(), ().into(), ().into()).await {
            Ok(exp) => exp,
            Err(err) => return HttpResponse::InternalServerError().body(err.to_string()),
        };
    HttpResponse::Ok().body(format!("Module: {}\nExports: {:#?}\n", idx, exps))
    // HttpResponse::Ok().json(exps)
}

#[get("/wasmfun/{index}")]
async fn wasm_function(idx: web::Path<String>, fun: web::Json<Addition>) -> impl Responder {
    println!("API HIT");
    let idx = idx.into_inner();
    let idx = format!("wasm/{}.wasm", idx);
    let fun = fun.into_inner();

    let result = run_wasm_function(idx, fun.func, fun.x, fun.y)
        .await
        .unwrap();
    println!("Func Result = {:?}", result);

    HttpResponse::Ok().body(format!(
        "Return value from Wasm module function = {:?}",
        result
    ))
}

#[get("/function/{index}")]
async fn function_runner(idx: web::Path<String>, fun: web::Json<Addition>) -> impl Responder {
    println!("function API hit");
    let idx = format!("wasm/{}.wasm", idx.into_inner());
    let fun = fun.into_inner();
    let function = fun.func;
    let x = fun.x;
    let y = fun.y;

    match function.as_str() {
        "add" | "sub" | "mul" => {
            let mut config = Config::default();
            config.async_support(true);
            let engine = Engine::new(&config).unwrap();
            let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
            wasmtime_wasi::preview1::add_to_linker_async(&mut linker, |s| s).unwrap();

            let wasi = WasiCtxBuilder::new().inherit_stdio().build_p1();
            let mut store = Store::new(&engine, wasi);
            let module = Module::from_file(&engine, idx).unwrap();

            let func = linker
                .instantiate_async(&mut store, &module)
                .await
                .unwrap()
                .get_typed_func::<(i32, i32), i32>(&mut store, function.as_str())
                .expect(format!("Cannot find function {} in module", function).as_str());

            let result = func.call_async(&mut store, (x, y)).await.unwrap();

            HttpResponse::Ok().body(format!("{}", result))
        }
        "div" => {
            let mut config = Config::default();
            config.async_support(true);
            let engine = Engine::new(&config).unwrap();
            let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
            wasmtime_wasi::preview1::add_to_linker_async(&mut linker, |s| s).unwrap();

            let wasi = WasiCtxBuilder::new().inherit_stdio().build_p1();
            let mut store = Store::new(&engine, wasi);
            let module = Module::from_file(&engine, idx).unwrap();

            let func = linker
                .instantiate_async(&mut store, &module)
                .await
                .unwrap()
                .get_typed_func::<(i32, i32), f64>(&mut store, function.as_str())
                .expect(format!("Cannot find function {} in module", function).as_str());

            let result = func.call_async(&mut store, (x, y)).await.unwrap();

            HttpResponse::Ok().json(result)
        }
        _ => HttpResponse::BadRequest().body("Unknown_function"),
    }
}

#[post("/wasm/deploy")]
async fn wasm_deploy(MultipartForm(module): MultipartForm<WasmModule>) -> impl Responder {
    let idx = get_index();
    let _ = module
        .module
        .file
        .persist_noclobber(format!("wasm/{}.wasm", idx));

    HttpResponse::Ok().body(format!("Index of Wasm Module: {}", idx,))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::Builder::from_default_env()
        .filter_level(LevelFilter::Info)
        .init();
    HttpServer::new(|| {
        App::new()
            .wrap(Logger::default())
            .service(index)
            .service(wasm_idx)
            .service(wasm_deploy)
            .service(wasm_function)
            .service(function_runner)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
