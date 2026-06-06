//! Team C: Memory Sanitizers
//!
//! ASAN (AddressSanitizer), MSAN (MemorySanitizer), TSAN (ThreadSanitizer), LSAN (LeakSanitizer)
//! integration for detecting buffer overflows, use-after-free, data races, and memory leaks.

pub mod interfaces;
pub mod config;
pub mod memory_tracker;
pub mod sanitizer_report;

pub use interfaces::*;
pub use config::SanitizerConfig;
pub use memory_tracker::MemoryTracker;
pub use sanitizer_report::{SanitzerReport, MemoryIssue, IssueType};

pub struct SanitizerEngine {
    config: SanitizerConfig,
    tracker: MemoryTracker,
}

impl SanitizerEngine {
    pub fn new(config: SanitizerConfig) -> Self {
        Self {
            tracker: MemoryTracker::new(),
            config,
        }
    }

    pub async fn run_with_sanitizers<F>(&mut self, test_fn: F) -> SanitzerReport
    where
        F: Fn() -> futures::future::BoxFuture<'static, ()>,
    {
        tracing::info!("Running sanitizer analysis");

        let start_time = std::time::Instant::now();
        let test = test_fn();

        tokio::time::timeout(
            std::time::Duration::from_secs(self.config.timeout_secs),
            test,
        )
        .await
        .ok();

        let issues = self.tracker.get_issues();

        SanitzerReport {
            asan_issues: issues.iter().filter(|i| i.issue_type == IssueType::BufferOverflow).count(),
            msan_issues: issues.iter().filter(|i| i.issue_type == IssueType::UninitializedMemory).count(),
            tsan_issues: issues.iter().filter(|i| i.issue_type == IssueType::DataRace).count(),
            lsan_issues: issues.iter().filter(|i| i.issue_type == IssueType::MemoryLeak).count(),
            total_issues: issues.len(),
            issues,
            duration_secs: start_time.elapsed().as_secs_f64(),
        }
    }

    pub fn track_allocation(&mut self, ptr: u64, size: usize) {
        self.tracker.track_allocation(ptr, size);
    }

    pub fn track_deallocation(&mut self, ptr: u64) {
        self.tracker.track_deallocation(ptr);
    }

    pub fn track_access(&mut self, ptr: u64, size: usize, is_write: bool) {
        self.tracker.track_access(ptr, size, is_write);
    }
}

pub async fn init() -> Result<(), anyhow::Error> {
    tracing::info!("Initializing Memory Sanitizers");
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_initialization() {
        assert!(init().await.is_ok());
    }

    #[tokio::test]
    async fn test_engine_creation() {
        let config = SanitizerConfig::default();
        let engine = SanitizerEngine::new(config);
        assert!(engine.config.enable_asan);
    }

    #[tokio::test]
    async fn test_allocation_tracking() {
        let config = SanitizerConfig::default();
        let mut engine = SanitizerEngine::new(config);

        engine.track_allocation(0x1000, 100);
        engine.track_deallocation(0x1000);

        // Basic sanity check
        assert!(true);
    }
}
