use async_trait::async_trait;
use bonsai_language_frontend::LanguageFrontend;
use bonsai_lair::{LairModule, ModuleMetadata};
use std::path::Path;

#[derive(Clone)]
pub struct OcamlFrontend;

impl OcamlFrontend {
    pub fn new() -> Self { Self }
}

#[async_trait]
impl LanguageFrontend for OcamlFrontend {
    fn language_name(&self) -> &str { "OCaml" }
    fn file_extensions(&self) -> &[&str] { &["ocaml"] }
    
    async fn parse(&self, _source: &str, file_path: &Path) -> bonsai_language_frontend::Result<LairModule> {
        Ok(LairModule {
            name: file_path.file_stem().unwrap().to_string_lossy().to_string(),
            functions: vec![],
            types: vec![],
            constants: vec![],
            metadata: ModuleMetadata {
                imports: vec![],
                exports: vec![],
                source_language: Some("OCaml".into()),
            },
        })
    }
}
