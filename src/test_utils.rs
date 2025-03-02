#![cfg(test)]

use sqlx::SqlitePool;

use crate::database::axerdb::AxerDBPool;

pub async fn init_axerdb_test_pool() -> AxerDBPool {
    let cpool = AxerDBPool(
        SqlitePool::connect("sqlite::memory:")
            .await
            .expect("Failed to connect to database"),
    );

    sqlx::query(
        "
            CREATE TABLE wasm_modules (
                id TEXT PRIMARY KEY,
                name TEXT NOT NULL,
                wasm BLOB NOT NULL
            )
        ",
    )
    .execute(&cpool.0)
    .await
    .expect("Failed to run migration for tests");

    cpool
}
