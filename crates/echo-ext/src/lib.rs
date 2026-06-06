use anyhow::Result;
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum CompilationMessage {
    FindArtifact { key: String, request_id: String },
    ProvideArtifact { key: String, artifact: Vec<u8>, request_id: String },
}

pub struct ArtifactDiscovery {
    peers: Arc<RwLock<Vec<String>>>,
}

impl ArtifactDiscovery {
    pub async fn new() -> Result<Self> {
        Ok(Self {
            peers: Arc::new(RwLock::new(Vec::new())),
        })
    }

    pub async fn broadcast_find(&self, key: &str, request_id: &str) -> Result<()> {
        let msg = CompilationMessage::FindArtifact {
            key: key.to_string(),
            request_id: request_id.to_string(),
        };
        let _json = serde_json::to_vec(&msg)?;
        // TODO: Broadcast via Echo fabric
        Ok(())
    }

    pub async fn respond_with_artifact(&self, key: &str, artifact: Vec<u8>, request_id: &str) -> Result<()> {
        let msg = CompilationMessage::ProvideArtifact {
            key: key.to_string(),
            artifact,
            request_id: request_id.to_string(),
        };
        let _json = serde_json::to_vec(&msg)?;
        // TODO: Broadcast via Echo fabric
        Ok(())
    }

    pub async fn add_peer(&self, peer_addr: String) {
        let mut peers = self.peers.write().await;
        peers.push(peer_addr);
    }

    pub async fn get_peers(&self) -> Vec<String> {
        self.peers.read().await.clone()
    }
}
