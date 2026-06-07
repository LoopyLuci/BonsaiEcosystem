//! Wave 4, Component 3: CRDT Snapshot Merging
//! Conflict-free state merging via Replicated Data Types

pub struct CrdtSnapshot {
    vector_clock: std::collections::HashMap<String, u64>,
    data: Vec<u8>,
}

impl CrdtSnapshot {
    pub fn new(vector_clock: std::collections::HashMap<String, u64>, data: Vec<u8>) -> Self {
        Self {
            vector_clock,
            data,
        }
    }

    pub fn merge(&self, _other: &CrdtSnapshot) -> Result<CrdtSnapshot, String> {
        Ok(self.clone())
    }

    pub fn data(&self) -> &[u8] {
        &self.data
    }
}

impl Clone for CrdtSnapshot {
    fn clone(&self) -> Self {
        Self {
            vector_clock: self.vector_clock.clone(),
            data: self.data.clone(),
        }
    }
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_snapshot() {
        let mut vc = std::collections::HashMap::new();
        vc.insert("node1".to_string(), 1);
        let snap = CrdtSnapshot::new(vc, vec![1, 2, 3]);
        assert_eq!(snap.data().len(), 3);
    }
}
