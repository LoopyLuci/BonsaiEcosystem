pub mod emitter;
pub mod event;
pub mod snapshot;
pub mod store;

pub use emitter::UniverseEmitter;
pub use event::{
    EventCategory, EventSource, RevertPreview, SubsystemHashes, TimelineFilter,
    UniverseEvent, UniverseSnapshot,
};
pub use snapshot::{RetentionPolicy, SnapshotEngine};
pub use store::UniverseStore;

use std::sync::Arc;

/// Top-level handle grouping store + emitter + snapshot engine.
pub struct Universe {
    pub store: Arc<UniverseStore>,
    pub emitter: Arc<UniverseEmitter>,
    pub snapshots: Arc<SnapshotEngine>,
}

impl Universe {
    pub async fn open(db_path: &std::path::Path, device_id: impl Into<String>) -> Result<Arc<Self>, String> {
        let store = UniverseStore::open(db_path, device_id).await?;
        let emitter = UniverseEmitter::spawn(store.clone(), 4096);

        // Load retention policy from config/time_travel.yaml (relative to cwd or workspace root).
        let config_candidates = [
            std::path::PathBuf::from("config/time_travel.yaml"),
            std::path::PathBuf::from("../../config/time_travel.yaml"),
        ];
        let retention = config_candidates.iter()
            .find(|p| p.exists())
            .map(|p| RetentionPolicy::from_yaml(p))
            .unwrap_or_default();

        let snapshots = Arc::new(
            SnapshotEngine::new(store.clone(), emitter.clone())
                .with_interval(retention.pruning_interval_hours.max(1) * 60)
                .with_retention(retention),
        );
        Ok(Arc::new(Self { store, emitter, snapshots }))
    }

    /// Convenience: emit a file-change event.
    pub fn record_file_change(
        &self,
        path: &str,
        before_hash: Option<String>,
        after_hash: Option<String>,
        source: EventSource,
    ) {
        let mut ev = UniverseEvent::new(
            source,
            EventCategory::FileChange,
            format!("File changed: {}", path),
            path.to_string(),
            self.store.device_id().to_string(),
        )
        .with_hashes(before_hash, after_hash);
        ev.metadata = serde_json::json!({ "path": path });
        self.emitter.emit(ev);
    }
}
