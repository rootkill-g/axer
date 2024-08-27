use serde_json::Value;
use std::{collections::HashMap, error::Error, fmt::Debug, ops::Add, path::PathBuf};
use wasmtime::{
    Config, Engine, ExportType, ExternType, FuncType, Linker, Module, Store, TypedFunc, Val,
    ValType,
};
use wasmtime_wasi::{preview1::WasiP1Ctx, WasiCtxBuilder};

pub async fn run_wasm(
    module_index: &str,
    params: Value,
    results: Value,
) -> Result<HashMap<String, (Vec<ValType>, Vec<ValType>)>, Box<dyn Error>> {
    let config = Config::default();
    // config.async_support(true);

    let engine = Engine::new(&config)?;
    // let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);

    // wasmtime_wasi::preview1::add_to_linker_async(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new().inherit_stdio().build_p1();
    // let mut store = Store::new(&engine, wasi);
    let module = Module::from_file(&engine, module_index)?;
    // let func = linker
    //     .module_async(&mut store, "", &module)
    //     .await?
    //     .get_default(&mut store, "")?;
    // let _ret = func.call_async(&mut store, &[], &mut []).await;
    // println!("Return Val: {:?}", _ret);

    // let mut exports: Vec<String> = vec![];
    // let mut tyes: Vec<ExternType> = vec![];

    // for exp in module.exports() {
    //     match exp.ty() {
    //         ExternType::Func(_) => {
    //             println!("Type: {:?}", exp.ty());
    //             exports.push(exp.name().to_string().to_owned())
    //         }
    //         _ => (),
    //     }
    // }

    let mut exports_with_params_and_return_types: std::collections::HashMap<
        String,
        (Vec<ValType>, Vec<ValType>),
    > = std::collections::HashMap::new();

    module.exports().for_each(|exp| match exp.ty() {
        ExternType::Func(f) => {
            // let params = f.params().map(|p| p.to_string()).collect();
            // let returns = f.results().map(|r| r.to_string()).collect();
            let params = f.params().collect();
            let returns = f.results().collect();

            exports_with_params_and_return_types.insert(exp.name().to_string(), (params, returns));
        }
        _ => (),
    });

    Ok(exports_with_params_and_return_types)
}

pub async fn run_wasm_function(
    idx: String,
    func: String,
    x: i32,
    y: i32,
) -> Result<wasmtime::Val, Box<dyn Error>> {
    println!("Func called");
    let mut config = Config::default();
    config.async_support(true);
    let engine = Engine::new(&config)?;
    let mut linker: Linker<WasiP1Ctx> = Linker::new(&engine);
    wasmtime_wasi::preview1::add_to_linker_async(&mut linker, |s| s)?;

    let wasi = WasiCtxBuilder::new().inherit_stdio().build_p1();
    let mut store = Store::new(&engine, wasi);
    let module = Module::from_file(&engine, idx)?;
    // let mut ret: [wasmtime::Val; 100] = if func != String::from("div") {
    //     [const { wasmtime::Val::I32(0) }; 100]
    // } else {
    //     [const { wasmtime::Val::F64(0) }; 100]
    // };

    if func != String::from("div") {
        let function = linker
            .instantiate_async(&mut store, &module)
            .await?
            .get_func(&mut store, func.as_str())
            .expect(format!("Cannot find function `{}` in module", func).as_str());

        let mut ret: [wasmtime::Val; 1] = [wasmtime::Val::I32(0); 1];

        let res = function
            .call_async(
                &mut store,
                &[wasmtime::Val::I32(x), wasmtime::Val::I32(y)],
                &mut ret,
            )
            .await?;

        println!("Rust Logged: RESULT = {:?}", ret);

        // Ok(ret.first().unwrap().to_owned())
        Ok(ret[0].clone())
    } else {
        let function = linker
            .instantiate_async(&mut store, &module)
            .await?
            .get_func(&mut store, func.as_str())
            .expect(format!("Cannot find function `{}` in module", func).as_str());

        let mut ret: [wasmtime::Val; 1] = [wasmtime::Val::F64(0); 1];

        let _ = function
            .call_async(
                &mut store,
                &[wasmtime::Val::I32(x), wasmtime::Val::I32(y)],
                &mut ret,
            )
            .await?;

        println!("Rust Logged: RESULT = {:?}", ret);

        // Ok(ret.first().unwrap().to_owned())
        Ok(ret[0].clone())
    }
}
