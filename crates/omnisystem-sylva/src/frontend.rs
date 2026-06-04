//! Sylva Language Frontend

use async_trait::async_trait;
use bonsai_language_frontend::{LanguageFrontend, Result, FrontendError};
use bonsai_lair::LairModule;
use std::path::Path;

/// Sylva language frontend
#[derive(Clone)]
pub struct SylvaFrontend;

impl SylvaFrontend {
    pub fn new() -> Self {
        Self
    }
}

impl Default for SylvaFrontend {
    fn default() -> Self {
        Self::new()
    }
}

#[async_trait]
impl LanguageFrontend for SylvaFrontend {
    fn language_name(&self) -> &str {
        "Sylva"
    }

    fn file_extensions(&self) -> &[&str] {
        &["sylva", "sv"]
    }

    async fn parse(&self, source: &str, file_path: &Path) -> Result<LairModule> {
        tracing::info!("Parsing Sylva source file: {:?}", file_path);

        // Parse Sylva source into AST
        let ast = crate::parser::parse_sylva(source)
            .map_err(|e| FrontendError::ParseError(format!("{:?}", e)))?;

        // Compile AST to bytecode
        let _bytecode = crate::compiler::compile_ast(&ast)
            .map_err(|e| FrontendError::LoweringError(e.to_string()))?;

        // Create a LAIR module from the compiled bytecode
        // For now, return a placeholder module
        let lair_module = LairModule {
            name: file_path.file_stem()
                .and_then(|n| n.to_str())
                .unwrap_or("unknown")
                .to_string(),
            ..Default::default()
        };

        Ok(lair_module)
    }

    async fn format(&self, source: &str) -> Result<String> {
        // For now, return source unchanged
        // TODO: Implement Sylva code formatter
        Ok(source.to_string())
    }

    async fn lint(&self, _source: &str) -> Result<Vec<bonsai_language_frontend::Diagnostic>> {
        // TODO: Implement Sylva linter
        Ok(Vec::new())
    }
}
