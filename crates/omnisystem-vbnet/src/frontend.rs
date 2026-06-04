use bonsai_language_frontend::LanguageFrontend;
use bonsai_lair::*;
use std::path::Path;
use anyhow::Result;
use async_trait::async_trait;

pub struct VB.NETFrontend;

impl VB.NETFrontend {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl LanguageFrontend for VB.NETFrontend {
    fn language_name(&self) -> &str { "VB.NET" }
    fn file_extensions(&self) -> &[&str] { &["vb"] }

    async fn parse(&self, _source: &str, _path: &Path) -> Result<LairModule> {
        Ok(LairModule {
            name: "vb.net_module".into(),
            functions: vec![],
            types: vec![],
            constants: vec![],
            metadata: bonsai_lair::ModuleMetadata {
                imports: vec![],
                exports: vec![],
                source_language: Some("VB.NET".into()),
            },
        })
    }
}
