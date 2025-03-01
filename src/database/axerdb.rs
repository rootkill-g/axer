use rusqlite::{params, Connection, Error as AxerDBError, Result as AxerDBResult};
use ulid::Ulid;

use crate::dtos::WasmModule;

pub struct AxerDB(Connection);

impl AxerDB {
    pub fn initialize_axerdb() -> Self {
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
