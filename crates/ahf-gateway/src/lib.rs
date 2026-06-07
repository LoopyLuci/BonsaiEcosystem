//! AHF Gateway - main orchestrator
use ahf_arbiter::Arbiter;
use ahf_bias_detector::BiasDetector;
use ahf_core::{
    apply_safety_envelope, extract_claims, OutputResult, Result,
    FormalVerificationResult,
};
use ahf_formal_verifier::FormalVerifier;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GatewayConfig {
    pub default_schema: String,
    pub ai_enhanced_parsing: bool,
    pub max_concurrency: usize,
    pub session_ttl_secs: u64,
}

impl Default for GatewayConfig {
    fn default() -> Self {
        Self {
            default_schema: "text".into(),
            ai_enhanced_parsing: false,
            max_concurrency: 100,
            session_ttl_secs: 3600,
        }
    }
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct PipelineStats {
    pub total_processed: u64,
    pub accepted: u64,
    pub rejected: u64,
    pub avg_latency_us: u64,
}

pub struct AhfGateway {
    verifier: Arc<FormalVerifier>,
    arbiter: Arc<Arbiter>,
    bias: Arc<BiasDetector>,
    config: Arc<RwLock<GatewayConfig>>,
    stats: Arc<RwLock<PipelineStats>>,
}

impl AhfGateway {
    pub fn new(
        verifier: Arc<FormalVerifier>,
        arbiter: Arc<Arbiter>,
        bias: Arc<BiasDetector>,
    ) -> Self {
        Self {
            verifier,
            arbiter,
            bias,
            config: Arc::new(RwLock::new(GatewayConfig::default())),
            stats: Arc::new(RwLock::new(PipelineStats::default())),
        }
    }

    pub async fn process_output(
        &self,
        raw_output: &str,
        schema_name: &str,
        model_confidence: f32,
        session_id: Option<&str>,
    ) -> Result<OutputResult> {
        let start = std::time::Instant::now();

        if raw_output.is_empty() {
            return Ok(OutputResult::reject(
                "Empty output received",
                "I cannot process an empty response.",
            ));
        }

        let claims = extract_claims(raw_output);

        let grounding_score = if claims.is_empty() {
            1.0
        } else {
            0.5
        };

        let verification = self
            .verifier
            .validate(raw_output, schema_name, session_id)
            .await
            .unwrap_or_else(|e| {
                FormalVerificationResult::Invalid(vec![e.to_string()])
            });

        let bias_report = self.bias.detect(raw_output).await;

        let decision = self
            .arbiter
            .decide(grounding_score, &verification, bias_report.bias_score, model_confidence)
            .await;

        let final_output = if decision.accepted {
            apply_safety_envelope(raw_output, &decision.safety_envelope)
        } else {
            decision.fallback.clone()
        };

        {
            let mut stats = self.stats.write().await;
            stats.total_processed += 1;
            if decision.accepted {
                stats.accepted += 1;
            } else {
                stats.rejected += 1;
            }
            let elapsed_us = start.elapsed().as_micros() as u64;
            stats.avg_latency_us =
                (stats.avg_latency_us * (stats.total_processed - 1) + elapsed_us)
                    / stats.total_processed;
        }

        Ok(OutputResult {
            accepted: decision.accepted,
            final_output,
            grounding_score,
            verification_result: verification,
            bias_detected: bias_report.bias_score > 0.5,
            confidence: model_confidence,
            reason: decision.reason,
            fallback_used: !decision.accepted,
        })
    }

    pub async fn process_output_default(
        &self,
        raw_output: &str,
        model_confidence: f32,
    ) -> Result<OutputResult> {
        let config = self.config.read().await;
        self.process_output(raw_output, &config.default_schema, model_confidence, None)
            .await
    }

    pub async fn update_config(&self, new_config: GatewayConfig) {
        let mut config = self.config.write().await;
        *config = new_config;
    }

    pub async fn stats(&self) -> PipelineStats {
        self.stats.read().await.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    async fn create_test_gateway() -> AhfGateway {
        let verifier = Arc::new(FormalVerifier::new());
        let arbiter = Arc::new(Arbiter::new());
        let bias = Arc::new(BiasDetector::new());
        AhfGateway::new(verifier, arbiter, bias)
    }

    #[tokio::test]
    async fn test_empty_output_rejected() {
        let gateway = create_test_gateway().await;
        let result = gateway.process_output_default("", 0.95).await.unwrap();
        assert!(!result.accepted);
    }

    #[tokio::test]
    async fn test_biased_output_blocked() {
        let gateway = create_test_gateway().await;
        let result = gateway
            .process_output_default("Women are not good at math.", 0.99)
            .await
            .unwrap();
        assert!(!result.accepted);
    }
}
