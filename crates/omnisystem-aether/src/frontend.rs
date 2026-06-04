use async_trait::async_trait;
use bonsai_language_frontend::{LanguageFrontend, Result};
use bonsai_lair::LairModule;
use std::path::Path;

#[derive(Clone)]
pub struct AetherFrontend;

impl AetherFrontend {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl LanguageFrontend for AetherFrontend {
    fn language_name(&self) -> &str { "Aether" }
    fn file_extensions(&self) -> &[&str] { &["aether", "ae"] }

    async fn parse(&self, _source: &str, _path: &Path) -> Result<LairModule> {
        // Phase 5: Actor parsing and LAIR lowering
        Ok(LairModule { name: "aether_module".into(), ..Default::default() })
    }
}
