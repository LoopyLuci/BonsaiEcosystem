use bonsai_auth::*;
use tauri::State;
use std::sync::Arc;
use tokio::sync::RwLock;

pub struct AuthState {
    pub active_session: RwLock<Option<Session>>,
    pub profiles: RwLock<Vec<UserProfile>>,
    pub workspaces: RwLock<Vec<Workspace>>,
}

#[tauri::command]
pub async fn create_profile(state: State<'_, AuthState>, passphrase: String, display_name: String) -> Result<UserProfile, String> {
    let (p, _sk) = create_profile(&passphrase, &display_name);
    state.profiles.write().await.push(p.clone());
    Ok(p)
}

#[tauri::command]
pub async fn unlock_profile(state: State<'_, AuthState>, profile_id: String, passphrase: String) -> Result<String, String> {
    let profiles = state.profiles.read().await;
    let p = profiles.iter().find(|p| p.id == profile_id).ok_or("profile not found")?;
    let session = unlock_profile(p, &passphrase).ok_or("wrong passphrase")?;
    let pid = session.profile_id.clone();
    *state.active_session.write().await = Some(session);
    Ok(pid)
}

#[tauri::command]
pub async fn lock_profile(state: State<'_, AuthState>) -> Result<(), String> {
    *state.active_session.write().await = None;
    Ok(())
}

#[tauri::command]
pub async fn create_workspace(state: State<'_, AuthState>, name: String) -> Result<Workspace, String> {
    let s = state.active_session.read().await;
    let session = s.as_ref().ok_or("no session")?;
    let ws = create_workspace(session, &name);
    state.workspaces.write().await.push(ws.clone());
    Ok(ws)
}

#[tauri::command]
pub async fn share_workspace(state: State<'_, AuthState>, workspace_id: String, grantee_pub: Vec<u8>, permissions: u8) -> Result<(), String> {
    let s = state.active_session.read().await;
    let session = s.as_ref().ok_or("no session")?;
    let mut ws_list = state.workspaces.write().await;
    let ws = ws_list.iter_mut().find(|w| w.id == workspace_id).ok_or("workspace not found")?;
    grant_access(session, ws, &grantee_pub, permissions);
    Ok(())
}

#[tauri::command]
pub async fn list_workspaces(state: State<'_, AuthState>) -> Result<Vec<Workspace>, String> {
    Ok(state.workspaces.read().await.clone())
}
