//! Wave 2, Phase 4: Clojure WASM
//! Compile Clojure to WebAssembly modules

pub struct WasmModule {
    binary: Vec<u8>,
}

impl WasmModule {
    pub fn new(binary: Vec<u8>) -> Self {
        Self { binary }
    }

    pub fn to_bytes(&self) -> &[u8] {
        &self.binary
    }
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_module_creation() {
        let m = WasmModule::new(vec![]);
        assert_eq!(m.to_bytes().len(), 0);
    }
}
