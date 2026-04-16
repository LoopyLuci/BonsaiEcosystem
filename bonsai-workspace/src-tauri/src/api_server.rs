//! OpenAI-compatible HTTP API server.
//!
//! Listens on `127.0.0.1:11369` by default (configurable in Settings) and exposes:
//!   GET  /v1/models              → list available models (OpenAI format)
//!   POST /v1/chat/completions    → proxy to active llama-server slot
//!   GET  /api/tags               → Ollama-compatible model list
//!   POST /api/chat               → Ollama-compatible chat
//!   POST /api/generate           → Ollama-compatible generate
//!   GET  /health                 → liveness probe
//!
//! External tools (Claude Code `--api-base`, GitHub Copilot, Continue.dev, etc.)
//! can point at `http://localhost:11369` and use the Bonsai models directly.
//! Default port is 11369.

use std::convert::Infallible;
use std::process::Command;
use std::sync::Arc;
use std::time::{Duration, SystemTime, UNIX_EPOCH};
use axum::{
    body::Body,
    extract::{State, ws::{WebSocket, WebSocketUpgrade}},
    http::{HeaderMap, HeaderValue, StatusCode},
    response::{IntoResponse, Response, Sse},
    response::sse::{Event, KeepAlive},
    routing::{get, post},
    Json, Router,
};
use base64::Engine;
use bytes::Bytes;
use futures::{StreamExt, SinkExt, stream::BoxStream};
use serde::Deserialize;
use serde_json::{json, Value};
use tokio_stream::wrappers::IntervalStream;
use tower_http::cors::{Any, CorsLayer};
use tokio::sync::oneshot;
use tokio::task::JoinHandle;

use crate::model_orchestrator::ModelOrchestrator;
use crate::model_registry::ModelInfo;
use crate::remote::RemoteManager;
use crate::remote_input::RemoteInputEvent;
use crate::ws_router::WsRouter;

const CONTENT_TYPE_JSON: HeaderValue = HeaderValue::from_static("application/json");

// ── Shared state ──────────────────────────────────────────────────────────────

#[derive(Clone)]
struct ApiState {
    orchestrator:    Arc<ModelOrchestrator>,
    client:          reqwest::Client,
    remote_manager:  Arc<RemoteManager>,
    ws_router:       Arc<WsRouter>,
    pair_token:      String,
    api_host:        String,
    api_port:        u16,
}

// ── Entry point ───────────────────────────────────────────────────────────────

pub struct ApiServerHandle {
    shutdown_tx: Option<oneshot::Sender<()>>,
    join:        JoinHandle<()>,
    pub host:    String,
    pub port:    u16,
}

impl ApiServerHandle {
    pub async fn stop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
        let _ = (&mut self.join).await;
    }
}

impl Drop for ApiServerHandle {
    fn drop(&mut self) {
        if let Some(tx) = self.shutdown_tx.take() {
            let _ = tx.send(());
        }
    }
}

