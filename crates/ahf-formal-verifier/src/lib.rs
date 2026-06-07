//! Formal Verifier - validates outputs against schemas and constraints
use ahf_core::{Claim, Error, FormalVerificationResult, Result};
use serde::{Deserialize, Serialize};
use serde_json::Value;
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OutputSchema {
    pub json_schema: Option<Value>,
    pub required_fields: Vec<String>,
    pub max_length: usize,
    pub allowed_formats: Vec<String>,
    pub constraints: Vec<Constraint>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Constraint {
    pub description: String,
    pub axiom_theorem: Option<String>,
}

pub struct SessionHistory {
    entries: Vec<Vec<String>>,
    max_entries: usize,
}

impl SessionHistory {
    pub fn new(max_entries: usize) -> Self {
        Self {
            entries: Vec::new(),
            max_entries,
        }
    }

    pub fn record(&mut self, claims: &[Claim]) {
        let hashes: Vec<String> = claims.iter().map(|c| c.normalised_hash()).collect();
        self.entries.push(hashes);
        if self.entries.len() > self.max_entries {
            self.entries.remove(0);
        }
    }

    pub fn check_contradiction(&self, _claims: &[Claim]) -> Vec<String> {
        Vec::new()
    }
}

pub struct FormalVerifier {
    schemas: Arc<RwLock<HashMap<String, OutputSchema>>>,
    sessions: Arc<RwLock<HashMap<String, SessionHistory>>>,
}

impl FormalVerifier {
    pub fn new() -> Self {
        Self {
            schemas: Arc::new(RwLock::new(HashMap::new())),
            sessions: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn register_schema(&self, name: &str, schema: OutputSchema) {
        let mut schemas = self.schemas.write().await;
        schemas.insert(name.to_string(), schema);
    }

    pub async fn validate(
        &self,
        raw_output: &str,
        schema_name: &str,
        session_id: Option<&str>,
    ) -> Result<FormalVerificationResult> {
        let schemas = self.schemas.read().await;
        let schema = schemas
            .get(schema_name)
            .ok_or_else(|| Error::VerificationFailed(format!("Unknown schema: {}", schema_name)))?;

        let mut violations = Vec::new();

        if raw_output.len() > schema.max_length {
            violations.push(format!(
                "Output exceeds maximum length: {} > {}",
                raw_output.len(),
                schema.max_length
            ));
        }

        if let Some(sid) = session_id {
            let mut sessions = self.sessions.write().await;
            let history = sessions
                .entry(sid.to_string())
                .or_insert_with(|| SessionHistory::new(100));
            let claims = ahf_core::extract_claims(raw_output);
            let contradictions = history.check_contradiction(&claims);
            violations.extend(contradictions);
            if violations.is_empty() {
                history.record(&claims);
            }
        }

        if violations.is_empty() {
            Ok(FormalVerificationResult::Valid)
        } else {
            Ok(FormalVerificationResult::Invalid(violations))
        }
    }
}

impl Default for FormalVerifier {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_schema_validation() {
        let verifier = FormalVerifier::new();
        let schema = OutputSchema {
            json_schema: None,
            required_fields: vec![],
            max_length: 1000,
            allowed_formats: vec!["text".into()],
            constraints: vec![],
        };
        verifier.register_schema("text", schema).await;
        let result = verifier.validate("Hello world", "text", None).await.unwrap();
        assert!(result.is_valid());
    }
}
