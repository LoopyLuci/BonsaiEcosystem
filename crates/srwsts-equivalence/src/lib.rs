//! # SRWSTS Hardware Equivalence Validation System
//!
//! Comprehensive multi-architecture hardware equivalence validation framework.
//!
//! ## Overview
//!
//! This crate provides a complete system for validating behavior equivalence across
//! multiple hardware architectures (x86_64, ARMv8, RISC-V). It ensures that the same
//! test with the same seed produces identical results on all supported architectures,
//! while also validating performance characteristics and memory access patterns.
//!
//! ## Key Components
//!
//! ### Architecture Support
//! - **x86_64**: Skylake, EPYC, Xeon with SSE/AVX support
//! - **ARMv8**: Cortex-A72, Cortex-A76 with NEON support
//! - **RISC-V**: RV64 with vector extension support
//! - **Emulated variants**: Fallback emulation for testing on non-native architectures
//!
//! ### Test Harness
//! - **DeterministicTestHarness**: Runs tests with identical seeds across architectures
//! - **ExecutionTracer**: Captures detailed execution traces for comparison
//! - **TraceComparator**: Identifies divergence points between architectures
//!
//! ### Equivalence Validation
//! - **OutputValidator**: Exact byte-for-byte output matching
//! - **PerformanceValidator**: Latency within 10% (accounting for clock differences)
//! - **MemoryAccessValidator**: L1/L2/L3 hit/miss pattern verification
//! - **AtomicSemantics**: Acquire/release semantics verification
//! - **SIMDValidator**: Vector operation equivalence
//!
//! ### Edge Case Testing
//! - **IntegerOverflow**: Undefined vs defined behavior
//! - **FloatingPoint**: IEEE 754 compliance and rounding
//! - **Endianness**: Big-endian vs little-endian validation
//! - **UnalignedAccess**: Memory alignment handling
//! - **CacheCoherency**: Concurrent access patterns
//! - **BranchPrediction**: Timing bounds under contention
//!
//! ### Reporting
//! - **Green**: Output matches, performance within tolerance
//! - **Yellow**: Output matches, performance differs but explained
//! - **Red**: Output differs or performance exceeds tolerance
//! - **RootCauseAnalysis**: Automated divergence point identification
//!
//! ## Example
//!
//! ```ignore
//! use srwsts_equivalence::*;
//!
//! #[tokio::main]
//! async fn main() -> Result<(), Box<dyn std::error::Error>> {
//!     // Create harness for x86_64 and ARMv8
//!     let harness = DeterministicTestHarness::new(vec![
//!         ArchitectureTarget::X86_64(ArchVariant::Skylake),
//!         ArchitectureTarget::ARMv8(ArchVariant::CortexA76),
//!     ]).await?;
//!
//!     // Run test on all architectures
//!     let results = harness.run_test_all_architectures(
//!         "my_test",
//!         42,  // deterministic seed
//!     ).await?;
//!
//!     // Validate equivalence
//!     let report = harness.validate_equivalence(&results).await?;
//!     println!("{}", report);
//!
//!     Ok(())
//! }
//! ```

pub mod architecture;
pub mod edge_cases;
pub mod equivalence;
pub mod execution;
pub mod feature_validation;
pub mod memory;
pub mod monitoring;
pub mod performance;
pub mod reporting;

pub use architecture::*;
pub use edge_cases::*;
pub use equivalence::*;
pub use execution::*;
pub use feature_validation::*;
pub use memory::*;
pub use monitoring::*;
pub use performance::*;
pub use reporting::*;

use async_trait::async_trait;
use std::sync::Arc;
use thiserror::Error;

/// Error type for equivalence validation operations
#[derive(Error, Debug)]
pub enum EquivalenceError {
    #[error("Architecture not supported: {0}")]
    UnsupportedArchitecture(String),

    #[error("Test execution failed: {0}")]
    ExecutionFailed(String),

    #[error("Output mismatch on architecture {arch}: expected {expected_bytes} bytes, got {actual_bytes}")]
    OutputMismatch {
        arch: String,
        expected_bytes: usize,
        actual_bytes: usize,
    },

    #[error("Performance divergence on {arch}: {reason}")]
    PerformanceDivergence { arch: String, reason: String },

    #[error("Memory pattern mismatch on {arch}: {reason}")]
    MemoryPatternMismatch { arch: String, reason: String },

    #[error("Atomic semantics violation on {arch}: {reason}")]
    AtomicSemanticsViolation { arch: String, reason: String },

    #[error("SIMD equivalence failure on {arch}: {reason}")]
    SIMDEquivalenceFailure { arch: String, reason: String },

    #[error("Cache coherency violation: {0}")]
    CacheCoherencyViolation(String),

    #[error("IEEE 754 compliance failure: {0}")]
    IEEE754ComplianceFailure(String),

