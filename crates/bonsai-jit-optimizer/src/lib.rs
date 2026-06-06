pub mod profiler;
pub mod optimizer;
pub mod cache;
pub mod inline;

use core_ir::LairFunction;

pub struct JitOptimizer {
    profiler: profiler::HotspotProfiler,
    optimizer: optimizer::PassManager,
    cache: cache::CodeCache,
    inliner: inline::Inliner,
}

impl JitOptimizer {
    pub fn new() -> Self {
        Self {
            profiler: profiler::HotspotProfiler::new(),
            optimizer: optimizer::PassManager::default(),
            cache: cache::CodeCache::new(),
            inliner: inline::Inliner::new(),
        }
    }
    
    pub fn optimize(&mut self, func: &mut LairFunction) -> anyhow::Result<()> {
        let hash = blake3::hash(&serde_json::to_vec(&func.body).unwrap_or_default());
        
        if let Some(cached) = self.cache.get(&hash) {
            func.body = cached;
            return Ok(());
        }
        
        let hotspots = self.profiler.analyze(func);
        self.inliner.inline_calls(func, &hotspots);
        self.optimizer.run(func);
        self.cache.put(hash, func.body.clone());
        
        Ok(())
    }
}