pub async fn start(
    orchestrator: Arc<ModelOrchestrator>,
    remote_manager: Arc<RemoteManager>,
    ws_router: Arc<WsRouter>,
    pair_token: String,
    host: String,
    port: u16,
) -> Result<ApiServerHandle, String> {
    let state = ApiState {
        orchestrator,
        client: reqwest::Client::builder()
            .timeout(std::time::Duration::from_secs(300))
            .build()
            .unwrap_or_default(),
        remote_manager,
        ws_router,
        pair_token,
        api_host: host.clone(),
        api_port: port,
    };

    let cors = CorsLayer::new()
        .allow_origin(Any)
        .allow_methods(Any)
        .allow_headers(Any);

    let app = Router::new()
        // OpenAI-compatible
        .route("/v1/models",            get(list_models))
        .route("/v1/chat/completions",  post(chat_completions))
        // Ollama-compatible (for tools that speak Ollama)
        .route("/api/tags",             get(ollama_tags))
        .route("/api/chat",             post(ollama_chat))
        .route("/api/generate",         post(ollama_generate))
        // Remote control / screen capture
        .route("/remote/session/start", post(start_remote_session))
        .route("/remote/session/stop",  post(stop_remote_session))
        .route("/remote/session/offer", post(remote_session_offer))
        .route("/remote/input",         post(remote_input_event))
        .route("/remote/frame",         get(remote_frame))
        .route("/remote/stream",        get(remote_stream))
        // Meta
        .route("/health",               get(health))
        .route("/api/version",          get(ollama_version))
        // WebSocket — bidirectional relay for Android app + VSCode extension
        .route("/ws",                   get(ws_handler))
        .layer(cors)
        .with_state(state);

    let addr = format!("{host}:{port}");

    let listener = match tokio::net::TcpListener::bind(&addr).await {
        Ok(l) => l,
        Err(e) => {
            if e.kind() == std::io::ErrorKind::AddrInUse {
                // If a healthy Bonsai API is already bound, attach instead of failing.
                if is_api_healthy(&host, port).await {
                    eprintln!("[api] Port {addr} already in use by healthy API; attaching to existing runtime");
                    return Ok(ApiServerHandle {
                        shutdown_tx: None,
                        join: tokio::spawn(async {}),
                        host,
                        port,
                    });
                }

                // Direct EXE launches can inherit stale listeners from old Bonsai processes.
                // On Windows, reclaim those stale listeners and retry bind once.
                if try_reclaim_stale_bonsai_listener(port) {
                    tokio::time::sleep(Duration::from_millis(300)).await;
                    match tokio::net::TcpListener::bind(&addr).await {
                        Ok(l2) => l2,
                        Err(e2) => return Err(format!("Failed to bind {addr}: {e2}")),
                    }
                } else {
                    return Err(format!("Failed to bind {addr}: {e}"));
                }
            } else {
                return Err(format!("Failed to bind {addr}: {e}"));
            }
        }
    };

    eprintln!("[api] Bonsai API server listening on http://{addr}");

    let (shutdown_tx, shutdown_rx) = oneshot::channel::<()>();
    let join = tokio::spawn(async move {
        let server = axum::serve(listener, app).with_graceful_shutdown(async move {
            let _ = shutdown_rx.await;
        });
        if let Err(e) = server.await {
            eprintln!("[api] Server error: {e}");
        }
        eprintln!("[api] Bonsai API server stopped");
    });

    Ok(ApiServerHandle {
        shutdown_tx: Some(shutdown_tx),
        join,
        host,
        port,
    })
}

pub async fn start_with_fallback(
    orchestrator: Arc<ModelOrchestrator>,
    remote_manager: Arc<RemoteManager>,
    ws_router: Arc<WsRouter>,
    pair_token: String,
    host: String,
    preferred_port: u16,
    max_extra_attempts: u16,
) -> Result<ApiServerHandle, String> {
    let mut ports = Vec::with_capacity(max_extra_attempts as usize + 1);
    ports.push(preferred_port);
    for i in 1..=max_extra_attempts {
        if let Some(p) = preferred_port.checked_add(i) {
            ports.push(p);
        } else {
            break;
        }
    }

    let mut last_err = String::new();
    for p in ports {
        match start(
            orchestrator.clone(),
            remote_manager.clone(),
            ws_router.clone(),
            pair_token.clone(),
            host.clone(),
            p,
        )
        .await
        {
            Ok(handle) => return Ok(handle),
            Err(e) => last_err = e,
        }
    }

    Err(if last_err.is_empty() {
        format!(
            "Failed to start API server on {}:{} and fallback ports",
            host, preferred_port
        )
    } else {
        last_err
    })
}

async fn is_api_healthy(host: &str, port: u16) -> bool {
    let url = format!("http://{host}:{port}/health");
    match reqwest::Client::builder()
        .timeout(Duration::from_millis(1200))
        .build()
    {
        Ok(client) => client
            .get(url)
            .send()
            .await
            .is_ok_and(|r| r.status().is_success()),
        Err(_) => false,
    }
}

fn try_reclaim_stale_bonsai_listener(port: u16) -> bool {
    #[cfg(not(target_os = "windows"))]
    {
        let _ = port;
        false
    }

    #[cfg(target_os = "windows")]
    {
        let pids = listening_pids_on_port(port);
        if pids.is_empty() {
            return false;
        }

        let mut killed_any = false;
        for pid in pids {
            let image = process_image_name(pid);
            let img = image.to_ascii_lowercase();
            if img != "bonsai-workspace.exe" && img != "bonsai-workspace" {
                continue;
            }
            if let Ok(out) = Command::new("taskkill")
                .args(["/PID", &pid.to_string(), "/T", "/F"])
                .output()
            {
                if out.status.success() {
                    killed_any = true;
                }
            }
        }
        killed_any
    }
}

