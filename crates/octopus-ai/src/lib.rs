//! Octopus AI - Multi-language specialized model system
//!
//! Provides multi-language support with optimized inference for 750+ languages

use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Language {
    pub code: String,
    pub name: String,
    pub tokenizer: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OctopusConfig {
    pub languages: Vec<Language>,
    pub base_model: String,
    pub quantization: String,
}

pub struct OctopusAI {
    config: OctopusConfig,
    language_models: Arc<RwLock<HashMap<String, String>>>,
}

impl OctopusAI {
    pub fn new(config: OctopusConfig) -> Self {
        Self {
            config,
            language_models: Arc::new(RwLock::new(HashMap::new())),
        }
    }

    pub async fn initialize(&self) -> anyhow::Result<()> {
        let mut models = self.language_models.write().await;

        for lang in &self.config.languages {
            models.insert(
                lang.code.clone(),
                format!("{}-{}", self.config.base_model, lang.code),
            );
        }

        Ok(())
    }

    pub async fn get_language_model(&self, lang_code: &str) -> Option<String> {
        self.language_models.read().await.get(lang_code).cloned()
    }

    pub async fn infer(
        &self,
        text: &str,
        language: &str,
    ) -> anyhow::Result<String> {
        let models = self.language_models.read().await;

        if !models.contains_key(language) {
            anyhow::bail!("Language not supported: {}", language);
        }

        // Simulate inference
        Ok(format!("Response [{}]: {}", language, text))
    }

    pub fn supported_languages(&self) -> Vec<String> {
        self.config
            .languages
            .iter()
            .map(|l| l.name.clone())
            .collect()
    }

    pub fn language_count(&self) -> usize {
        self.config.languages.len()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_octopus_ai_initialization() {
        let config = OctopusConfig {
            languages: vec![
                Language {
                    code: "en".to_string(),
                    name: "English".to_string(),
                    tokenizer: "gpt2".to_string(),
                },
                Language {
                    code: "es".to_string(),
                    name: "Spanish".to_string(),
                    tokenizer: "gpt2".to_string(),
                },
            ],
            base_model: "octopus-1b".to_string(),
            quantization: "q4_k_m".to_string(),
        };

        let ai = OctopusAI::new(config);
        ai.initialize().await.unwrap();

        let lang_count = ai.language_count();
        assert_eq!(lang_count, 2);

        let result = ai.infer("Hello", "en").await;
        assert!(result.is_ok());
    }
}