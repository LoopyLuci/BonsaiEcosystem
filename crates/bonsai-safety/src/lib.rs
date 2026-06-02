pub mod pii_redactor;
pub mod refusal_classifier;
pub mod watermark;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyConfig {
    pub enable_refusal_classifier: bool,
    pub enable_pii_redaction: bool,
    pub enable_watermarking: bool,
    pub refusal_threshold: f32,
}

impl Default for SafetyConfig {
    fn default() -> Self {
        Self {
            enable_refusal_classifier: true,
            enable_pii_redaction: true,
            enable_watermarking: true,
            refusal_threshold: 0.5,
        }
    }
}

pub struct GuardrailEngine {
    config: SafetyConfig,
    classifier: refusal_classifier::RefusalClassifier,
    redactor: pii_redactor::PiiRedactor,
    watermarker: watermark::Watermarker,
}

impl GuardrailEngine {
    pub fn new(config: SafetyConfig) -> anyhow::Result<Self> {
        Ok(Self {
            classifier: refusal_classifier::RefusalClassifier::new()?,
            redactor: pii_redactor::PiiRedactor::new(),
            watermarker: watermark::Watermarker::new()?,
            config,
        })
    }

    pub fn check_prompt(&self, prompt: &str) -> anyhow::Result<SafetyCheck> {
        if !self.config.enable_refusal_classifier {
            return Ok(SafetyCheck {
                allowed: true,
                risk: 0.0,
            });
        }
        self.classifier.classify(prompt)
    }

    pub fn check_output(&self, output: &str) -> anyhow::Result<SafetyCheck> {
        if !self.config.enable_refusal_classifier {
            return Ok(SafetyCheck {
                allowed: true,
                risk: 0.0,
            });
        }
        self.classifier.classify(output)
    }

    pub fn redact_output(&self, output: &str) -> String {
        if !self.config.enable_pii_redaction {
            return output.to_string();
        }
        self.redactor.redact(output)
    }

    pub fn watermark_output(&self, output: &str) -> anyhow::Result<String> {
        if !self.config.enable_watermarking {
            return Ok(output.to_string());
        }
        self.watermarker.watermark(output)
    }

    pub async fn process_output(&self, output: &str) -> anyhow::Result<String> {
        let safety_check = self.check_output(output)?;
        if !safety_check.allowed {
            return Ok("I cannot complete this request as it may violate safety guidelines.".to_string());
        }

        let redacted = self.redact_output(output);
        let watermarked = self.watermark_output(&redacted)?;
        Ok(watermarked)
    }
}

#[derive(Debug, Clone)]
pub struct SafetyCheck {
    pub allowed: bool,
    pub risk: f32,
}