#[cfg(target_os = "windows")]
fn listening_pids_on_port(port: u16) -> Vec<u32> {
    let out = match Command::new("netstat").args(["-ano"]).output() {
        Ok(o) => o,
        Err(_) => return vec![],
    };
    let dump = String::from_utf8_lossy(&out.stdout);
    let mut pids = std::collections::BTreeSet::new();
    let needle = format!(":{port}");

    for line in dump.lines() {
        let l = line.trim();
        if l.is_empty() || !l.contains(&needle) || !l.to_ascii_uppercase().contains("LISTEN") {
            continue;
        }
        let parts: Vec<&str> = l.split_whitespace().collect();
        if let Some(last) = parts.last() {
            if let Ok(pid) = last.parse::<u32>() {
                if pid > 0 {
                    pids.insert(pid);
                }
            }
        }
    }

    pids.into_iter().collect()
}

#[cfg(target_os = "windows")]
fn process_image_name(pid: u32) -> String {
    let out = match Command::new("tasklist")
        .args(["/FI", &format!("PID eq {pid}"), "/FO", "CSV", "/NH"])
        .output()
    {
        Ok(o) => o,
        Err(_) => return String::new(),
    };
    let text = String::from_utf8_lossy(&out.stdout);
    let line = text.trim();
    if line.is_empty() || line.contains("No tasks are running") {
        return String::new();
    }
    line.split(',')
        .next()
        .map(|s| s.trim_matches('"').to_string())
        .unwrap_or_default()
}

// ── Health ────────────────────────────────────────────────────────────────────

async fn health() -> impl IntoResponse {
    Json(json!({ "status": "ok", "service": "bonsai-workspace" }))
}

async fn ollama_version() -> impl IntoResponse {
    Json(json!({ "version": "0.1.0-bonsai" }))
}
// ── Remote session / screen capture ───────────────────────────────────────────────

async fn start_remote_session(State(s): State<ApiState>) -> impl IntoResponse {
    match s.remote_manager.start_session().await {
        Ok(session) => Json(json!({
            "session_id": session.id,
            "state": session.state,
            "stream_url": format!("http://{}:{}/remote/stream", s.api_host, s.api_port),
            "frame_url": format!("http://{}:{}/remote/frame", s.api_host, s.api_port),
            "input_url": format!("http://{}:{}/remote/input", s.api_host, s.api_port),
        })).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err}))).into_response(),
    }
}

async fn stop_remote_session(State(s): State<ApiState>) -> impl IntoResponse {
    match s.remote_manager.stop_session().await {
        Ok(()) => (StatusCode::OK, Json(json!({"status": "stopped"}))).into_response(),
        Err(err) => (StatusCode::INTERNAL_SERVER_ERROR, Json(json!({"error": err}))).into_response(),
    }
}

async fn remote_session_offer(State(s): State<ApiState>, Json(payload): Json<Value>) -> impl IntoResponse {
    let Some(session) = s.remote_manager.get_active_session() else {
        return (StatusCode::BAD_REQUEST, Json(json!({"error": "No active remote session"}))).into_response();
    };

    Json(json!({
        "session_id": session.id,
        "answer": {
            "status": "ready",
            "received_offer": payload,
        }
    })).into_response()
}

async fn remote_frame(State(s): State<ApiState>) -> Response {
    let result = tokio::task::spawn_blocking(move || s.remote_manager.capture_png()).await;
    match result {
        Ok(Ok(bytes)) => Response::builder()
            .status(StatusCode::OK)
            .header("content-type", "image/png")
            .body(Body::from(bytes))
            .unwrap(),
        Ok(Err(err)) => Response::builder()
            .status(StatusCode::BAD_REQUEST)
            .header("content-type", "application/json")
            .body(Body::from(Bytes::from(json!({"error": err}).to_string())))
            .unwrap(),
        Err(err) => Response::builder()
            .status(StatusCode::INTERNAL_SERVER_ERROR)
            .header("content-type", "application/json")
            .body(Body::from(Bytes::from(json!({"error": err.to_string()}).to_string())))
            .unwrap(),
    }
}

