pub mod encryption;
pub mod filter;

use serde::{Serialize, Deserialize};

/// A complete email message
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Message {
    pub id: String,
    pub from: String,
    pub to: Vec<String>,
    pub cc: Vec<String>,
    pub subject: String,
    pub body: String,
    pub body_type: BodyType,
    pub attachments: Vec<Attachment>,
    pub in_reply_to: Option<String>,
    pub references: Vec<String>,
    pub date: String,
    pub headers: std::collections::HashMap<String, String>,
    pub encrypted: bool,
    pub signature: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BodyType {
    Plain,
    Html,
    Multipart,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Attachment {
    pub filename: String,
    pub content_type: String,
    pub data: Vec<u8>,
    pub size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SmtpConfig {
    pub listen_addr: String,
    pub port: u16,
    pub domain: String,
    pub max_message_size: usize,
}

impl Default for SmtpConfig {
    fn default() -> Self {
        Self {
            listen_addr: "0.0.0.0".into(),
            port: 25,
            domain: "bonsai.local".into(),
            max_message_size: 25 * 1024 * 1024,
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ImapConfig {
    pub listen_addr: String,
    pub port: u16,
    pub tls: bool,
}

impl Default for ImapConfig {
    fn default() -> Self {
        Self {
            listen_addr: "0.0.0.0".into(),
            port: 143,
            tls: false,
        }
    }
}
