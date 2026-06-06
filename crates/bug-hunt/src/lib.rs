// Bonsai Bug Hunt & Repo Sweep System
//
// A multi-stage, intelligent, and extensible platform for detecting bugs,
// errors, security vulnerabilities, performance issues, and style violations
// across entire repositories.
//
// Deep integration with Survival System, Knowledge Database, and Universe logging.

pub mod analyzer;
pub mod cache;
pub mod database;
pub mod engines;
pub mod finding;
pub mod kdb_integration;
pub mod mcp_tools;
pub mod orchestrator;
pub mod report;
pub mod self_healing;
pub mod survival_integration;
pub mod universe_integration;

pub use analyzer::LanguageAnalyzer;
pub use cache::ScanCache;
pub use database::{init_db, insert_finding, record_scan, record_fix};
pub use finding::{Finding, FindingStatus, ScanReport, ScanSummary, Severity};
pub use kdb_integration::{enrich_finding_with_kdb, store_new_pattern};
pub use orchestrator::BugHuntOrchestrator;
pub use report::ReportGenerator;
pub use survival_integration::{scan_on_crash, record_fix_in_survival_kb, SurvivalScanRequest, SurvivalScanResponse};
pub use universe_integration::{log_sweep_completed, log_finding_created, log_fix_applied};
