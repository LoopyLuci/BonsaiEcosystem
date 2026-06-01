use crate::{ModelInfo, RegistryConfig};
use std::collections::HashMap;
use std::path::PathBuf;
use anyhow::{Result, anyhow};
use tokio::sync::RwLock;

pub struct ModelRegistry {
    config: RegistryConfig,
    models: RwLock<HashMap<String, ModelInfo>>,
    active_models: RwLock<HashMap<String, PathBuf>>,
}

impl ModelRegistry {
    pub fn new(config: RegistryConfig) -> Result<Self> {
        std::fs::create_dir_all(&config.models_dir)?;
        let registry = Self {
            config,
            models: RwLock::new(HashMap::new()),
            active_models: RwLock::new(HashMap::new()),
        };
        registry.scan_local()?;
        Ok(registry)
    }

    fn scan_local(&self) -> Result<()> {
        let models_dir = &self.config.models_dir;
        if !models_dir.exists() {
            return Ok(());
        }
        for entry in walkdir::WalkDir::new(models_dir)
            .max_depth(2)
            .into_iter()
            .filter_map(|e| e.ok())
            .filter(|e| e.file_name() == "manifest.json")
        {
            if let Ok(contents) = std::fs::read_to_string(entry.path()) {
                if let Ok(info) = serde_json::from_str::<ModelInfo>(&contents) {
                    let name = format!("{}:{}", info.name, info.version);
                    let mut models = self.models.blocking_write();
                    models.insert(name, info);
                }
            }
        }
        Ok(())
    }

    pub async fn list(&self) -> Vec<ModelInfo> {
        self.models.read().await.values().cloned().collect()
    }

    pub async fn get(&self, name: &str, version: &str) -> Option<ModelInfo> {
        let key = format!("{}:{}", name, version);
        self.models.read().await.get(&key).cloned()
    }

    pub async fn register(&self, info: ModelInfo) -> Result<()> {
        let key = format!("{}:{}", info.name, info.version);
        let manifest_path = self.config.models_dir
            .join(&info.name)
            .join(&info.version)
            .join("manifest.json");
        std::fs::create_dir_all(manifest_path.parent().unwrap())?;
        let json = serde_json::to_string_pretty(&info)?;
        std::fs::write(&manifest_path, json)?;
        self.models.write().await.insert(key, info);
        Ok(())
    }

    pub async fn load_model(&self, name: &str, version: &str) -> Result<PathBuf> {
        let key = format!("{}:{}", name, version);
        if let Some(path) = self.active_models.read().await.get(&key) {
            return Ok(path.clone());
        }
        let info = self.get(name, version).await
            .ok_or_else(|| anyhow!("Model not found: {}:{}", name, version))?;
        let crystal_path = self.config.models_dir
            .join(&info.name)
            .join(&info.version)
            .join("model.crystal");
        if !crystal_path.exists() {
            anyhow::bail!("Model image not found at {}", crystal_path.display());
        }
        if self.config.verify_signatures && !info.verified {
            anyhow::bail!("Model {}:{} has not been verified", name, version);
        }
        self.active_models.write().await.insert(key, crystal_path.clone());
        Ok(crystal_path)
    }
}
