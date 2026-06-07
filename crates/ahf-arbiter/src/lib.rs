//! Arbiter - decision engine for AHF
use ahf_core::{
    ArbiterDecision, FormalVerificationResult,
    SafetyEnvelope,
};
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbiterConfig {
    pub grounding_threshold: f64,
    pub confidence_threshold: f32,
    pub bias_threshold: f32,
    pub critical_requires_full_grounding: bool,
}

impl Default for ArbiterConfig {
    fn default() -> Self {
        Self {
            grounding_threshold: 0.9,
            confidence_threshold: 0.8,
            bias_threshold: 0.1,
            critical_requires_full_grounding: true,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Policy {
    pub version: String,
    pub config: ArbiterConfig,
    pub safety_envelope: SafetyEnvelope,
    pub fallback_responses: Vec<FallbackResponse>,
    pub council_signature: Option<Vec<u8>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FallbackResponse {
    pub pattern: String,
    pub response: String,
}

impl Policy {
    pub fn default_policy() -> Self {
        Self {
            version: "1.0.0".into(),
            config: ArbiterConfig::default(),
            safety_envelope: SafetyEnvelope::default(),
            fallback_responses: vec![
                FallbackResponse {
                    pattern: "grounding".into(),
                    response: "I cannot provide a verified answer based on available knowledge sources.".into(),
                },
                FallbackResponse {
                    pattern: "confidence".into(),
                    response: "I'm not confident enough in this response.".into(),
                },
                FallbackResponse {
                    pattern: "bias".into(),
                    response: "I cannot provide an answer that may contain biased content.".into(),
                },
            ],
            council_signature: None,
        }
    }

    pub fn find_fallback(&self, reason_type: &str) -> String {
        self.fallback_responses
            .iter()
            .find(|f| reason_type.contains(&f.pattern))
            .map(|f| f.response.clone())
            .unwrap_or_else(|| "I cannot provide a verified answer at this time.".into())
    }
}

pub struct Arbiter {
    policy: Arc<RwLock<Policy>>,
}

impl Arbiter {
    pub fn new() -> Self {
        Self {
            policy: Arc::new(RwLock::new(Policy::default_policy())),
        }
    }

    pub async fn update_policy(&self, new_policy: Policy) {
        let mut policy = self.policy.write().await;
        *policy = new_policy;
    }

    pub async fn decide(
        &self,
        grounding_score: f64,
        verification: &FormalVerificationResult,
        bias_score: f32,
        model_confidence: f32,
    ) -> ArbiterDecision {
        let policy = self.policy.read().await;

        if !verification.is_valid() {
            return ArbiterDecision {
                accepted: false,
                reason: "Formal verification failed".into(),
                safety_envelope: policy.safety_envelope.clone(),
                fallback: policy.find_fallback("verification"),
            };
        }

        if grounding_score < policy.config.grounding_threshold {
            return ArbiterDecision {
                accepted: false,
                reason: format!("Grounding score {:.2} below threshold", grounding_score),
                safety_envelope: policy.safety_envelope.clone(),
                fallback: policy.find_fallback("grounding"),
            };
        }

        if bias_score > policy.config.bias_threshold {
            return ArbiterDecision {
                accepted: false,
                reason: format!("Bias score {:.2} exceeds threshold", bias_score),
                safety_envelope: policy.safety_envelope.clone(),
                fallback: policy.find_fallback("bias"),
            };
        }

        if model_confidence < policy.config.confidence_threshold {
            return ArbiterDecision {
                accepted: false,
                reason: format!("Model confidence {:.2} below threshold", model_confidence),
                safety_envelope: policy.safety_envelope.clone(),
                fallback: policy.find_fallback("confidence"),
            };
        }

        ArbiterDecision {
            accepted: true,
            reason: "All checks passed".into(),
            safety_envelope: policy.safety_envelope.clone(),
            fallback: String::new(),
        }
    }

    pub async fn safety_envelope(&self) -> SafetyEnvelope {
        self.policy.read().await.safety_envelope.clone()
    }
}

impl Default for Arbiter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_accept_high_confidence() {
        let arbiter = Arbiter::new();
        let decision = arbiter
            .decide(1.0, &FormalVerificationResult::Valid, 0.0, 0.95)
            .await;
        assert!(decision.accepted);
    }

    #[tokio::test]
    async fn test_reject_low_grounding() {
        let arbiter = Arbiter::new();
        let decision = arbiter
            .decide(0.3, &FormalVerificationResult::Valid, 0.0, 0.95)
            .await;
        assert!(!decision.accepted);
    }
}
