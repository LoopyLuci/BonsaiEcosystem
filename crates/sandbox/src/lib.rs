//! Bonsai Enclave - Universal Deterministic Environment & Dependency Manager
//!
//! A next-generation replacement for venv, pip, npm, cargo, conda, and all language-specific
//! dependency managers. Provides deterministic, reproducible, content-addressed environments
//! for any programming language.

pub mod cas;
pub mod environment;
pub mod lockfile;
pub mod manifest;
pub mod resolver;
pub mod runtime;
pub mod sandbox;
pub mod advisor;
pub mod p2p;

pub use cas::ContentAddressedStore;
pub use environment::{EnvironmentManager, Environment};
pub use lockfile::Lockfile;
pub use manifest::Manifest;
pub use resolver::DependencyResolver;
pub use runtime::{RuntimeManager, Runtime, RuntimeDownloader, RuntimeManifest, RuntimeEntry, PluginRegistry, create_builtin_registry};
pub use advisor::{RuntimeAdvisor, RuntimeRecommendation, PerformanceMetrics};
pub use p2p::{P2PDistributor, PeerInfo, MeshStats};

use anyhow::Result;
use std::path::PathBuf;

/// Main Enclave configuration
#[derive(Debug, Clone)]
pub struct EnclaveConfig {
    /// Root directory for Enclave data (~/.enclave or project-local)
    pub root_dir: PathBuf,
    /// Project manifest path
    pub manifest_path: PathBuf,
    /// Lock file path
    pub lockfile_path: PathBuf,
    /// CAS directory
    pub cas_dir: PathBuf,
    /// Environments directory
    pub env_dir: PathBuf,
}

impl EnclaveConfig {
    /// Create a new Enclave config for a project
    pub fn new(project_root: PathBuf) -> Result<Self> {
        let root_dir = project_root.clone();
        let cas_dir = root_dir.join(".enclave").join("cas");
        let env_dir = root_dir.join(".enclave").join("environments");

        // Create directories synchronously
        std::fs::create_dir_all(&cas_dir)?;
        std::fs::create_dir_all(&env_dir)?;

        Ok(Self {
            root_dir,
            manifest_path: project_root.join("enclave.toml"),
            lockfile_path: project_root.join("enclave.lock"),
            cas_dir,
            env_dir,
        })
    }
}

/// Main Enclave instance - the universal dependency and environment manager
pub struct Enclave {
    pub config: EnclaveConfig,
    cas: ContentAddressedStore,
    env_manager: EnvironmentManager,
    runtime_manager: RuntimeManager,
}

impl Enclave {
    /// Initialize a new Enclave instance
    pub async fn new(config: EnclaveConfig) -> Result<Self> {
        let cas = ContentAddressedStore::new(config.cas_dir.clone()).await?;
        let env_manager = EnvironmentManager::new(config.env_dir.clone()).await?;
        let runtime_manager = RuntimeManager::new(config.cas_dir.clone()).await?;

        Ok(Self {
            config,
            cas,
            env_manager,
            runtime_manager,
        })
    }

    /// Load or create project manifest
    pub async fn load_manifest(&self) -> Result<Manifest> {
        if self.config.manifest_path.exists() {
            Manifest::load(&self.config.manifest_path).await
        } else {
            Ok(Manifest::default())
        }
    }

    /// Lock dependencies deterministically
    pub async fn lock(&mut self) -> Result<Lockfile> {
        let manifest = self.load_manifest().await?;
        let mut resolver = DependencyResolver::new();

        // Resolve all dependencies
        let locked = resolver.resolve(&manifest).await?;

        // Save lockfile
        locked.save(&self.config.lockfile_path).await?;

        Ok(locked)
    }

    /// Create or update environment
    pub async fn create_environment(&mut self, name: &str) -> Result<Environment> {
        // Load lockfile
        let lockfile = if self.config.lockfile_path.exists() {
            Lockfile::load(&self.config.lockfile_path).await?
        } else {
            self.lock().await?
        };

        // Create environment
        let env = self.env_manager.create(name, &lockfile).await?;

        Ok(env)
    }

    /// Run a command in an isolated environment
    pub async fn run(&self, env_name: &str, command: &[&str]) -> Result<()> {
        let env = self.env_manager.get(env_name).await?;
        env.run_command(command).await?;
        Ok(())
    }

    pub fn cas(&self) -> &ContentAddressedStore {
        &self.cas
    }

    pub fn env_manager(&self) -> &EnvironmentManager {
        &self.env_manager
    }

    pub fn runtime_manager(&self) -> &RuntimeManager {
        &self.runtime_manager
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_enclave_creation() {
        let tmpdir = tempfile::tempdir().unwrap();
        let config = EnclaveConfig::new(tmpdir.path().to_path_buf()).unwrap();
        let enclave = Enclave::new(config).await.unwrap();

        assert!(enclave.config.cas_dir.exists());
        assert!(enclave.config.env_dir.exists());
    }
}
