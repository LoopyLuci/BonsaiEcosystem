//! Bonsai Universal Linter (BUL) – Real-time polyglot linting for code and prose.
//!
//! BUL provides:
//! - Incremental, Salsa-based analysis with blast-radius tracking
//! - Real-time diagnostics for 26+ programming languages + Omnisystem (Titan, Aether, Sylva, Axiom)
//! - AI-augmented rule generation and false-positive filtering
//! - Spellchecking and grammar checking for 80+ human languages
//! - Deep integration with Bonsai Ecosystem (Bug Hunt, Survival, Universe, MCP)

pub mod diagnostics;
pub mod engine;
pub mod parsers;
pub mod rules;
pub mod spell;
pub mod integration;
pub mod plugin;
pub mod phase_c;
pub mod collaboration;
pub mod distribution;
pub mod prose;
pub mod plugins;
pub mod universe;

pub use diagnostics::{Diagnostic, Severity, Range, Fix};
pub use engine::{LintEngine, LintSession, LintConfig};
pub use rules::{RuleRegistry, StaticRule, NativeRule};
pub use spell::{SpellChecker, LanguageDetector};
pub use phase_c::{PhaseCOrchestrator, PhaseCConfig, PhaseCEnrichment, AxiomVerifier, PredictiveLinter, OmnisystemLinter};
pub use collaboration::TransferDaemonBridge;
pub use distribution::DistributedLintCoordinator;
pub use prose::ProseChecker;
pub use plugins::PluginMarketplace;
pub use integration::survival_feedback::SurvivalFeedbackBridge;
pub use universe::LintDashboard;

use anyhow::Result;
use std::path::PathBuf;

/// Entry point for linting a workspace or file.
pub async fn lint(config: LintConfig) -> Result<Vec<Diagnostic>> {
    let engine = LintEngine::new(config)?;
    engine.lint().await
}

/// Quick lint of a single file with default config.
pub async fn lint_file(path: PathBuf) -> Result<Vec<Diagnostic>> {
    let config = LintConfig {
        root: path.parent().unwrap_or_else(|| std::path::Path::new(".")).to_path_buf(),
        target_files: Some(vec![path]),
        ..Default::default()
    };
    lint(config).await
}
