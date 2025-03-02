use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize)]
pub struct WasmModuleRegisterRequest {
    pub name: String,
    pub wasm: Vec<u8>,
}

#[derive(Debug, Serialize, Clone, PartialEq, FromRow)]
pub struct WasmModule {
    pub id: String,
    pub name: String,
    pub wasm: Vec<u8>,
}

#[derive(Debug, Serialize, FromRow)]
pub struct WasmModuleResponse {
    pub id: String,
    pub name: String,
}
