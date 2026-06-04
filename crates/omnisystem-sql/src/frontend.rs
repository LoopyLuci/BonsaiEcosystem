use async_trait::async_trait;
use bonsai_language_frontend::LanguageFrontend;
use bonsai_lair::{LairModule, ModuleMetadata};
use std::path::Path;

#[derive(Clone)]
pub struct SqlFrontend;

impl SqlFrontend {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl LanguageFrontend for SqlFrontend {
    fn language_name(&self) -> &str { "SQL" }
    fn file_extensions(&self) -> &[&str] { &["sql"] }
    
    async fn parse(&self, _source: &str, _path: &Path) -> bonsai_language_frontend::Result<LairModule> {
        Ok(LairModule {
            name: "sql_module".into(),
            functions: vec![],
            types: vec![],
            constants: vec![],
            metadata: ModuleMetadata {
                imports: vec![],
                exports: vec![],
                source_language: Some("SQL".into()),
            },
        })
    }
}
