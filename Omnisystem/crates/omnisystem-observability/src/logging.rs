use parking_lot::Mutex;
use std::sync::Arc;

pub struct Logger {
    logs: Arc<Mutex<Vec<String>>>,
}

impl Logger {
    pub fn new() -> Self {
        Self {
            logs: Arc::new(Mutex::new(Vec::new())),
        }
    }

    pub fn log(&self, message: String) {
        self.logs.lock().push(message);
    }

    pub fn get_logs(&self) -> Vec<String> {
        self.logs.lock().clone()
    }
}

impl Default for Logger {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_logging() {
        let logger = Logger::new();
        logger.log("test".to_string());
        assert_eq!(logger.get_logs().len(), 1);
    }
}
