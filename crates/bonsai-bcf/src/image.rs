use crate::{Result, blueprint::Blueprint};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CrystalImage {
    pub manifest: ImageManifest,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageManifest {
    pub config: ImageConfig,
    pub layers: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImageConfig {
    pub entrypoint: Vec<String>,
    pub env: Vec<String>,
}

pub struct ImageManager;

impl ImageManager {
    pub fn new() -> Self {
        Self
    }

    pub async fn build_from_blueprint(&self, _blueprint: &Blueprint) -> Result<CrystalImage> {
        Ok(CrystalImage {
            manifest: ImageManifest {
                config: ImageConfig {
                    entrypoint: vec![],
                    env: vec![],
                },
                layers: vec![],
            },
            signature: String::new(),
        })
    }
}
