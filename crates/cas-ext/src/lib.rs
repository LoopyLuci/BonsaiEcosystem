use buir::BuirModule;
use anyhow::Result;
use std::path::{Path, PathBuf};
use tokio::fs;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct CompilationArtifact {
    pub buir_hash: String,
    pub object_code: Vec<u8>,
    pub bytecode: Option<Vec<u8>>,
    pub metadata: ArtifactMetadata,
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct ArtifactMetadata {
    pub language: buir::Language,
    pub compiler_version: String,
    pub optimisation_level: u8,
    pub target_triple: String,
    pub dependencies: Vec<String>,
    pub timestamp: u64,
}

pub struct CompilationCache {
    base_path: PathBuf,
}

impl CompilationCache {
    pub async fn new(path: impl AsRef<Path>) -> Result<Self> {
        let base_path = path.as_ref().to_path_buf();
        fs::create_dir_all(&base_path).await?;
        Ok(Self { base_path })
    }

    /// Compute the cache key for a given BUIR module and compilation context.
    pub fn compute_key(buir: &BuirModule, opt_level: u8, target: &str) -> String {
        let buir_hash = buir::hash_buir(buir);
        let context = format!("{}|{}|{}", buir_hash, opt_level, target);
        blake3::hash(context.as_bytes()).to_hex().to_string()
    }

    pub async fn put(&self, key: &str, artifact: &CompilationArtifact) -> Result<()> {
        let data = serde_json::to_vec(artifact)?;
        let path = self.base_path.join(key);
        fs::write(&path, &data).await?;
        Ok(())
    }

    pub async fn get(&self, key: &str) -> Result<Option<CompilationArtifact>> {
        let path = self.base_path.join(key);
        match fs::read(&path).await {
            Ok(data) => {
                let artifact = serde_json::from_slice(&data)?;
                Ok(Some(artifact))
            }
            Err(e) if e.kind() == std::io::ErrorKind::NotFound => Ok(None),
            Err(e) => Err(e.into()),
        }
    }

    pub async fn broadcast_find(&self, key: &str) -> Result<Option<CompilationArtifact>> {
        // Placeholder for Echo fabric integration
        let _ = key;
        Ok(None)
    }
}
