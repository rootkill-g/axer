use serde::{Deserialize, Serialize};
use sqlx::FromRow;

#[derive(Debug, Serialize, Deserialize, PartialEq, Clone, FromRow)]
pub struct WasmModule {
    pub id: String,
    pub name: String,
    pub wasm: Vec<u8>,
}
