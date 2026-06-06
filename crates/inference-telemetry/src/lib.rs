use serde::{Serialize, Deserialize};
use tokio::sync::broadcast::{Sender, channel};

#[derive(Clone, Serialize, Deserialize)]
pub struct TelemetryEvent {
    pub topic: String,
    pub payload: serde_json::Value,
}

pub struct TelemetryClient {
    tx: Sender<TelemetryEvent>,
}

impl TelemetryClient {
    pub fn new(buffer: usize) -> Self {
        let (tx, _) = channel(buffer);
        Self { tx }
    }

    pub fn subscribe(&self) -> tokio::sync::broadcast::Receiver<TelemetryEvent> {
        self.tx.subscribe()
    }

    pub fn publish(&self, ev: TelemetryEvent) {
        let _ = self.tx.send(ev);
    }
}
use serde::{Serialize, Deserialize};
use std::sync::Arc;
use std::time::Instant;

#[derive(Serialize, Deserialize, Debug)]
pub struct InferenceEvent {
	pub request_id: String,
	pub model: String,
	pub duration_ms: u64,
	pub tokens: usize,
	pub host: String,
}

pub struct TelemetryClient {
	// In real code this would hold exporters, http clients, etc.
}

impl TelemetryClient {
	pub fn new() -> Self { Self {} }
	pub async fn record(&self, ev: InferenceEvent) -> anyhow::Result<()> {
		tracing::info!("telemetry: {:?}", ev);
		Ok(())
	}
}

pub async fn record_inference(request_id: &str, model: &str, start: Instant, tokens: usize) {
	let dur = start.elapsed().as_millis() as u64;
	let ev = InferenceEvent { request_id: request_id.to_string(), model: model.to_string(), duration_ms: dur, tokens, host: hostname::get().unwrap_or_default().to_string_lossy().into_owned() };
	let client = TelemetryClient::new();
	let _ = client.record(ev).await;
}

#[cfg(test)]
mod tests { use super::*; #[tokio::test] async fn smoke() { let start = Instant::now(); record_inference("r1","m", start, 10).await; assert!(true); } }
