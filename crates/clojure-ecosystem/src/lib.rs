//! Wave 2, Phase 7: Clojure Ecosystem
//! Integration with Clojure libraries and ecosystem

pub struct EcosystemRegistry {
    libraries: Vec<String>,
}

impl EcosystemRegistry {
    pub fn new() -> Self {
        Self {
            libraries: Vec::new(),
        }
    }

    pub fn register(&mut self, name: String) {
        self.libraries.push(name);
    }

    pub fn list(&self) -> &[String] {
        &self.libraries
    }
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_registry() {
        let mut reg = EcosystemRegistry::new();
        assert_eq!(reg.list().len(), 0);
        reg.register("test".to_string());
        assert_eq!(reg.list().len(), 1);
    }
}
