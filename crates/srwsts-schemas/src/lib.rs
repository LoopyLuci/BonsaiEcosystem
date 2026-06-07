//! SRWSTS Schema and Parsing
//!
//! Provides YAML schema definitions, serialization/deserialization,
//! and comprehensive validation for SRWSTS test plans.
//!
//! ## Example
//!
//! ```yaml
//! version: "1.0"
//! metadata:
//!   id: "kernel-scheduler-heavy-load"
//!   name: "Kernel Scheduler Heavy Load Test"
//!   description: "Stress the kernel task scheduler with heavy concurrent load"
//!   tags:
//!     - kernel
//!     - scheduler
//!     - stress
//!
//! resource_limits:
//!   max_cpu_percent: 100
//!   max_memory_bytes: 4294967296  # 4GB
//!   max_threads: 2048
//!
//! workloads:
//!   - id: "cpu-burn"
//!     type: "cpu_stress"
//!     concurrency: 16
//!     duration_secs: 300
//!     params:
//!       cpu_cores: "8"
//!       burn_type: "math-intensive"
//!
//! faults:
//!   - id: "sched-stress"
//!     type: "task_scheduling_stress"
//!     inject_at_secs: 60
//!     duration_secs: 120
//!     params:
//!       context_switch_rate: "10000"
//! ```

pub mod parser;
pub mod schema;
pub mod validation;

pub use parser::TestPlanParser;
pub use schema::{FaultDefinitionYaml, TestPlanYaml, WorkloadYaml};
pub use validation::{SchemaValidator, ValidationError};

use srwsts_core::SrwstsResult;

/// Parse a test plan from YAML string
pub fn parse_test_plan(yaml_str: &str) -> SrwstsResult<srwsts_core::TestPlan> {
    let parser = TestPlanParser::new();
    parser.parse_yaml(yaml_str)
}

/// Parse a test plan from YAML file
pub fn parse_test_plan_file(path: &std::path::Path) -> SrwstsResult<srwsts_core::TestPlan> {
    let parser = TestPlanParser::new();
    parser.parse_file(path)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_parse_simple_test_plan() {
        let yaml = r#"
version: "1.0"
metadata:
  id: "test1"
  name: "Simple Test"
  description: "A simple test"
resource_limits:
  max_cpu_percent: 100
  max_memory_bytes: 1000000000
workloads:
  - id: "w1"
    type: "cpu_stress"
    concurrency: 4
    duration_secs: 60
"#;
        let result = parse_test_plan(yaml);
        assert!(result.is_ok());
    }
}
