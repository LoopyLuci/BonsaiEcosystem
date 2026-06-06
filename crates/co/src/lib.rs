use buir::{BuirFunction, FunctionHash};
use serde::{Serialize, Deserialize};
use anyhow::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BcoFile {
    pub function_hash: FunctionHash,
    pub function_name: String,
    pub language: String,
    pub buir_function: BuirFunction,
    pub object_code: Vec<u8>,
    pub compiler_version: String,
    pub compilation_flags: Vec<String>,
    pub dependencies: Vec<FunctionHash>,
}

impl BcoFile {
    pub fn new(
        buir_function: BuirFunction,
        object_code: Vec<u8>,
        compiler_version: &str,
        flags: &[String],
        dependencies: &[FunctionHash],
    ) -> Self {
        let function_hash = buir::hash_function(&buir_function);
        let language = match buir_function.language {
            buir::Language::Rust => "rust".to_string(),
            buir::Language::Python => "python".to_string(),
            buir::Language::C => "c".to_string(),
            buir::Language::Cpp => "cpp".to_string(),
            buir::Language::Go => "go".to_string(),
            buir::Language::Zig => "zig".to_string(),
            buir::Language::Java => "java".to_string(),
            buir::Language::Kotlin => "kotlin".to_string(),
            buir::Language::CSharp => "csharp".to_string(),
            buir::Language::JavaScript => "javascript".to_string(),
            buir::Language::TypeScript => "typescript".to_string(),
            buir::Language::Lua => "lua".to_string(),
            buir::Language::Titan => "titan".to_string(),
            buir::Language::Aether => "aether".to_string(),
            buir::Language::Sylva => "sylva".to_string(),
            buir::Language::Axiom => "axiom".to_string(),
        };
        Self {
            function_hash,
            function_name: buir_function.name.clone(),
            language,
            buir_function,
            object_code,
            compiler_version: compiler_version.to_string(),
            compilation_flags: flags.to_vec(),
            dependencies: dependencies.to_vec(),
        }
    }

    pub async fn store(&self) -> Result<String> {
        let data = serde_json::to_vec(self)?;
        let hash_hex = hex::encode(self.function_hash.0);
        std::fs::create_dir_all("target/cas")?;
        std::fs::write(format!("target/cas/{}", &hash_hex), &data)?;
        Ok(hash_hex)
    }

    pub async fn load(hash: &str) -> Result<Self> {
        let data = std::fs::read(format!("target/cas/{}", hash))?;
        let bco: BcoFile = serde_json::from_slice(&data)?;
        Ok(bco)
    }

    pub fn verify(&self) -> bool {
        let computed = buir::hash_function(&self.buir_function);
        computed == self.function_hash
    }
}
