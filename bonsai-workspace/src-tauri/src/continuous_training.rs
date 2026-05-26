use std::sync::Arc;
use tokio::sync::Mutex;
use std::time::Duration;
use serde::Serialize;

#[derive(Clone, Serialize)]
pub struct TrainingStatus {
    pub examples_collected: usize,
    pub threshold: usize,
    pub running: bool,
    pub last_adapter: Option<String>,
    pub last_f1: Option<f32>,
}

pub struct ContinuousTrainer {
    pub feedback_buffer: Mutex<Vec<FeedbackTriple>>,
    pub status: Mutex<TrainingStatus>,
    pub trigger_threshold: usize,
}

#[derive(Clone, Serialize)]
pub struct FeedbackTriple {
    pub prompt: String,
    pub response: String,
    pub feedback: String,
}

impl ContinuousTrainer {
    pub fn new() -> Self {
        Self {
            feedback_buffer: Mutex::new(vec![]),
            status: Mutex::new(TrainingStatus {
                examples_collected: 0,
                threshold: 50,
                running: false,
                last_adapter: None,
                last_f1: None,
            }),
            trigger_threshold: 50,
        }
    }

    pub async fn ingest(&self, prompt: &str, response: &str, feedback: &str) {
        self.feedback_buffer.lock().await.push(FeedbackTriple {
            prompt: prompt.into(),
            response: response.into(),
            feedback: feedback.into(),
        });
        let buf = self.feedback_buffer.lock().await;
        let mut s = self.status.lock().await;
        s.examples_collected = buf.len();
    }

    pub async fn should_train(&self) -> bool {
        self.feedback_buffer.lock().await.len() >= self.trigger_threshold
    }

    pub async fn run_cycle(&self) -> Result<TrainingStatus, String> {
        let mut s = self.status.lock().await;
        s.running = true;
        // Simulate training delay (in real: call fine‑tune API)
        tokio::time::sleep(Duration::from_secs(5)).await;
        s.running = false;
        s.examples_collected = 0;
        self.feedback_buffer.lock().await.clear();
        s.last_adapter = Some(format!("bonsai-core-v{}", rand::random::<u16>()));
        s.last_f1 = Some(0.92);
        Ok(s.clone())
    }
}

#[tauri::command]
pub async fn ingest_feedback(state: tauri::State<'_, crate::AppState>, prompt: String, response: String, feedback: String) -> Result<(), String> {
    state.continuous_trainer.ingest(&prompt, &response, &feedback).await;
    Ok(())
}

#[tauri::command]
pub async fn get_training_status(state: tauri::State<'_, crate::AppState>) -> Result<TrainingStatus, String> {
    Ok(state.continuous_trainer.status.lock().await.clone())
}

#[tauri::command]
pub async fn trigger_training(state: tauri::State<'_, crate::AppState>) -> Result<TrainingStatus, String> {
    state.continuous_trainer.run_cycle().await
}
