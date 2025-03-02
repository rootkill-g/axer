use rusqlite::{params, Connection, Result as AxerDBResult};
use ulid::Ulid;

use crate::dtos::WasmModule;

pub struct AxerDB(Connection);

impl AxerDB {
    pub fn new() -> Self {
        let mut conn = Connection::open("axer.db")
            .expect("FATAL: Error occured while initializing database! ABORTING");

        conn.set_transaction_behavior(rusqlite::TransactionBehavior::Immediate);

        conn.execute(
            "
                CREATE TABLE IF NOT EXISTS wasm_modules (
                    id TEXT PRIMARY KEY,
                    module_name TEXT NOT NULL,
                    mime_type TEXT NOT NULL,
                    bin_data BLOB NOT NULL
                )
            ",
            [],
        )
        .expect("Failed to create database table");

        AxerDB(conn)
    }

    pub fn write(&self, wasm_module: WasmModule) -> AxerDBResult<()> {
        let module_id = Ulid::new().to_string();

        let mut cached_statement = self.0.prepare_cached(
            "
                INSERT INTO wasm_modules 
                (id, module_name, mime_type, bin_data)
                VALUES (?1,  ?2, ?3, ?4)
            ",
        )?;

        cached_statement.execute(params![
            module_id,
            wasm_module.module_name,
            wasm_module.mime_type,
            wasm_module.bin_data
        ])?;

        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init_axer_db() {
        let axerdb = AxerDB::new();
        let mut row = axerdb
            .0
            .prepare("SELECT id, module_name, mime_type, bin_data FROM wasm_modules")
            .expect("Failed to get rows from table");

        assert_eq!(row.execute(()), Ok(0));
    }

    #[test]
    fn test_write() {
        let axerdb = AxerDB::new();
        let wat_bytes =
            b"(module (func (export \"test\") (param i32) (result i32) local.get 0))".to_vec();
        let wasm_module = WasmModule {
            id: Ulid::new().to_string(),
            module_name: String::from("test_module"),
            mime_type: String::from("wasm"),
            bin_data: wat_bytes,
        };

        assert_eq!(axerdb.write(wasm_module), Ok(()));
    }

    #[test]
    fn test_write_and_read() {
        let axerdb = AxerDB::new();
        let wat_bytes =
            b"(module (func (export \"test\") (param i32) (result i32) local.get 0))".to_vec();
        let module_id = Ulid::new().to_string();
        let wasm_module = WasmModule {
            id: module_id,
            module_name: String::from("test_module"),
            mime_type: String::from("wasm"),
            bin_data: wat_bytes,
        };

        assert_eq!(axerdb.write(wasm_module), Ok(()));

        assert_eq!(axerdb.read(module_id), Ok(wasm_module));
    }
}
