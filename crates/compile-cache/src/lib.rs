use buir::FunctionHash;
use dashmap::DashMap;
use std::sync::Arc;

pub struct CompilationCache {
    memory_cache: Arc<DashMap<FunctionHash, String>>,
    echo_enabled: bool,
}

impl CompilationCache {
    pub fn new(echo_enabled: bool) -> Self {
        Self {
            memory_cache: Arc::new(DashMap::new()),
            echo_enabled,
        }
    }

    pub async fn get(&self, hash: &FunctionHash) -> Option<bco::BcoFile> {
        let hash_str = hex::encode(hash.0);
        if let Some(cas_hash) = self.memory_cache.get(hash) {
            if let Ok(bco) = bco::BcoFile::load(&cas_hash).await {
                return Some(bco);
            }
        }
        if let Ok(bco) = bco::BcoFile::load(&hash_str).await {
            self.memory_cache.insert(*hash, hash_str);
            return Some(bco);
        }
        if self.echo_enabled {
            if let Some(bco) = self.query_echo(hash).await {
                self.memory_cache.insert(*hash, hash_str);
                return Some(bco);
            }
        }
        None
    }

    pub async fn put(&self, bco: &bco::BcoFile) -> anyhow::Result<()> {
        let cas_hash = bco.store().await?;
        self.memory_cache.insert(bco.function_hash, cas_hash);
        Ok(())
    }

    async fn query_echo(&self, _hash: &FunctionHash) -> Option<bco::BcoFile> {
        None
    }

    pub fn stats(&self) -> CacheStats {
        CacheStats {
            memory_entries: self.memory_cache.len(),
            echo_enabled: self.echo_enabled,
        }
    }
}

pub struct CacheStats {
    pub memory_entries: usize,
    pub echo_enabled: bool,
}
