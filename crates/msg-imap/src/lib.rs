use tokio::net::TcpListener;
use tokio::io::{AsyncBufReadExt, AsyncWriteExt, BufReader};
use msg_core::{Message, ImapConfig};
use std::sync::Arc;

pub struct ImapServer {
    config: ImapConfig,
    messages: Arc<tokio::sync::Mutex<Vec<Message>>>,
}

impl ImapServer {
    pub fn new(config: ImapConfig, messages: Arc<tokio::sync::Mutex<Vec<Message>>>) -> Self {
        Self { config, messages }
    }

    pub async fn serve(&self) -> anyhow::Result<()> {
        let addr = format!("{}:{}", self.config.listen_addr, self.config.port);
        let listener = TcpListener::bind(&addr).await?;
        tracing::info!("IMAP server listening on {}", addr);

        loop {
            let (socket, _) = listener.accept().await?;
            let messages = self.messages.clone();
            tokio::spawn(async move {
                if let Err(e) = handle_imap_session(socket, messages).await {
                    tracing::error!("IMAP session error: {}", e);
                }
            });
        }
    }
}

async fn handle_imap_session(
    mut socket: tokio::net::TcpStream,
    messages: Arc<tokio::sync::Mutex<Vec<Message>>>,
) -> anyhow::Result<()> {
    let (reader, mut writer) = socket.split();
    let mut buf = BufReader::new(reader);
    writer.write_all(b"* OK IMAP4 Bonsai BMF ready\r\n").await?;

    let mut line = String::new();

    loop {
        line.clear();
        if buf.read_line(&mut line).await? == 0 { break; }
        let parts: Vec<&str> = line.trim().splitn(2, ' ').collect();
        if parts.len() < 2 { continue; }
        let tag = parts[0].to_string();
        let command = parts[1].to_uppercase();

        if command.starts_with("LOGOUT") {
            writer.write_all(format!("{} OK LOGOUT completed\r\n", tag).as_bytes()).await?;
            break;
        } else if command.starts_with("SELECT") {
            writer.write_all(format!("* 1 EXISTS\r\n* 0 RECENT\r\n{} OK SELECT completed\r\n", tag).as_bytes()).await?;
        } else if command.starts_with("FETCH") {
            let msgs = messages.lock().await;
            if !msgs.is_empty() {
                let msg = &msgs[0];
                writer.write_all(
                    format!("* 1 FETCH (BODY[] {{{}}}\r\n{}\r\n)\r\n{} OK FETCH completed\r\n",
                        msg.body.len(), msg.body, tag).as_bytes()
                ).await?;
            } else {
                writer.write_all(format!("{} OK FETCH completed\r\n", tag).as_bytes()).await?;
            }
        } else {
            writer.write_all(format!("{} OK {} completed\r\n", tag, command).as_bytes()).await?;
        }
    }
    Ok(())
}
