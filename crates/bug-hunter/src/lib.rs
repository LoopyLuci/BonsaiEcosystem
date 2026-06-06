/// Bonsai Bug Hunter – Complete stub detection, penetration testing, and integrated intelligence system
/// Powered by Knowledge Base for continuous learning
pub mod stub_detector;
pub mod repository_scanner;
pub mod auto_fixer;
pub mod audit_report;
pub mod penetration_tester;
pub mod fuzzing_engine;
pub mod knowledge_base;
pub mod integrated_system;

pub use stub_detector::{StubDetector, StubFinding, StubType, Severity};
pub use repository_scanner::{RepositoryScanner, ScanResult};
pub use auto_fixer::AutoFixer;
pub use audit_report::AuditReport;
pub use penetration_tester::{PenetrationTester, VulnerabilityFinding, VulnerabilityType, VulnerabilitySeverity};
pub use fuzzing_engine::{FuzzingEngine, FuzzingResult};
pub use knowledge_base::{KnowledgeBase, KnowledgeEntry, IssueCategory, KnowledgeIncident};
pub use integrated_system::{IntegratedSystem, IntegratedIntelligenceReport};

use std::path::Path;

/// Complete audit and fix workflow
pub async fn audit_and_fix_repository(root_path: &Path, fix_issues: bool) -> Result<AuditReport, Box<dyn std::error::Error>> {
    // Scan repository
    let scanner = RepositoryScanner::new(root_path);
    let scan_result = scanner.scan()?;

    // Generate report
    let report = AuditReport::from_scan_result(scan_result, root_path);

    // Apply fixes if requested
    if fix_issues && !report.findings.is_empty() {
        report.apply_fixes().await?;
    }

    Ok(report)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_stub_detection() {
        let detector = StubDetector::new();
        let findings = detector.scan_line("unimplemented!()", 1, "test.rs");
        assert!(!findings.is_empty());
    }

    #[test]
    fn test_severity_ordering() {
        assert!(Severity::Critical > Severity::High);
        assert!(Severity::High > Severity::Medium);
        assert!(Severity::Medium > Severity::Low);
    }
}
