use serde::{Serialize, Deserialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulateRequest {
    pub device_file: String,
    pub cycles: Option<u64>,
    pub snapshot_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EmulateResponse {
    pub session_id: String,
    pub cycles_executed: u64,
    pub results: Vec<serde_json::Value>,
    pub exception: Option<String>,
}

pub fn emulate_tool_schema() -> serde_json::Value {
    serde_json::json!({
        "name": "emulate_device",
        "description": "Launch an emulation session for a device defined in HDL.",
        "parameters": {
            "type": "object",
            "properties": {
                "device_file": { "type": "string", "description": "Path to the HDL device definition file." },
                "cycles": { "type": "integer", "description": "Number of cycles to execute." },
                "snapshot_id": { "type": "string", "description": "Optional snapshot to restore from." }
            },
            "required": ["device_file"]
        }
    })
}

pub fn snapshot_tool_schema() -> serde_json::Value {
    serde_json::json!({
        "name": "emulator_snapshot",
        "description": "Take a snapshot of a running emulation session.",
        "parameters": {
            "type": "object",
            "properties": {
                "session_id": { "type": "string" }
            },
            "required": ["session_id"]
        }
    })
}

pub fn restore_tool_schema() -> serde_json::Value {
    serde_json::json!({
        "name": "emulator_restore",
        "description": "Restore an emulation session from a snapshot.",
        "parameters": {
            "type": "object",
            "properties": {
                "session_id": { "type": "string" },
                "snapshot_id": { "type": "string" }
            },
            "required": ["session_id", "snapshot_id"]
        }
    })
}
