use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct WasmModule {
    pub id: String,
    pub module_name: String,
    pub mime_type: String,
    pub bin_data: Vec<u8>,
}
