//! Integrated HDE Runtime
//! Combines AI Advisor, Safety Envelope, Model Framework, and Shadow Mode

use hde_ai_advisor::AdvisoryContext;
use hde_safety_envelope::SafetyConstraints;
use hde_model_framework::Model;
use hde_shadow_mode::ShadowExecution;

pub struct HdeRuntime {
    constraints: SafetyConstraints,
    context: Option<AdvisoryContext>,
    model: Option<Model>,
}

pub enum ExecutionResult {
    Success(Vec<u8>),
    SafetyViolation(String),
    ValidationFailure(String),
}

impl HdeRuntime {
    pub fn new(max_latency_ms: u64, max_memory_mb: u64) -> Self {
        Self {
            constraints: SafetyConstraints::new(max_latency_ms, max_memory_mb),
            context: None,
            model: None,
        }
    }

    pub fn set_advisor_context(&mut self, context: AdvisoryContext) {
        self.context = Some(context);
    }

    pub fn set_model(&mut self, model: Model) {
        self.model = Some(model);
    }

    pub fn execute(&self, baseline: Vec<u8>, latency: u64, memory: u64) -> ExecutionResult {
        // Check safety constraints
        if let Err(e) = self.constraints.check(latency, memory) {
            return ExecutionResult::SafetyViolation(e);
        }

        // Apply AI optimization if available
        let optimized = if self.context.is_some() && self.model.is_some() {
            // In production: use model for optimization
            baseline.clone()
        } else {
            baseline.clone()
        };

        // Validate in shadow mode
        let shadow = ShadowExecution::new(baseline, optimized.clone());
        match shadow.validate() {
            Ok(true) => ExecutionResult::Success(optimized),
            Ok(false) => ExecutionResult::ValidationFailure("shadow validation mismatch".to_string()),
            Err(e) => ExecutionResult::ValidationFailure(e),
        }
    }
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hde_runtime_creation() {
        let runtime = HdeRuntime::new(100, 512);
        let result = runtime.execute(vec![1, 2, 3], 50, 256);
        match result {
            ExecutionResult::Success(_) => {},
            _ => panic!("Expected success"),
        }
    }

    #[test]
    fn test_safety_constraint_violation() {
        let runtime = HdeRuntime::new(100, 512);
        let result = runtime.execute(vec![1, 2, 3], 150, 256);
        match result {
            ExecutionResult::SafetyViolation(_) => {},
            _ => panic!("Expected safety violation"),
        }
    }

    #[test]
    fn test_advisor_context() {
        let mut runtime = HdeRuntime::new(100, 512);
        runtime.set_advisor_context(AdvisoryContext::new("test-model".to_string()));
        assert!(runtime.context.is_some());
    }
}
