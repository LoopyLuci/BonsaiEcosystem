//! Aether-Based Distributed Vision Pipelines
//!
//! Phase 5: Planet-scale, fault-tolerant vision processing

use async_trait::async_trait;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

/// Actor trait for vision processing
#[async_trait]
pub trait VisionActor: Send + Sync {
    async fn process(&self, input: PipelineMessage) -> Result<PipelineMessage, String>;
    fn actor_type(&self) -> &'static str;
}

/// Pipeline message format
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PipelineMessage {
    pub id: String,
    pub message_type: String,
    pub data: Vec<u8>,
    pub timestamp: u64,
}

impl PipelineMessage {
    pub fn new(message_type: String, data: Vec<u8>) -> Self {
        PipelineMessage {
            id: Uuid::new_v4().to_string(),
            message_type,
            data,
            timestamp: std::time::SystemTime::now()
                .duration_since(std::time::UNIX_EPOCH)
                .unwrap_or_default()
                .as_secs(),
        }
    }
}

/// Vision pipeline coordinator
pub struct VisionPipeline {
    _name: String,
    actors: Arc<RwLock<HashMap<String, Arc<dyn VisionActor>>>>,
    message_log: Arc<RwLock<Vec<PipelineMessage>>>,
}

impl VisionPipeline {
    pub fn new(name: String) -> Self {
        VisionPipeline {
            _name: name,
            actors: Arc::new(RwLock::new(HashMap::new())),
            message_log: Arc::new(RwLock::new(Vec::new())),
        }
    }

    pub async fn register_actor(&self, actor_id: String, actor: Arc<dyn VisionActor>) {
        let mut actors = self.actors.write().await;
        actors.insert(actor_id, actor);
    }

    pub async fn process_message(&self, actor_id: String, msg: PipelineMessage) -> Result<(), String> {
        let actors = self.actors.read().await;
        let actor = actors
            .get(&actor_id)
            .ok_or_else(|| format!("Actor {} not found", actor_id))?;

        let result = actor.process(msg).await?;

        let mut log = self.message_log.write().await;
        log.push(result);

        Ok(())
    }

    pub async fn get_actor_count(&self) -> usize {
        self.actors.read().await.len()
    }

    pub async fn get_message_count(&self) -> usize {
        self.message_log.read().await.len()
    }
}
