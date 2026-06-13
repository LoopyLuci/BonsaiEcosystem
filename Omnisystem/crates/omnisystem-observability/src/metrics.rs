use parking_lot::Mutex;
use std::sync::Arc;
use dashmap::DashMap;

#[derive(Debug, Clone)]
pub struct Metric {
    pub name: String,
    pub value: f64,
}

pub struct MetricsCollector {
    metrics: Arc<DashMap<String, Metric>>,
}

impl MetricsCollector {
    pub fn new() -> Self {
        Self {
            metrics: Arc::new(DashMap::new()),
        }
    }

    pub fn record(&self, name: String, value: f64) {
        self.metrics.insert(name, Metric { name: name.clone(), value });
    }

    pub fn get(&self, name: &str) -> Option<f64> {
        self.metrics.get(name).map(|m| m.value)
    }

    pub fn count(&self) -> usize {
        self.metrics.len()
    }
}

impl Default for MetricsCollector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_record_metric() {
        let collector = MetricsCollector::new();
        collector.record("cpu".to_string(), 45.5);
        assert_eq!(collector.get("cpu"), Some(45.5));
    }
}
