//! SRWSTS Chaos Engineering - Comprehensive Fault Injection and Chaos Scenario System
//!
//! A production-grade chaos engineering framework for systematic testing of system resilience.
//!
//! # Features
//!
//! ## 1. Advanced Fault Types
//! - **Latent Faults**: Injected but manifests after delay (time-bomb style)
//! - **Cascading Faults**: One fault triggers others
//! - **Transient Faults**: Appear and disappear on schedule
//! - **Byzantine Faults**: Component gives inconsistent responses
//! - **Silent Faults**: Failures with no error indication
//!
//! ## 2. Fault Schedule Generation
//! - **Deterministic**: Seed → reproducible fault schedule
//! - **Random**: Probability distribution over fault types
//! - **Clustered**: Faults grouped together for stress testing
//! - **Spread**: Faults distributed throughout test duration
//!
//! ## 3. Pre-Configured Chaos Scenarios (40+)
//! - Black Friday: 10,000x normal load with network congestion
//! - Power Grid Failure: Rolling blackouts with UPS activation
//! - Data Center Fire: Cooling failure → thermal throttling → cascade
//! - Network Meltdown: Progressive packet loss 1% → 50% → recovery
//! - Storage Corruption: Bit flips in RAID data and recovery
//! - Byzantine Leader: Consensus node inconsistency
//! - Zombie Apocalypse: 50% services become unresponsive
//! - Cascading Restart: Service crash triggers dependent failures
//! - Memory Leak Under Load: Progressive memory exhaustion
//! - Slow Query Cascade: Database queries exhaust thread pools
//! - And 30+ more real-world scenarios...
//!
//! ## 4. Deterministic Clock
//! - All faults tied to virtual clock for reproducibility
//! - Same test time → same faults at same moments
//! - Enables perfect replay and debugging
//!
//! ## 5. VirtioFaultChannel Protocol
//! - Live fault injection communication
//! - Acknowledgment of fault application
//! - Recovery confirmation
//! - Heartbeat and status monitoring
//!
//! ## 6. Recovery Validation
//! - Measure time to detect fault
//! - Measure time to recover
//! - Measure data loss (must be zero for critical operations)
//! - Measure consistency violations
//!
//! ## 7. Chaos Suite Executor
//! - Run test once normally (baseline)
//! - Run 100+ times with random faults (each seeded)
//! - Verify all runs pass
//! - Verify performance degradation is bounded
//!
//! ## 8. Weakness Prediction
//! - AI advisor analyzes failure patterns
//! - Identifies system vulnerabilities
//! - Suggests additional tests or hardening
//! - Generates prioritized recommendations
//!
//! # Quick Start
//!
//! ```no_run
//! use srwsts_chaos::scenarios;
//! use srwsts_chaos::suite_executor::{ChaosSuiteExecutor, ChaosTestConfig};
//!
//! #[tokio::main]
//! async fn main() {
//!     // Create test configuration
//!     let config = ChaosTestConfig {
//!         scenario: "Black Friday Traffic Surge".to_string(),
//!         num_runs: 50,
//!         ..Default::default()
//!     };
//!
//!     // Create executor
//!     let mut executor = ChaosSuiteExecutor::new(config);
//!
//!     // Get scenario
//!     let scenario = scenarios::scenario_black_friday().unwrap();
//!
//!     // Run full chaos suite
//!     executor.run_suite(&scenario).await.unwrap();
//!
//!     // Get results
//!     let results = executor.results();
//!     println!("{}", results.report());
//! }
//! ```

// Module declarations
pub mod advanced_faults;
pub mod deterministic_clock;
pub mod error;
pub mod recovery_validation;
pub mod scenarios;
pub mod schedule_generator;
pub mod suite_executor;
pub mod virtio_channel;
pub mod weakness_prediction;

// Re-exports for convenience
pub use advanced_faults::{
    AdvancedFaultId, ByzantineFault, CascadingEffect, CascadingFault, InconsistencyType,
    LatentFault, SilentFault, SilentFailureType, TransientFault,
};
pub use deterministic_clock::DeterministicClock;
pub use error::{ChaosError, Result};
pub use recovery_validation::{RecoveryMetrics, RecoveryStats, RecoveryValidator, ValidationResult};
pub use scenarios::{ChaosScenario, ImpactLevel, ScenarioCategory};
pub use schedule_generator::{FaultSchedule, ScheduleGenerator, ScheduleStrategy};
pub use suite_executor::{ChaosTestConfig, ChaosTestResults, ChaosSuiteExecutor};
pub use virtio_channel::{FaultChannelMessage, FaultChannelProtocol, VirtioFaultChannel};
pub use weakness_prediction::{
    EffortEstimate, PrioritizedRecommendation, RecommendationGenerator, WeaknessAnalysis,
    WeaknessPredictor,
};

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_crate_exports() {
        // Verify all public types are accessible
        let _fault_type = advanced_faults::LatentFault::new(
            "test".to_string(),
            100,
            50,
            30,
        );
        let _clock = DeterministicClock::new(1000);
        let _generator = ScheduleGenerator::new(ScheduleStrategy::Deterministic, 42, 1000, 3600);
    }
}
