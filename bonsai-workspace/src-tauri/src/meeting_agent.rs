use std::sync::Arc;
use tokio::sync::{Mutex, broadcast};
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct TranscriptSegment {
    pub speaker: String,
    pub text: String,
    pub timestamp_ms: u64,
}

pub struct MeetingAgent {
    pub session_id: String,
    pub transcripts: Mutex<Vec<TranscriptSegment>>,
    pub running: Mutex<bool>,
    pub progress_tx: broadcast::Sender<MeetingProgress>,
}

#[derive(Clone, Serialize)]
pub struct MeetingProgress {
    pub event: String,
    pub data: String,
}

impl MeetingAgent {
    pub fn new() -> (Self, broadcast::Receiver<MeetingProgress>) {
        let (tx, rx) = broadcast::channel(64);
        (
            Self {
                session_id: uuid::Uuid::new_v4().to_string(),
                transcripts: Mutex::new(vec![]),
                running: Mutex::new(false),
                progress_tx: tx,
            },
            rx,
        )
    }

    pub async fn start(&self, _audio_source: &str) -> Result<(), String> {
        *self.running.lock().await = true;
        let _ = self.progress_tx.send(MeetingProgress {
            event: "started".into(),
            data: self.session_id.clone(),
        });
        Ok(())
    }

    pub async fn stop(&self) -> Result<String, String> {
        *self.running.lock().await = false;
        let transcripts = self.transcripts.lock().await;
        let summary = format!("Meeting summary: {} segments", transcripts.len());
        let _ = self.progress_tx.send(MeetingProgress {
            event: "stopped".into(),
            data: summary.clone(),
        });
        Ok(summary)
    }

    pub async fn ask(&self, question: &str) -> Result<String, String> {
        let tx = self.transcripts.lock().await;
        let ctx: String = tx.iter()
            .map(|s| format!("{}: {}", s.speaker, s.text))
            .collect::<Vec<_>>()
            .join("\n");
        Ok(format!("Based on the meeting:\n{ctx}\n\nAnswer to '{question}': ..."))
    }
}

#[tauri::command]
pub async fn start_meeting_agent(state: tauri::State<'_, crate::AppState>, audio_source: String) -> Result<String, String> {
    let agent = &state.meeting_agent;
    agent.start(&audio_source).await.map_err(|e| e.to_string())?;
    Ok(agent.session_id.clone())
}

#[tauri::command]
pub async fn stop_meeting_agent(state: tauri::State<'_, crate::AppState>) -> Result<String, String> {
    state.meeting_agent.stop().await
}

#[tauri::command]
pub async fn ask_meeting_agent(state: tauri::State<'_, crate::AppState>, question: String) -> Result<String, String> {
    state.meeting_agent.ask(&question).await
}
