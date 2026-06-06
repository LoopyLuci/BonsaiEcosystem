pub mod runtime;
pub mod pointer_table;
pub mod transaction;

pub use runtime::HotReloadRuntime;
pub use pointer_table::FunctionPointerTable;
pub use transaction::{AtomicTransaction, Snapshot};

use std::sync::OnceLock;
use parking_lot::RwLock;

static GLOBAL_RUNTIME: OnceLock<RwLock<HotReloadRuntime>> = OnceLock::new();

/// Get or initialize the global hot-reload runtime.
pub fn global_runtime() -> &'static RwLock<HotReloadRuntime> {
    GLOBAL_RUNTIME.get_or_init(|| RwLock::new(HotReloadRuntime::new()))
}

/// Register a function in the global pointer table.
pub fn register_function(name: &str, ptr: *const ()) {
    let runtime = global_runtime();
    runtime.write().register_function(name, ptr);
}

/// Retrieve a function pointer from the global pointer table.
pub fn get_function(name: &str) -> Option<*const ()> {
    let runtime = global_runtime();
    runtime.read().get_function(name)
}
