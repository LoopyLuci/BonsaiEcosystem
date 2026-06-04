use async_trait::async_trait;
use bonsai_language_frontend::LanguageFrontend;
use bonsai_lair::{LairModule, ModuleMetadata};
use std::path::Path;

#[derive(Clone)]
pub struct JavaFrontend;

impl JavaFrontend {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl LanguageFrontend for JavaFrontend {
    fn language_name(&self) -> &str { "Java" }
    fn file_extensions(&self) -> &[&str] { &["java"] }
    
    async fn parse(&self, _source: &str, _path: &Path) -> bonsai_language_frontend::Result<LairModule> {
        Ok(LairModule {
            name: "java_module".into(),
            functions: vec![],
            types: vec![],
            constants: vec![],
            metadata: ModuleMetadata {
                imports: vec![],
                exports: vec![],
                source_language: Some("Java".into()),
            },
        })
    }
}
