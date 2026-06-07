//! Wave 1, Phase 5: Bonsai Buddy Integration
//! Integration with standalone offline-first agent system

pub struct Buddy {
    id: String,
    state: BuddyState,
}

#[derive(Debug, Clone, PartialEq)]
pub enum BuddyState {
    Idle,
    Processing,
    Synchronized,
}

impl Buddy {
    pub fn new(id: String) -> Self {
        Self {
            id,
            state: BuddyState::Idle,
        }
    }

    pub fn activate(&mut self) {
        self.state = BuddyState::Processing;
    }

    pub fn sync(&mut self) {
        self.state = BuddyState::Synchronized;
    }

    pub fn state(&self) -> &BuddyState {
        &self.state
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
    fn test_buddy_lifecycle() {
        let mut buddy = Buddy::new("buddy1".to_string());
        assert_eq!(buddy.state(), &BuddyState::Idle);
        buddy.activate();
        assert_eq!(buddy.state(), &BuddyState::Processing);
        buddy.sync();
        assert_eq!(buddy.state(), &BuddyState::Synchronized);
    }

    #[test]
    fn test_buddy_id() {
        let buddy = Buddy::new("test_id".to_string());
        assert_eq!(buddy.id(), "test_id");
    }
}
