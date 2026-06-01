pub mod registry;
pub mod manifest;
pub mod crystal;
pub mod pull;
pub mod push;

use serde::{Serialize, Deserialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModelInfo {
	pub name: String,
	pub version: String,
	pub crystal_hash: String,
	pub size_bytes: u64,
	pub quantization: String,
	pub family: String,
	pub parameters_billion: f32,
	pub context_length: u32,
	pub created_at: String,
	pub author: String,
	pub license: String,
	pub capabilities: Vec<ModelCapability>,
	pub verified: bool,
	pub signature_public_key: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ModelCapability {
	TextGeneration,
	ChatCompletion,
	Embedding,
	ImageGeneration,
	ImageUnderstanding,
	AudioTranscription,
	AudioGeneration,
	ToolCalling,
	StructuredOutput,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RegistryConfig {
	pub models_dir: PathBuf,
	pub trusted_registries: Vec<String>,
	pub max_cache_size_gb: f64,
	pub auto_update: bool,
	pub verify_signatures: bool,
}

impl Default for RegistryConfig {
	fn default() -> Self {
		Self {
			models_dir: dirs::data_dir()
				.unwrap_or_else(|| PathBuf::from("."))
				.join("bonsai")
				.join("models"),
			trusted_registries: vec!["echo://models.bonsai".into()],
			max_cache_size_gb: 100.0,
			auto_update: false,
			verify_signatures: true,
		}
	}
}