async fn remote_stream(State(s): State<ApiState>) -> Sse<BoxStream<'static, Result<Event, Infallible>>> {
    let stream = IntervalStream::new(tokio::time::interval(Duration::from_secs(1)))
        .then(move |_| {
            let manager = s.remote_manager.clone();
            async move {
                let event = match tokio::task::spawn_blocking(move || manager.capture_png()).await {
                    Ok(Ok(image_bytes)) => {
                        let encoded = base64::engine::general_purpose::STANDARD.encode(&image_bytes);
                        let now = SystemTime::now().duration_since(UNIX_EPOCH).unwrap_or_default().as_millis();
                        let payload = json!({"timestamp": now, "frame": encoded}).to_string();
                        Event::default().data(payload)
                    }
                    Ok(Err(err)) => Event::default().event("error").data(err),
                    Err(err) => Event::default().event("error").data(err.to_string()),
                };
                Ok::<Event, Infallible>(event)
            }
        })
        .boxed();

    Sse::new(stream).keep_alive(KeepAlive::new().interval(Duration::from_secs(10)))
}

async fn remote_input_event(
    State(s): State<ApiState>,
    Json(event): Json<RemoteInputEvent>,
) -> impl IntoResponse {
    match s.remote_manager.submit_input(event).await {
        Ok(()) => Json(json!({"status": "accepted"})).into_response(),
        Err(err) => (StatusCode::BAD_REQUEST, Json(json!({"error": err}))).into_response(),
    }
}
// ── Model listing ─────────────────────────────────────────────────────────────

async fn list_models(State(s): State<ApiState>) -> impl IntoResponse {
    let models = s.orchestrator.list_models().await;
    let data: Vec<Value> = models.iter().map(openai_model_object).collect();
    Json(json!({ "object": "list", "data": data }))
}

fn openai_model_object(m: &ModelInfo) -> Value {
    json!({
        "id":       m.id,
        "object":   "model",
        "owned_by": "bonsai",
        "created":  0,
        "name":     m.name,
        "quant":    m.quant_label,
        "ram_mb":   m.ram_required_mb,
    })
}

// Ollama-compatible model list
async fn ollama_tags(State(s): State<ApiState>) -> impl IntoResponse {
    let models = s.orchestrator.list_models().await;
    let list: Vec<Value> = models.iter().map(|m| json!({
        "name":        format!("{}:latest", m.name.to_lowercase().replace(' ', "-")),
        "model":       m.id,
        "modified_at": "2024-01-01T00:00:00Z",
        "size":        m.file_size_bytes,
        "details": { "parameter_size": format!("{}B", m.parameter_count / 1_000_000_000) }
    })).collect();
    Json(json!({ "models": list }))
}

// ── Chat completions (OpenAI-compatible) ──────────────────────────────────────

async fn chat_completions(
    State(s): State<ApiState>,
    headers: HeaderMap,
    body: Bytes,
) -> Response {
    proxy_to_llama(s, "/v1/chat/completions", headers, body).await
}

// ── Ollama chat / generate ────────────────────────────────────────────────────

#[derive(Deserialize)]
struct OllamaChat {
    model:    Option<String>,
    messages: Option<Vec<Value>>,
    prompt:   Option<String>,
    stream:   Option<bool>,
}

async fn ollama_chat(
    State(s): State<ApiState>,
    Json(body): Json<OllamaChat>,
) -> Response {
    // Convert Ollama format → OpenAI format and proxy
    let messages = body.messages.unwrap_or_else(|| {
        if let Some(p) = body.prompt {
            vec![json!({ "role": "user", "content": p })]
        } else {
            vec![]
        }
    });
    let openai_body = json!({
        "model":    body.model.unwrap_or_else(|| "local".into()),
        "messages": messages,
        "stream":   body.stream.unwrap_or(false),
    });
    let body_bytes = Bytes::from(serde_json::to_vec(&openai_body).unwrap_or_default());
    let mut headers = HeaderMap::new();
    headers.insert("content-type", CONTENT_TYPE_JSON.clone());
    proxy_to_llama(s, "/v1/chat/completions", headers, body_bytes).await
}

