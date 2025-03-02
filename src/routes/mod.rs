pub mod index;
pub mod wasm;

// Re-exports
pub use index::root;
pub use wasm::register::register_wasm_module;
