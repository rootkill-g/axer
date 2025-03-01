use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
struct FunctionRunner {
    module_index: i32,
    function_name: String,
    args: Vec<String>,
}