    #[error("Endianness mismatch: {0}")]
    EndiannessMismatch(String),

    #[error("Unaligned access violation: {0}")]
    UnalignedAccessViolation(String),

    #[error("Feature not available on {arch}: {feature}")]
    FeatureNotAvailable { arch: String, feature: String },

    #[error("Trace divergence detected at instruction {instruction_count}: {reason}")]
    TraceDivergence {
        instruction_count: u64,
        reason: String,
    },

    #[error("Serialization error: {0}")]
    SerializationError(String),

    #[error("IO error: {0}")]
    IoError(#[from] std::io::Error),

    #[error("Timeout: {0}")]
    Timeout(String),

    #[error("Configuration error: {0}")]
    ConfigurationError(String),

    #[error("Internal error: {0}")]
    Internal(String),
}

pub type EquivalenceResult<T> = Result<T, EquivalenceError>;

/// Configuration for equivalence validation
#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EquivalenceConfig {
    /// List of architectures to test
    pub architectures: Vec<ArchitectureTarget>,

    /// Performance tolerance in percentage (default: 10%)
    pub performance_tolerance_percent: f64,

    /// Maximum latency divergence in nanoseconds
    pub max_latency_divergence_ns: u64,

    /// Enable detailed trace collection
    pub enable_trace_collection: bool,

    /// Enable memory access pattern tracking
    pub enable_memory_tracking: bool,

    /// Enable atomic operation verification
    pub enable_atomic_verification: bool,

    /// Enable SIMD validation
    pub enable_simd_validation: bool,

    /// Test timeout in seconds
    pub test_timeout_secs: u64,

    /// Enable caching of results
    pub enable_result_caching: bool,

    /// Root cause analysis depth
    pub rca_depth: u32,
}

impl Default for EquivalenceConfig {
    fn default() -> Self {
        Self {
            architectures: vec![
                ArchitectureTarget::X86_64(ArchVariant::Skylake),
                ArchitectureTarget::ARMv8(ArchVariant::CortexA76),
            ],
            performance_tolerance_percent: 10.0,
            max_latency_divergence_ns: 10_000_000, // 10ms
            enable_trace_collection: true,
            enable_memory_tracking: true,
            enable_atomic_verification: true,
            enable_simd_validation: true,
            test_timeout_secs: 300,
            enable_result_caching: true,
            rca_depth: 5,
        }
    }
}

/// Global equivalence test coordinator
pub struct EquivalenceCoordinator {
    config: Arc<EquivalenceConfig>,
    test_harness: Arc<DeterministicTestHarness>,
    monitor: Arc<EquivalenceMonitor>,
}

impl EquivalenceCoordinator {
    /// Create a new equivalence coordinator
    pub async fn new(config: EquivalenceConfig) -> EquivalenceResult<Self> {
        let harness = DeterministicTestHarness::new(config.architectures.clone()).await?;
        let monitor = EquivalenceMonitor::new();

        Ok(Self {
            config: Arc::new(config),
            test_harness: Arc::new(harness),
            monitor: Arc::new(monitor),
        })
    }

    /// Run a complete equivalence test
    pub async fn run_equivalence_test(
        &self,
        test_name: &str,
        _test_fn: impl Fn() -> Vec<u8> + Send + Sync + 'static,
        seed: u64,
    ) -> EquivalenceResult<EquivalenceReport> {
        tracing::info!(
            test_name = test_name,
            seed = seed,
            "Starting equivalence test"
        );

        // Run test on all architectures
        let results = self.test_harness.run_test_all_architectures(test_name, seed).await?;

        // Run validators
        let mut validation_results = Vec::new();
        let output_validator = OutputValidator::default();
        let output_result = output_validator.validate(&results, &self.config).await?;
        validation_results.push(output_result);

        let perf_validator = PerformanceValidator::new();
        let perf_result = perf_validator.validate(&results, &self.config).await?;
        validation_results.push(perf_result);

        let mem_validator = MemoryAccessValidator::default();
        let mem_result = mem_validator.validate(&results, &self.config).await?;
        validation_results.push(mem_result);

        // Generate report
        let report = EquivalenceReport::new(test_name, validation_results);

        // Record in monitoring
        self.monitor.record_test(&report).await;

        tracing::info!(
            test_name = test_name,
            status = ?report.status,
            "Equivalence test completed"
        );

        Ok(report)
    }

    /// Get the monitoring interface
    pub fn monitor(&self) -> Arc<EquivalenceMonitor> {
        self.monitor.clone()
    }
}

/// Trait for implementing equivalence validators
#[async_trait]
pub trait EquivalenceValidator: Send + Sync {
    /// Validate equivalence across architectures
    async fn validate(
        &self,
        results: &ArchitectureTestResults,
        config: &EquivalenceConfig,
    ) -> EquivalenceResult<ValidationResult>;

    /// Get the name of this validator
    fn name(&self) -> &str;
}
