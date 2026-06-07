//! Wave 2, Phase 5: Aether Clojure
//! Distributed actor framework for Clojure

pub struct Actor {
    id: String,
}

impl Actor {
    pub fn new(id: String) -> Self {
        Self { id }
    }

    pub fn id(&self) -> &str {
        &self.id
    }
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_actor_creation() {
        let a = Actor::new("test".to_string());
        assert_eq!(a.id(), "test");
    }
}
