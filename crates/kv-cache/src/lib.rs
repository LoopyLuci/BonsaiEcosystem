use serde::{Serialize, Deserialize};
use sled::Db;
use anyhow::Result;

pub struct KvCache {
    db: Db,
}

impl KvCache {
    pub fn open(path: &str) -> Result<Self> {
        let db = sled::open(path)?;
        Ok(Self { db })
    }

    pub fn get(&self, key: &str) -> Result<Option<Vec<u8>>> {
        Ok(self.db.get(key)?.map(|v| v.to_vec()))
    }

    pub fn set(&self, key: &str, value: &[u8]) -> Result<()> {
        self.db.insert(key, value)?;
        Ok(())
    }
}
use std::collections::HashMap;
use tokio::sync::Mutex;

pub struct KvCache { inner: Mutex<HashMap<String,String>> }

impl KvCache {
    pub fn new() -> Self { Self { inner: Mutex::new(HashMap::new()) } }
}

#[cfg(test)]
mod tests { use super::*; #[tokio::test] async fn smoke() { let _ = KvCache::new(); } }
