use bonsai_marketplace::*;
use tauri::State;
use std::sync::Arc;
use tokio::sync::Mutex;

pub struct MarketState {
    pub marketplace: Mutex<Marketplace>,
}

#[tauri::command]
pub async fn publish_asset(state: State<'_, MarketState>, asset: Asset) -> Result<(), String> {
    state.marketplace.lock().await.publish(asset).await;
    Ok(())
}

#[tauri::command]
pub async fn search_marketplace(state: State<'_, MarketState>, query: String) -> Result<Vec<Asset>, String> {
    Ok(state.marketplace.lock().await.search(&query).await)
}

#[tauri::command]
pub async fn install_asset(state: State<'_, MarketState>, cid: String) -> Result<String, String> {
    Ok(format!("installed {cid}"))
}
