//! OpenCV 5 UMS Module Manifest Definitions
//!
//! Phase 1: Universal Module System integration

use serde::{Deserialize, Serialize};

/// Complete OpenCV 5 module manifest for UMS distribution
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleManifest {
    pub api_version: String,
    pub kind: String,
    pub metadata: ManifestMetadata,
    pub spec: ModuleSpec,
    pub signature: Option<SignatureInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ManifestMetadata {
    pub name: String,
    pub version: String,
    pub namespace: String,
    pub timestamp: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleSpec {
    pub targets: Vec<Target>,
    pub exports: Vec<ExportedSymbol>,
    pub capabilities: CapabilityRequirements,
    pub artifacts: Vec<Artifact>,
    pub dependencies: Vec<ModuleDependency>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Target {
    pub arch: String,
    pub os: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ExportedSymbol {
    pub name: String,
    pub symbol: String,
    pub signature: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CapabilityRequirements {
    pub required: Vec<String>,
    pub optional: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Artifact {
    pub name: String,
    pub hash: String,
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ModuleDependency {
    pub module: String,
    pub version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SignatureInfo {
    pub algorithm: String,
    pub signers: Vec<String>,
    pub threshold: usize,
}

impl ModuleManifest {
    pub fn new(name: String, version: String) -> Self {
        ModuleManifest {
            api_version: "v1".to_string(),
            kind: "Module".to_string(),
            metadata: ManifestMetadata {
                name: name.clone(),
                version: version.clone(),
                namespace: "bonsai.omnisystem".to_string(),
                timestamp: chrono::Utc::now().to_rfc3339(),
            },
            spec: ModuleSpec {
                targets: vec![],
                exports: vec![],
                capabilities: CapabilityRequirements {
                    required: vec![],
                    optional: vec![],
                },
                artifacts: vec![],
                dependencies: vec![],
            },
            signature: None,
        }
    }

    pub fn validate(&self) -> Result<(), String> {
        if self.spec.targets.is_empty() {
            return Err("No targets specified".to_string());
        }
        Ok(())
    }
}
