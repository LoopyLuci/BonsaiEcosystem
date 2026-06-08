//! Terminal User Interface for Bonsai Ecosystem

use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone)]
pub struct TuiState {
    pub active_tab: String,
    pub messages: Vec<String>,
    pub models: Vec<String>,
}

pub struct BonsaiTui {
    state: Arc<RwLock<TuiState>>,
}

impl BonsaiTui {
    pub fn new() -> Self {
        Self {
            state: Arc::new(RwLock::new(TuiState {
                active_tab: "dashboard".to_string(),
                messages: vec![],
                models: vec![],
            })),
        }
    }

    pub async fn initialize(&self) -> anyhow::Result<()> {
        let mut state = self.state.write().await;
        state.messages.push("Initializing Bonsai TUI...".to_string());
        Ok(())
    }

    pub async fn render(&self) -> anyhow::Result<String> {
        let state = self.state.read().await;
        let output = format!(
            "=== Bonsai Ecosystem ===\nTab: {}\nModels: {}\n",
            state.active_tab,
            state.models.len()
        );
        Ok(output)
    }

    pub async fn handle_input(&self, input: &str) -> anyhow::Result<()> {
        let mut state = self.state.write().await;
        match input {
            "models" => state.active_tab = "models".to_string(),
            "dashboard" => state.active_tab = "dashboard".to_string(),
            "monitor" => state.active_tab = "monitor".to_string(),
            _ => state.messages.push(format!("Unknown command: {}", input)),
        }
        Ok(())
    }

    pub async fn get_state(&self) -> TuiState {
        self.state.read().await.clone()
    }
}

impl Default for BonsaiTui {
    fn default() -> Self {
        Self::new()
    }

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_tui_creation() {
        let tui = BonsaiTui::new();
        tui.initialize().await.unwrap();
        let output = tui.render().await.unwrap();
        assert!(output.contains("Bonsai Ecosystem"));
    }

    #[tokio::test]
    async fn test_input_handling() {
        let tui = BonsaiTui::new();
        tui.handle_input("models").await.unwrap();
        let state = tui.get_state().await;
        assert_eq!(state.active_tab, "models");
    }
}
