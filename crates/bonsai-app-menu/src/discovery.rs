use serde::{Serialize, Deserialize};
use std::path::PathBuf;
use crate::error::Result;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BonsaiApp {
    pub id: String,
    pub name: String,
    pub description: String,
    pub icon: String,  // emoji
    pub category: AppCategory,
    pub executable: Option<PathBuf>,
    pub launch_command: Option<String>,
    pub is_installed: bool,
    pub is_running: bool,
    pub port: Option<u16>,
    pub version: String,
}

#[derive(Debug, Clone, Copy, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum AppCategory {
    #[serde(rename = "ai")]
    AI,
    #[serde(rename = "infrastructure")]
    Infrastructure,
    #[serde(rename = "media")]
    Media,
    #[serde(rename = "development")]
    Development,
    #[serde(rename = "knowledge")]
    Knowledge,
    #[serde(rename = "security")]
    Security,
    #[serde(rename = "communication")]
    Communication,
    #[serde(rename = "utility")]
    Utility,
}

impl AppCategory {
    pub fn color_hex(&self) -> &str {
        match self {
            AppCategory::AI => "#6C5CE7",
            AppCategory::Infrastructure => "#00B894",
            AppCategory::Media => "#E17055",
            AppCategory::Development => "#0984E3",
            AppCategory::Knowledge => "#FDCB6E",
            AppCategory::Security => "#D63031",
            AppCategory::Communication => "#00CEC9",
            AppCategory::Utility => "#636E72",
        }
    }

    pub fn emoji(&self) -> &str {
        match self {
            AppCategory::AI => "🧠",
            AppCategory::Infrastructure => "📦",
            AppCategory::Media => "🎥",
            AppCategory::Development => "🔧",
            AppCategory::Knowledge => "📚",
            AppCategory::Security => "🛡️",
            AppCategory::Communication => "🤖",
            AppCategory::Utility => "📱",
        }
    }
}

pub fn discover_apps() -> Result<Vec<BonsaiApp>> {
    let mut apps = vec![
        BonsaiApp {
            id: "bonsai-workspace".into(),
            name: "Bonsai Workspace".into(),
            description: "AI-powered code editor and assistant".into(),
            icon: "🧠".into(),
            category: AppCategory::AI,
            executable: find_executable("bonsai-workspace"),
            launch_command: Some("bonsai-workspace".into()),
            is_installed: is_installed("bonsai-workspace"),
            is_running: is_port_open(1420),
            port: Some(1420),
            version: "0.2.0".into(),
        },
        BonsaiApp {
            id: "bonsai-model-workshop".into(),
            name: "Model Workshop".into(),
            description: "Design, build, edit & convert AI models".into(),
            icon: "🧬".into(),
            category: AppCategory::AI,
            executable: find_executable("bonsai-model-workshop"),
            launch_command: Some("bonsai-model-workshop".into()),
            is_installed: is_installed("bonsai-model-workshop"),
            is_running: is_port_open(4200),
            port: Some(4200),
            version: "0.1.0".into(),
        },
        BonsaiApp {
            id: "bonsai-kdb".into(),
            name: "Knowledge Base".into(),
            description: "Search your knowledge modules".into(),
            icon: "📚".into(),
            category: AppCategory::Knowledge,
            executable: find_executable("bonsai-kdb"),
            launch_command: Some("bonsai kdb serve".into()),
            is_installed: is_installed("bonsai-kdb"),
            is_running: is_port_open(8089),
            port: Some(8089),
            version: "0.1.0".into(),
        },
        BonsaiApp {
            id: "bonsai-mcp".into(),
            name: "MCP Server".into(),
            description: "AI agent tools & bridge".into(),
            icon: "🔌".into(),
            category: AppCategory::Infrastructure,
            executable: find_executable("mcp-server"),
            launch_command: Some("mcp-server --port 11426".into()),
            is_installed: is_installed("mcp-server"),
            is_running: is_port_open(11426),
            port: Some(11426),
            version: "0.1.0".into(),
        },
        BonsaiApp {
            id: "bonsai-mcp-manager".into(),
            name: "MCP Manager".into(),
            description: "Manage MCP servers, clients & tools".into(),
            icon: "⚙️".into(),
            category: AppCategory::Infrastructure,
            executable: find_executable("bonsai-mcp-manager"),
            launch_command: Some("bonsai-mcp-manager".into()),
            is_installed: is_installed("bonsai-mcp-manager"),
            is_running: is_port_open(4201),
            port: Some(4201),
            version: "0.1.0".into(),
        },
        BonsaiApp {
            id: "bonsai-android-bridge".into(),
            name: "Android Bridge".into(),
            description: "Control Android devices".into(),
            icon: "📱".into(),
            category: AppCategory::Utility,
            executable: find_executable("bonsai-android-bridge"),
            launch_command: Some("bonsai-android-bridge serve".into()),
            is_installed: is_installed("bonsai-android-bridge"),
            is_running: is_port_open(8090),
            port: Some(8090),
            version: "0.1.0".into(),
        },
        BonsaiApp {
            id: "bonsai-bug-hunter".into(),
            name: "Bug Hunter".into(),
            description: "Code sweeper & security scanner".into(),
            icon: "🛡️".into(),
            category: AppCategory::Security,
            executable: find_executable("bonsai-cli"),
            launch_command: Some("bonsai sweep --dashboard".into()),
            is_installed: is_installed("bonsai-cli"),
            is_running: false,
            port: None,
            version: "0.1.0".into(),
        },
        BonsaiApp {
            id: "bonsai-devkit".into(),
            name: "DevKit".into(),
            description: "Build, test, deploy tools".into(),
            icon: "🔧".into(),
            category: AppCategory::Development,
            executable: find_executable("bonsai-cli"),
            launch_command: Some("bonsai-cli".into()),
            is_installed: is_installed("bonsai-cli"),
            is_running: false,
            port: None,
            version: "0.1.0".into(),
        },
    ];

    apps.sort_by(|a, b| a.name.cmp(&b.name));
    Ok(apps)
}

fn find_executable(name: &str) -> Option<PathBuf> {
    let candidates = vec![
        format!("./target/release/{}.exe", name),
        format!("./target/release/{}", name),
        format!("{}/target/release/{}.exe",
            std::env::current_dir().unwrap_or_default().display(), name),
    ];

    for candidate in candidates {
        let path = PathBuf::from(&candidate);
        if path.exists() {
            return Some(path);
        }
    }
    None
}

fn is_installed(_name: &str) -> bool {
    true  // Simplified; check registry or package manager in production
}

fn is_port_open(port: u16) -> bool {
    std::net::TcpStream::connect(format!("127.0.0.1:{}", port)).is_ok()
}
