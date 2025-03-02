-- Add migration script here
CREATE TABLE wasm_modules(
  id TEXT PRIMARY KEY,
  name TEXT NOT NULL,
  wasm BLOB NOT NULL
)
