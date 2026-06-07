//! Wave 2, Phase 2: Verified Titan Core
//! Persistent data structures with formal verification
//!
//! Implements O(1) operations on:
//! - Persistent Vector (structural sharing)
//! - Hash Map (HAMT)
//! - Concurrency primitives (atom, ref, agent, var)
//! - Axiom proof sketches for correctness

pub mod vector;
pub mod hashmap;
pub mod concurrency;
pub mod proofs;
pub mod var;

pub use vector::PersistentVector;
pub use hashmap::PersistentHashMap;
pub use concurrency::{Atom, Ref, Agent};
pub use var::Var;

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize Titan core runtime
pub fn init() {
    let _ = env_logger::builder().try_init();
    log::info!("Titan core initialized (v{})", VERSION);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_init() {
        init();
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_vector_creation() {
        let v: PersistentVector<i32> = PersistentVector::new();
        assert_eq!(v.len(), 0);
    }

    #[test]
    fn test_hashmap_creation() {
        let m: PersistentHashMap<String, i32> = PersistentHashMap::new();
        assert_eq!(m.len(), 0);
    }
}
