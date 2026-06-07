//! Wave 4, Component 1: Bonsai Buddy Agent
//! Standalone offline-first agent for system assistance

pub struct BuddyAgent {
    id: String,
    offline_mode: bool,
}

impl BuddyAgent {
    pub fn new(id: String) -> Self {
        Self {
            id,
            offline_mode: true,
        }
    }

    pub fn set_online(&mut self) {
        self.offline_mode = false;
    }

    pub fn is_offline(&self) -> bool {
        self.offline_mode
    }
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_agent() {
        let agent = BuddyAgent::new("buddy1".to_string());
        assert!(agent.is_offline());
    }
}
