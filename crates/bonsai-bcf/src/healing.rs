use crate::Result;

pub struct SurvivalSystem;

impl SurvivalSystem {
    pub fn new() -> Self {
        Self
    }

    pub async fn monitor_loop(&self) {
        loop {
            tokio::time::sleep(std::time::Duration::from_secs(1)).await;
        }
    }
}
