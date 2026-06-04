use async_trait::async_trait;
use bonsai_language_frontend::{LanguageFrontend, Result};
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

    async fn parse(&self, _source: &str, _path: &Path) -> Result<LairModule> {
        // Phase 3: Type checker, effects, and LAIR lowering
        Ok(LairModule { name: "titan_module".into(), ..Default::default() })
    }
}
