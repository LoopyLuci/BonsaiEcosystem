//! Wave 1, Phase 6: HDE Orchestrator
//! Orchestrates Hybrid Determinism Engine with AI advisor and safety constraints

pub struct Orchestrator {
    instances: Vec<HDEInstance>,
}

pub struct HDEInstance {
    id: String,
    ai_enabled: bool,
    safety_checks: bool,
}

impl HDEInstance {
    pub fn new(id: String) -> Self {
        Self {
            id,
            ai_enabled: true,
            safety_checks: true,
        }
    }

    pub fn toggle_ai(&mut self) {
        self.ai_enabled = !self.ai_enabled;
    }

    pub fn is_ai_enabled(&self) -> bool {
        self.ai_enabled
    }
}

impl Orchestrator {
    pub fn new() -> Self {
        Self {
            instances: Vec::new(),
        }
    }

    pub fn spawn(&mut self, id: String) -> &HDEInstance {
        self.instances.push(HDEInstance::new(id));
        &self.instances[self.instances.len() - 1]
    }

    pub fn count(&self) -> usize {
        self.instances.len()
    }
}

impl Default for Orchestrator {
    fn default() -> Self {
        Self::new()
    }
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_orchestrator() {
        let mut orch = Orchestrator::new();
        assert_eq!(orch.count(), 0);
        orch.spawn("hde1".to_string());
        assert_eq!(orch.count(), 1);
    }

    #[test]
    fn test_hde_instance() {
        let mut inst = HDEInstance::new("test".to_string());
        assert!(inst.is_ai_enabled());
        inst.toggle_ai();
        assert!(!inst.is_ai_enabled());
    }
}
