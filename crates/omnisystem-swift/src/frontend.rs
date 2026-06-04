use bonsai_language_frontend::LanguageFrontend;
use bonsai_lair::*;
use std::path::Path;
use anyhow::Result;
use async_trait::async_trait;

pub struct SwiftFrontend;

impl SwiftFrontend {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl LanguageFrontend for SwiftFrontend {
    fn language_name(&self) -> &str { "Swift" }
    fn file_extensions(&self) -> &[&str] { &["swift"] }

    async fn parse(&self, _source: &str, _path: &Path) -> Result<LairModule> {
        Ok(LairModule {
            name: "swift_module".into(),
            functions: vec![],
            types: vec![],
            constants: vec![],
            metadata: bonsai_lair::ModuleMetadata {
                imports: vec![],
                exports: vec![],
                source_language: Some("Swift".into()),
            },
        })
    }
}