async fn ollama_generate(
    State(s): State<ApiState>,
    Json(body): Json<OllamaChat>,
) -> Response {
    // Map to chat completions
    let prompt = body.prompt.unwrap_or_default();
    let openai_body = json!({
        "model":    body.model.unwrap_or_else(|| "local".into()),
        "messages": [{ "role": "user", "content": prompt }],
        "stream":   body.stream.unwrap_or(false),
    });
    let body_bytes = Bytes::from(serde_json::to_vec(&openai_body).unwrap_or_default());
    let mut headers = HeaderMap::new();
    headers.insert("content-type", CONTENT_TYPE_JSON.clone());
    proxy_to_llama(s, "/v1/chat/completions", headers, body_bytes).await
}

// ── WebSocket relay ───────────────────────────────────────────────────────────

async fn ws_handler(
    ws: WebSocketUpgrade,
    State(s): State<ApiState>,
) -> impl IntoResponse {
    ws.on_upgrade(move |socket| handle_ws(socket, s.ws_router.clone(), s.pair_token.clone()))
}

async fn handle_ws(socket: WebSocket, router: Arc<WsRouter>, pair_token: String) {
    use axum::extract::ws::Message;

    let (mut sender, mut receiver) = socket.split();

    // Wait for auth message (first message must be auth).
    let authed = match receiver.next().await {
        Some(Ok(Message::Text(txt))) => {
            if let Ok(v) = serde_json::from_str::<Value>(&txt) {
                let msg_type = v.get("type").and_then(|t| t.as_str()).unwrap_or("");
                let token    = v.pointer("/payload/token").and_then(|t| t.as_str()).unwrap_or("");
                msg_type == "auth" && (pair_token.is_empty() || token == pair_token)
            } else {
                false
            }
        }
        _ => false,
    };

    if !authed {
        let _ = sender.send(Message::Text(
            json!({"type":"auth_fail","payload":{"reason":"invalid token"}}).to_string(),
        )).await;
        return;
    }

    let _ = sender.send(Message::Text(
        json!({"type":"auth_ok","payload":{}}).to_string(),
    )).await;

    let (client_id, mut rx) = router.register();
    eprintln!("[ws] client {client_id} connected ({} total)", router.client_count());

    // Spawn a task that drains the broadcast channel → WebSocket sink.
    let send_task = tokio::spawn(async move {
        while let Some(msg) = rx.recv().await {
            if sender.send(msg).await.is_err() {
                break;
            }
        }
    });

    // Receive loop — relay inbound client messages to broadcast (other clients see them too).
    while let Some(Ok(msg)) = receiver.next().await {
        match &msg {
            Message::Close(_) => break,
            Message::Text(_) | Message::Binary(_) => {
                router.broadcast(msg);
            }
            _ => {}
        }
    }

    router.unregister(client_id);
    send_task.abort();
    eprintln!("[ws] client {client_id} disconnected ({} remaining)", router.client_count());
}

// ── Core proxy ────────────────────────────────────────────────────────────────

async fn proxy_to_llama(
    s: ApiState,
    path: &str,
    _headers: HeaderMap,
    body: Bytes,
) -> Response {
    let Some(base_url) = s.orchestrator.active_slot_url().await else {
        return (
            StatusCode::SERVICE_UNAVAILABLE,
            Json(json!({ "error": "No model is currently loaded. Download and select a model first." })),
        ).into_response();
    };

    let url = format!("{base_url}{path}");

    match s.client
        .post(&url)
        .header("content-type", "application/json")
        .body(body)
        .send()
        .await
    {
        Err(e) => (
            StatusCode::BAD_GATEWAY,
            Json(json!({ "error": format!("llama-server unreachable: {e}") })),
        ).into_response(),

        Ok(resp) => {
            let status = StatusCode::from_u16(resp.status().as_u16())
                .unwrap_or(StatusCode::INTERNAL_SERVER_ERROR);
            let mut out_headers = HeaderMap::new();
            for (k, v) in resp.headers() {
                if let (Ok(k2), Ok(v2)) = (
                    axum::http::HeaderName::from_bytes(k.as_str().as_bytes()),
                    axum::http::HeaderValue::from_bytes(v.as_bytes()),
                ) {
                    out_headers.insert(k2, v2);
                }
            }
            let body_bytes = resp.bytes().await.unwrap_or_default();
            (status, out_headers, body_bytes).into_response()
        }
    }
}
