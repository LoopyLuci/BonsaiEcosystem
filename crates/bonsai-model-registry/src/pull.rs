use crate::{ModelRegistry, ModelInfo, registry::ModelRegistry as Registry};
use anyhow::{Result, anyhow};

pub async fn pull_model(model_name: &str, registry: &Registry) -> Result<ModelInfo> {
    // Stub: simulate downloading a model
    let parts: Vec<&str> = model_name.splitn(2, ':').collect();
    let name = parts[0];
    let version = parts.get(1).unwrap_or(&"latest");
    let info = ModelInfo {
        name: name.to_string(),
        version: version.to_string(),
        crystal_hash: "mock".into(),
        size_bytes: 1024,
        quantization: "q4_k_m".into(),
        family: "llama".into(),
        parameters_billion: 1.0,
        context_length: 2048,
        created_at: chrono::Utc::now().to_rfc3339(),
        author: "bonsai".into(),
        license: "MIT".into(),
        capabilities: vec![],
        verified: true,
        signature_public_key: "mock".into(),
    };
    registry.register(info.clone()).await?;
    Ok(info)
}

pub async fn create_from_manifest(manifest: &crate::manifest::BluebonnetManifest, registry: &Registry) -> Result<ModelInfo> {
    // In production: download base, apply quantization, build crystal image.
    let info = ModelInfo {
        name: manifest.model.clone(),
        version: manifest.version.clone(),
        crystal_hash: blake3::hash(manifest.model.as_bytes()).to_hex().to_string(),
        size_bytes: 1024,
        quantization: manifest.quantization.clone(),
        family: "llama".into(),
        parameters_billion: 1.0,
        context_length: manifest.parameters.context_window,
        created_at: chrono::Utc::now().to_rfc3339(),
        author: "bonsai".into(),
        license: "MIT".into(),
        capabilities: vec![],
        verified: true,
        signature_public_key: "mock".into(),
    };
    registry.register(info.clone()).await?;
    Ok(info)
}
