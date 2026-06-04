use async_trait::async_trait;
use bonsai_language_frontend::LanguageFrontend;
use bonsai_lair::LairModule;
use std::path::Path;

#[derive(Clone)]
pub struct TitanFrontend;

impl TitanFrontend {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl LanguageFrontend for TitanFrontend {
    fn language_name(&self) -> &str { "Titan" }
    fn file_extensions(&self) -> &[&str] { &["titan", "ti"] }
    
    async fn parse(&self, source: &str, _path: &Path) -> bonsai_language_frontend::Result<LairModule> {
        let program = crate::parser::parse(source).map_err(|e| bonsai_language_frontend::FrontendError::ParseError(e.to_string()))?;
        let checker = crate::typeck::TypeChecker::new();
        checker.check(&program).map_err(|e| bonsai_language_frontend::FrontendError::TypeError(e.to_string()))?;
        let lair = crate::lower::lower_program(&program);
        Ok(lair)
    }
}
