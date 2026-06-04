use async_trait::async_trait;
use bonsai_language_frontend::LanguageFrontend;
use bonsai_lair::{LairModule, ModuleMetadata};
use std::path::Path;

#[derive(Clone)]
pub struct AsmFrontend;

impl AsmFrontend {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl LanguageFrontend for AsmFrontend {
    fn language_name(&self) -> &str { "Assembly" }
    fn file_extensions(&self) -> &[&str] { &["asm"] }
    
    async fn parse(&self, _source: &str, file_path: &Path) -> bonsai_language_frontend::Result<LairModule> {
        Ok(LairModule {
            name: file_path.file_stem().unwrap().to_string_lossy().to_string(),
            functions: vec![],
            types: vec![],
            constants: vec![],
            metadata: ModuleMetadata {
                imports: vec![],
                exports: vec![],
                source_language: Some("Assembly".into()),
            },
        })
    }
}
