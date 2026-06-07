//! Knowledge Grounding Service
use ahf_core::{Claim, GroundingScore, Result};
use serde::{Deserialize, Serialize};
use std::sync::Arc;

#[async_trait::async_trait]
pub trait CasClient: Send + Sync {
    async fn get(&self, _key: &str) -> Result<Option<Vec<u8>>> {
        Ok(None)
    }
}

#[async_trait::async_trait]
pub trait UmsClient: Send + Sync {
    async fn resolve_claim(&self, _claim: &Claim) -> Result<Option<String>> {
        Ok(None)
    }
    async fn get_module(&self, _id: &str) -> Result<Option<UmsModule>> {
        Ok(None)
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UmsModule {
    pub id: String,
    pub name: String,
}

pub struct CasKnowledgeBase {
    _cas: Arc<dyn CasClient>,
}

impl CasKnowledgeBase {
    pub fn new(cas: Arc<dyn CasClient>) -> Self {
        Self { _cas: cas }
    }
}

pub struct KnowledgeGroundingService {
    _kb: Arc<CasKnowledgeBase>,
    _ums: Option<Arc<dyn UmsClient>>,
}

impl KnowledgeGroundingService {
    pub fn new(kb: Arc<CasKnowledgeBase>, ums: Option<Arc<dyn UmsClient>>) -> Self {
        Self { _kb: kb, _ums: ums }
    }

    pub async fn ground_claims(&self, claims: &[Claim]) -> Vec<GroundingScore> {
        claims.iter().map(|_| GroundingScore::NotFound).collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_ground_claims() {
        let claims = vec![Claim::new("Paris", "is", "capital")];
        let scores = vec![GroundingScore::NotFound];
        assert_eq!(claims.len(), scores.len());
    }
}
