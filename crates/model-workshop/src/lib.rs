use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::collections::HashMap;
use tokio::sync::RwLock;

pub mod library;
pub mod datasets;
pub mod designer;
pub mod builder;
pub mod editor;
pub mod converter;
pub mod monitor;

pub use library::*;
pub use datasets::*;
pub use designer::*;
pub use builder::*;
pub use editor::*;
pub use converter::*;
pub use monitor::*;

#[derive(Clone)]
pub struct AppState {
    pub modules: Arc<RwLock<HashMap<String, ModuleInfo>>>,
    pub datasets: Arc<RwLock<HashMap<String, DatasetInfo>>>,
    pub training_jobs: Arc<RwLock<Vec<TrainingJob>>>,
    pub models: Arc<RwLock<HashMap<String, ModelInfo>>>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub description: String,
    pub num_chunks: usize,
    pub domains: Vec<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DatasetInfo {
    pub id: String,
    pub name: String,
    pub num_examples: usize,
    pub domains: Vec<String>,
    pub created_at: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TrainingJob {
    pub id: String,
    pub config: String,
    pub status: String,
    pub progress: f32,
    pub current_stage: u32,
    pub started_at: String,
    pub estimated_completion: String,
    pub logs: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
    pub id: String,
    pub name: String,
    pub version: String,
    pub size_mb: u64,
    pub quantization: String,
    pub is_active: bool,
}
