use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use msg_core::{Message, SmtpConfig, filter::SpamFilter};
use std::sync::Arc;

pub struct SmtpServer {
    config: SmtpConfig,
    filter: SpamFilter,
    messages: Arc<tokio::sync::Mutex<Vec<Message>>>,
}

impl SmtpServer {
    pub fn new(config: SmtpConfig) -> Self {
        Self {
            config,
            filter: SpamFilter,
            messages: Arc::new(tokio::sync::Mutex::new(Vec::new())),
        }
    }

    /// Start the SMTP server
    pub async fn serve(&self) -> anyhow::Result<()> {
        let addr = format!("{}:{}", self.config.listen_addr, self.config.port);
        let listener = TcpListener::bind(&addr).await?;
        tracing::info!("SMTP server listening on {}", addr);

        loop {
            let (socket, _) = listener.accept().await?;
            let messages = self.messages.clone();
            let domain = self.config.domain.clone();
            let filter = self.filter;

            tokio::spawn(async move {
                if let Err(e) = handle_smtp_session(socket, messages, &domain, filter).await {
                    tracing::error!("SMTP session error: {}", e);
                }
            });
        }
    }
}

async fn handle_smtp_session(
    mut socket: tokio::net::TcpStream,
    messages: Arc<tokio::sync::Mutex<Vec<Message>>>,
    domain: &str,
    filter: SpamFilter,
) -> anyhow::Result<()> {
    let (reader, mut writer) = socket.split();
    let mut buf = BufReader::new(reader);

    writer.write_all(format!("220 {} SMTP Bonsai BMF\r\n", domain).as_bytes()).await?;

    let mut from = String::new();
    let mut to = Vec::new();
    let mut data_lines = Vec::new();
    let mut in_data = false;

    let mut line = String::new();
    loop {
        line.clear();
        if buf.read_line(&mut line).await? == 0 { break; }
        let trimmed = line.trim().to_string();

        if trimmed.starts_with("QUIT") {
            writer.write_all(b"221 Bye\r\n").await?;
            break;
        } else if trimmed.starts_with("MAIL FROM:") {
            from = extract_email(&trimmed);
            writer.write_all(b"250 OK\r\n").await?;
        } else if trimmed.starts_with("RCPT TO:") {
            to.push(extract_email(&trimmed));
            writer.write_all(b"250 OK\r\n").await?;
        } else if trimmed == "DATA" {
            writer.write_all(b"354 Start mail input; end with <CRLF>.<CRLF>\r\n").await?;
            in_data = true;
        } else if in_data && trimmed == "." {
            in_data = false;
            let body = data_lines.join("\n");
            let msg = Message {
                id: uuid::Uuid::new_v4().to_string(),
                from: from.clone(),
                to: to.clone(),
                cc: vec![],
                subject: extract_subject(&data_lines),
                body,
                body_type: msg_core::BodyType::Plain,
                attachments: vec![],
                in_reply_to: None,
                references: vec![],
                date: chrono::Utc::now().to_rfc2822(),
                headers: std::collections::HashMap::new(),
                encrypted: false,
                signature: None,
            };

            if filter.is_spam(&msg, 0.7) {
                tracing::info!("Message from {} classified as spam", msg.from);
            } else {
                messages.lock().await.push(msg);
            }

            writer.write_all(b"250 OK\r\n").await?;
            data_lines.clear();
            from.clear();
            to.clear();
        } else if in_data {
            data_lines.push(trimmed);
        } else {
            writer.write_all(b"250 OK\r\n").await?;
        }
    }
    Ok(())
}

fn extract_email(s: &str) -> String {
    s.split(':').nth(1)
        .unwrap_or("")
        .trim()
        .trim_matches('<')
        .trim_matches('>')
        .to_string()
}

fn extract_subject(lines: &[String]) -> String {
    for line in lines {
        if line.to_lowercase().starts_with("subject:") {
            return line[8..].trim().to_string();
        }
    }
    "(no subject)".to_string()
}
