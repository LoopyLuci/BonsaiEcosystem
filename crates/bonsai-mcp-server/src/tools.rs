use crate::McpTool;
use serde_json::json;

pub fn list_tools() -> Vec<McpTool> {
    vec![
        McpTool {
            name: "read_file".into(),
            description: "Read the contents of a file in the Bonsai workspace.".into(),
            input_schema: json!({"type":"object","properties":{"path":{"type":"string"}},"required":["path"]}),
        },
        McpTool {
            name: "write_file".into(),
            description: "Create or overwrite a file.".into(),
            input_schema: json!({"type":"object","properties":{"path":{"type":"string"},"content":{"type":"string"}},"required":["path","content"]}),
        },
        McpTool {
            name: "chat".into(),
            description: "Send a message to BonsAI and get a response.".into(),
            input_schema: json!({"type":"object","properties":{"message":{"type":"string"}},"required":["message"]}),
        },
        McpTool {
            name: "run_cargo_check".into(),
            description: "Run `cargo check --workspace` and return the output.".into(),
            input_schema: json!({"type":"object","properties":{}}),
        },
        McpTool {
            name: "run_cargo_test".into(),
            description: "Run `cargo test --workspace` and return the output.".into(),
            input_schema: json!({"type":"object","properties":{}}),
        },
        McpTool {
            name: "pull_model".into(),
            description: "Download a model from the Bonsai model registry.".into(),
            input_schema: json!({"type":"object","properties":{"model_name":{"type":"string"}},"required":["model_name"]}),
        },
        McpTool {
            name: "list_models".into(),
            description: "List locally available models.".into(),
            input_schema: json!({"type":"object","properties":{}}),
        },
        McpTool {
            name: "submit_issue".into(),
            description: "Create an issue in the Bonsai Issue Tracker.".into(),
            input_schema: json!({"type":"object","properties":{"title":{"type":"string"},"body":{"type":"string"}},"required":["title","body"]}),
        },
        McpTool {
            name: "suggest_fix".into(),
            description: "Get a fix suggestion from the Survival System for an error.".into(),
            input_schema: json!({"type":"object","properties":{"error_description":{"type":"string"}},"required":["error_description"]}),
        },
        // Android Bridge tools
        McpTool {
            name: "android_list_devices".into(),
            description: "List all connected Android devices with metadata (model, API level, battery, etc).".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "status_filter": {"type": "string", "description": "Optional: filter by device status (connected, pairing, offline)"}
                },
                "required": []
            }),
        },
        McpTool {
            name: "android_connect".into(),
            description: "Connect to a specific Android device for control and streaming.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "device_id": {"type": "string", "description": "The device ID to connect to"},
                    "pairing_token": {"type": "string", "description": "Optional: QR pairing token"}
                },
                "required": ["device_id"]
            }),
        },
        McpTool {
            name: "android_start_screen_stream".into(),
            description: "Start H.264/H.265 screen streaming from an Android device.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "device_id": {"type": "string"},
                    "bitrate": {"type": "number", "description": "Bitrate in kbps (default: 5000)"},
                    "resolution": {"type": "string", "description": "Resolution: 1080p, 720p, or 480p"}
                },
                "required": ["device_id"]
            }),
        },
        McpTool {
            name: "android_stop_screen_stream".into(),
            description: "Stop screen streaming from an Android device.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "device_id": {"type": "string"}
                },
                "required": ["device_id"]
            }),
        },
        McpTool {
            name: "android_inject_touch".into(),
            description: "Inject touch input (tap, swipe) at coordinates on the Android device screen.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "device_id": {"type": "string"},
                    "x": {"type": "number", "description": "X coordinate (0-screen_width)"},
                    "y": {"type": "number", "description": "Y coordinate (0-screen_height)"},
                    "action": {"type": "string", "enum": ["DOWN", "MOVE", "UP"]},
                    "pointer_id": {"type": "number", "description": "Optional: multi-touch pointer ID"}
                },
                "required": ["device_id", "x", "y", "action"]
            }),
        },
        McpTool {
            name: "android_inject_key".into(),
            description: "Inject a keycode event on the Android device.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "device_id": {"type": "string"},
                    "keycode": {"type": "number", "description": "Android keycode (3=KEYCODE_HOME, 4=KEYCODE_BACK)"},
                    "down": {"type": "boolean", "description": "true for keydown, false for keyup"}
                },
                "required": ["device_id", "keycode", "down"]
            }),
        },
        McpTool {
            name: "android_install_app".into(),
            description: "Install an APK file on an Android device.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "device_id": {"type": "string"},
                    "apk_path": {"type": "string", "description": "Path to APK file on desktop"}
                },
                "required": ["device_id", "apk_path"]
            }),
        },
        McpTool {
            name: "android_hot_reload".into(),
            description: "Trigger hot reload on the Android app for changed files without full reinstall.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "device_id": {"type": "string"},
                    "changed_files": {
                        "type": "array",
                        "items": {"type": "string"},
                        "description": "List of changed file paths (relative to project root)"
                    }
                },
                "required": ["device_id", "changed_files"]
            }),
        },
        // Mobile Remote Desktop tools
        McpTool {
            name: "mobile_start_remote_session".into(),
            description: "Initiate a remote desktop session from mobile to desktop peer with encryption and capability tokens.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "peer_id": {
                        "type": "string",
                        "description": "UUID of the desktop peer to connect to"
                    },
                    "token": {
                        "type": "string",
                        "description": "Capability token granting access to this peer (generated via BRDF pairing)"
                    },
                    "encryption_key": {
                        "type": "string",
                        "description": "Optional: pre-shared encryption key for TLS session"
                    }
                },
                "required": ["peer_id", "token"]
            }),
        },
        McpTool {
            name: "mobile_stop_remote_session".into(),
            description: "Gracefully terminate a remote desktop session.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "session_id": {
                        "type": "string",
                        "description": "Session ID returned from mobile_start_remote_session"
                    }
                },
                "required": ["session_id"]
            }),
        },
        McpTool {
            name: "mobile_inject_text".into(),
            description: "Inject text input into the remote desktop (supports voice dictation, clipboard paste).".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "session_id": {
                        "type": "string",
                        "description": "Active session ID"
                    },
                    "text": {
                        "type": "string",
                        "description": "Text to inject (may include newlines for multi-line input)"
                    },
                    "target_field": {
                        "type": "string",
                        "description": "Optional: target field identifier (for focused input)"
                    }
                },
                "required": ["session_id", "text"]
            }),
        },
        McpTool {
            name: "mobile_take_screenshot".into(),
            description: "Capture current desktop screen as base64-encoded JPEG/PNG for display on mobile.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "session_id": {
                        "type": "string",
                        "description": "Active session ID"
                    },
                    "quality": {
                        "type": "number",
                        "description": "JPEG quality 0-100 (default: 85 for balance of size/quality)"
                    },
                    "format": {
                        "type": "string",
                        "enum": ["jpeg", "png"],
                        "description": "Image format (default: jpeg)"
                    }
                },
                "required": ["session_id"]
            }),
        },
        McpTool {
            name: "mobile_get_session_stats".into(),
            description: "Retrieve real-time session statistics: FPS, bitrate, latency, bandwidth usage.".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "session_id": {
                        "type": "string",
                        "description": "Active session ID"
                    }
                },
                "required": ["session_id"]
            }),
        },
        McpTool {
            name: "mobile_list_available_peers".into(),
            description: "List all available desktop peers that can be connected to (previously paired devices).".into(),
            input_schema: json!({
                "type": "object",
                "properties": {
                    "filter_status": {
                        "type": "string",
                        "enum": ["online", "offline", "pairing", "all"],
                        "description": "Filter by peer status (default: all)"
                    }
                },
                "required": []
            }),
        },
    ]
}
