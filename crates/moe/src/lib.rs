pub mod experts;
pub mod load_balancer;
pub mod routing;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MoeConfig {
    pub num_experts: usize,
    pub active_experts: usize,
    pub layer_ranges: Vec<(usize, usize, RoutingStrategy)>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RoutingStrategy {
    HashBased,
    LearnedTopK,
    SparseActivation,
}

pub struct MultiStreamMoE {
    config: MoeConfig,
    experts: Vec<experts::ExpertGroup>,
    router: routing::Router,
}

impl MultiStreamMoE {
    pub fn new(config: MoeConfig) -> Self {
        Self {
            experts: experts::create_expert_group(config.num_experts),
            router: routing::Router::new(&config),
            config,
        }
    }

    pub fn forward(&self, tokens: &[u32], layer: usize) -> Vec<Vec<f32>> {
        let strategy = self
            .config
            .layer_ranges
            .iter()
            .find(|(start, end, _)| layer >= *start && layer <= *end)
            .map(|(_, _, s)| s)
            .unwrap_or(&RoutingStrategy::HashBased);
        let _selected = self.router.route(tokens, layer, strategy);
        vec![vec![0.0; 256]; tokens.len()]
    }

    pub fn expert_group_count(&self) -> usize {
        self.experts.len()
    }
}
