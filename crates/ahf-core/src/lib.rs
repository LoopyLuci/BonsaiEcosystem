//! Anti-Hallucination Framework - Core Types
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Error {
    GroundingFailed(String),
    VerificationFailed(String),
    BiasDetected(String),
    ConfidenceTooLow(f32, f32),
    EmptyOutput,
    MissingCapability(String),
    Timeout(String),
    Contradiction(String),
    IoError(String),
    SerializationError(String),
}

impl fmt::Display for Error {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Error::GroundingFailed(msg) => write!(f, "Grounding failed: {}", msg),
            Error::VerificationFailed(msg) => write!(f, "Verification failed: {}", msg),
            Error::BiasDetected(msg) => write!(f, "Bias detected: {}", msg),
            Error::ConfidenceTooLow(got, need) => {
                write!(f, "Confidence too low: {:.2} < {:.2}", got, need)
            }
            Error::EmptyOutput => write!(f, "Empty output"),
            Error::MissingCapability(cap) => write!(f, "Missing capability: {}", cap),
            Error::Timeout(svc) => write!(f, "Timeout: {}", svc),
            Error::Contradiction(msg) => write!(f, "Contradiction: {}", msg),
            Error::IoError(msg) => write!(f, "I/O error: {}", msg),
            Error::SerializationError(msg) => write!(f, "Serialization error: {}", msg),
        }
    }
}

impl std::error::Error for Error {}

pub type Result<T> = std::result::Result<T, Error>;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum KnowledgeSource {
    Cas(String),
    Ums(String),
    SystemState(String),
    External(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub struct Claim {
    pub subject: String,
    pub predicate: String,
    pub object: String,
    pub qualifiers: Vec<String>,
}

impl Claim {
    pub fn new(subject: &str, predicate: &str, object: &str) -> Self {
        Self {
            subject: subject.to_string(),
            predicate: predicate.to_string(),
            object: object.to_string(),
            qualifiers: vec![],
        }
    }

    pub fn normalised_hash(&self) -> String {
        format!("{}|{}|{}", self.subject, self.predicate, self.object)
    }

    pub fn is_empty(&self) -> bool {
        self.subject.is_empty() && self.predicate.is_empty() && self.object.is_empty()
    }
}

impl fmt::Display for Claim {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "({}, {}, {})", self.subject, self.predicate, self.object)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct GroundingEvidence {
    pub source: KnowledgeSource,
    pub statement_hash: String,
    pub confidence: f32,
    pub metadata: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum GroundingScore {
    Found(GroundingEvidence),
    NotFound,
    Contradicted(GroundingEvidence),
}

impl GroundingScore {
    pub fn is_grounded(&self) -> bool {
        matches!(self, GroundingScore::Found(_))
    }

    pub fn confidence(&self) -> f32 {
        match self {
            GroundingScore::Found(ev) => ev.confidence,
            GroundingScore::Contradicted(ev) => ev.confidence,
            GroundingScore::NotFound => 0.0,
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum FormalVerificationResult {
    Valid,
    Invalid(Vec<String>),
}

impl FormalVerificationResult {
    pub fn is_valid(&self) -> bool {
        matches!(self, FormalVerificationResult::Valid)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasReport {
    pub bias_score: f32,
    pub matched_patterns: Vec<String>,
    pub ml_flagged: bool,
    pub explanation: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ArbiterDecision {
    pub accepted: bool,
    pub reason: String,
    pub safety_envelope: SafetyEnvelope,
    pub fallback: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SafetyEnvelope {
    pub phrase_replacements: Vec<(String, String)>,
    pub blocked_phrases: Vec<String>,
    pub max_certainty_level: CertaintyLevel,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CertaintyLevel {
    Absolute,
    High,
    Moderate,
    Low,
}

impl Default for SafetyEnvelope {
    fn default() -> Self {
        Self {
            phrase_replacements: vec![
                ("I am certain that".to_string(), "Based on verified sources,".to_string()),
                ("definitely".to_string(), "with high confidence".to_string()),
            ],
            blocked_phrases: vec!["I guarantee".to_string(), "trust me".to_string()],
            max_certainty_level: CertaintyLevel::High,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputResult {
    pub accepted: bool,
    pub final_output: String,
    pub grounding_score: f64,
    pub verification_result: FormalVerificationResult,
    pub bias_detected: bool,
    pub confidence: f32,
    pub reason: String,
    pub fallback_used: bool,
}

impl OutputResult {
    pub fn reject(reason: &str, fallback: &str) -> Self {
        Self {
            accepted: false,
            final_output: fallback.to_string(),
            grounding_score: 0.0,
            verification_result: FormalVerificationResult::Valid,
            bias_detected: false,
            confidence: 0.0,
            reason: reason.to_string(),
            fallback_used: true,
        }
    }
}

pub fn extract_claims(text: &str) -> Vec<Claim> {
    let mut claims = Vec::new();
    for sentence in text.split(|c| c == '.' || c == '!' || c == '?') {
        let sentence = sentence.trim();
        if sentence.is_empty() {
            continue;
        }
        if let Some(pos) = sentence.to_lowercase().find(" is ") {
            let subject = sentence[..pos].trim();
            let object = sentence[pos + 4..].trim();
            if !subject.is_empty() && !object.is_empty() {
                claims.push(Claim::new(subject, "is", object));
            }
        }
    }
    claims
}

pub fn apply_safety_envelope(text: &str, envelope: &SafetyEnvelope) -> String {
    let mut result = text.to_string();
    for blocked in &envelope.blocked_phrases {
        if result.to_lowercase().contains(&blocked.to_lowercase()) {
            result = result.replace(blocked, "[statement requires verification]");
        }
    }
    for (original, replacement) in &envelope.phrase_replacements {
        result = result.replace(original, replacement);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_extract_claims() {
        let claims = extract_claims("Paris is the capital of France.");
        assert!(!claims.is_empty());
    }
}
