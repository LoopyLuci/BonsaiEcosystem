use reqwest::Client;
use serde_json::Value;
use anyhow::Result;
use lazy_static::lazy_static;

lazy_static! {
    static ref HTTP_CLIENT: Client = Client::new();
    static ref DAEMON_URL: String = std::env::var("BONSAI_DAEMON_URL")
        .unwrap_or_else(|_| "http://127.0.0.1:8080/api".to_string());
}

pub async fn call_bonsai(token: &str, tool_name: &str, args: Value) -> Result<Value> {
    let endpoint = match tool_name {
        "read_file" => "file/read",
        "write_file" => "file/write",
        "chat" => "chat",
        "run_cargo_check" => "tools/run",
        "run_cargo_test" => "tools/run",
        "pull_model" => "models/pull",
        "list_models" => "models/list",
        "submit_issue" => "issues/create",
        "suggest_fix" => "survival/suggest_fix",
        _ => tool_name,
    };
    let url = format!("{}/{}", *DAEMON_URL, endpoint);
    let response = HTTP_CLIENT
        .post(&url)
        .header("Authorization", format!("Bearer {}", token))
        .json(&args)
        .send()
        .await?
        .error_for_status()?;
    Ok(response.json().await?)
}
