use sqlx::{self, Result as AxerDBResult, SqlitePool};

use crate::dtos::{WasmModule, WasmModuleResponse};

pub struct AxerDBPool(pub(crate) SqlitePool);

impl AxerDBPool {
    pub async fn new() -> AxerDBPool {
        let cpool = SqlitePool::connect("sqlite://axer.db")
            .await
            .expect("Failed to connect to database");

        AxerDBPool(cpool)
    }

    pub async fn write(&self, wasm_module: WasmModule) -> AxerDBResult<WasmModuleResponse> {
        match sqlx::query_as::<_, WasmModuleResponse>(
            "
            INSERT INTO wasm_modules (id, name, wasm)
            VALUES (?, ?, ?)
            RETURNING id, name
        ",
        )
        .bind(wasm_module.id)
        .bind(wasm_module.name.clone())
        .bind(wasm_module.wasm)
        .fetch_one(&self.0)
        .await
        {
            Ok(query_result) => {
                let response = query_result.into();

                Ok(response)
            }
            Err(err) => Err(err),
        }
    }

    pub async fn read(&self, module_id: String) -> AxerDBResult<WasmModule> {
        Ok(sqlx::query_as::<_, WasmModule>(
            "
            SELECT id, name, wasm
            FROM wasm_modules
            WHERE id = ?
        ",
        )
        .bind(&module_id)
        .fetch_one(&self.0)
        .await
        .unwrap())
    }
}

#[cfg(test)]
mod tests {
    use crate::test_utils::init_axerdb_test_pool;

    use super::*;
    use sqlx::Row;
    use ulid::Ulid;

    #[tokio::test]
    async fn test_init_axer_db() {
        let cpool = init_axerdb_test_pool().await;
        let query = "SELECT COUNT(*) FROM wasm_modules";

        let res = sqlx::query(query).fetch_one(&cpool.0).await.unwrap();

        assert!(!res.is_empty());
    }

    #[tokio::test]
    async fn test_write() {
        let cpool = init_axerdb_test_pool().await;
        let wat_bytes =
            b"(module (func (export \"test\") (param i32) (result i32) local.get 0))".to_vec();
        let wasm_module = WasmModule {
            id: Ulid::new().to_string(),
            name: String::from("test_module").into(),
            wasm: wat_bytes,
        };

        assert!(cpool.write(wasm_module).await.is_ok());
    }

    #[tokio::test]
    async fn test_write_and_read() {
        let cpool = init_axerdb_test_pool().await;
        let wat_bytes =
            b"(module (func (export \"test\") (param i32) (result i32) local.get 0))".to_vec();
        let module_id = Ulid::new().to_string();
        let wasm_module = WasmModule {
            id: module_id.clone(),
            name: String::from("test_module").into(),
            wasm: wat_bytes,
        };

        assert!(cpool.write(wasm_module.clone()).await.is_ok());
        assert!(cpool.read(module_id.into()).await.is_ok());
    }
}
