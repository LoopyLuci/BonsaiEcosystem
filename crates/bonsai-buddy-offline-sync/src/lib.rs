//! Wave 4, Component 2: Offline Sync
//! Queue and synchronize operations for offline-first agents

pub struct SyncQueue {
    operations: Vec<String>,
}

impl SyncQueue {
    pub fn new() -> Self {
        Self {
            operations: Vec::new(),
        }
    }

    pub fn enqueue(&mut self, op: String) {
        self.operations.push(op);
    }

    pub fn pending_count(&self) -> usize {
        self.operations.len()
    }

    pub fn flush(&mut self) -> Vec<String> {
        let mut result = Vec::new();
        std::mem::swap(&mut result, &mut self.operations);
        result
    }
}

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_queue() {
        let mut q = SyncQueue::new();
        assert_eq!(q.pending_count(), 0);
        q.enqueue("op1".to_string());
        assert_eq!(q.pending_count(), 1);
        let ops = q.flush();
        assert_eq!(ops.len(), 1);
        assert_eq!(q.pending_count(), 0);
    }
}
