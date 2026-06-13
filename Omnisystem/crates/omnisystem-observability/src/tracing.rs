use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TraceContext {
    pub trace_id: String,
    pub span_id: String,
    pub timestamp: DateTime<Utc>,
}

impl TraceContext {
    pub fn new(trace_id: String, span_id: String) -> Self {
        Self {
            trace_id,
            span_id,
            timestamp: Utc::now(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    #[test]
    fn test_trace_context() {
        let ctx = TraceContext::new("t1".to_string(), "s1".to_string());
        assert_eq!(ctx.trace_id, "t1");
    }
}
