//! Omnisystem Configuration Framework
//! Builder patterns for deploying complex Omnisystem scenarios

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OmnisystemConfig {
    pub system_id: String,
    pub version: String,
    pub wave1: Wave1Config,
    pub wave2: Wave2Config,
    pub wave3: Wave3Config,
    pub wave4: Wave4Config,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wave1Config {
    pub enabled: bool,
    pub buddy_agent_count: usize,
    pub max_service_instances: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wave2Config {
    pub enabled: bool,
    pub clojure_enabled: bool,
    pub wasm_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wave3Config {
    pub enabled: bool,
    pub ai_advisor_enabled: bool,
    pub latency_budget_ms: u64,
    pub memory_budget_mb: u64,
    pub shadow_mode_enabled: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Wave4Config {
    pub enabled: bool,
    pub offline_sync_enabled: bool,
    pub crdt_merging_enabled: bool,
}

impl Default for OmnisystemConfig {
    fn default() -> Self {
        Self {
            system_id: "omnisystem-default".to_string(),
            version: "1.0.0".to_string(),
            wave1: Wave1Config {
                enabled: true,
                buddy_agent_count: 1,
                max_service_instances: 10,
            },
            wave2: Wave2Config {
                enabled: true,
                clojure_enabled: true,
                wasm_enabled: false,
            },
            wave3: Wave3Config {
                enabled: true,
                ai_advisor_enabled: false,
                latency_budget_ms: 100,
                memory_budget_mb: 512,
                shadow_mode_enabled: true,
            },
            wave4: Wave4Config {
                enabled: true,
                offline_sync_enabled: true,
                crdt_merging_enabled: true,
            },
        }
    }
}

pub struct OmnisystemBuilder {
    config: OmnisystemConfig,
}

impl OmnisystemBuilder {
    pub fn new(system_id: String) -> Self {
        let mut config = OmnisystemConfig::default();
        config.system_id = system_id;
        Self { config }
    }

    pub fn with_wave1(mut self, enabled: bool, buddy_count: usize) -> Self {
        self.config.wave1.enabled = enabled;
        self.config.wave1.buddy_agent_count = buddy_count;
        self
    }

    pub fn with_wave2(mut self, enabled: bool, clojure: bool, wasm: bool) -> Self {
        self.config.wave2.enabled = enabled;
        self.config.wave2.clojure_enabled = clojure;
        self.config.wave2.wasm_enabled = wasm;
        self
    }

    pub fn with_wave3(
        mut self,
        enabled: bool,
        ai_enabled: bool,
        latency_ms: u64,
        memory_mb: u64,
        shadow_mode: bool,
    ) -> Self {
        self.config.wave3.enabled = enabled;
        self.config.wave3.ai_advisor_enabled = ai_enabled;
        self.config.wave3.latency_budget_ms = latency_ms;
        self.config.wave3.memory_budget_mb = memory_mb;
        self.config.wave3.shadow_mode_enabled = shadow_mode;
        self
    }

    pub fn with_wave4(
        mut self,
        enabled: bool,
        offline_sync: bool,
        crdt: bool,
    ) -> Self {
        self.config.wave4.enabled = enabled;
        self.config.wave4.offline_sync_enabled = offline_sync;
        self.config.wave4.crdt_merging_enabled = crdt;
        self
    }

    pub fn build(self) -> OmnisystemConfig {
        self.config
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_default_config() {
        let config = OmnisystemConfig::default();
        assert_eq!(config.system_id, "omnisystem-default");
        assert!(config.wave1.enabled);
        assert!(config.wave2.enabled);
    }

    #[test]
    fn test_builder_pattern() {
        let config = OmnisystemBuilder::new("test-system".to_string())
            .with_wave1(true, 5)
            .with_wave3(true, true, 200, 1024, true)
            .build();

        assert_eq!(config.system_id, "test-system");
        assert_eq!(config.wave1.buddy_agent_count, 5);
        assert_eq!(config.wave3.latency_budget_ms, 200);
        assert!(config.wave3.ai_advisor_enabled);
    }

    #[test]
    fn test_config_serialization() {
        let config = OmnisystemConfig::default();
        let json = serde_json::to_string(&config).unwrap();
        let deserialized: OmnisystemConfig = serde_json::from_str(&json).unwrap();
        assert_eq!(config.system_id, deserialized.system_id);
    }
}
