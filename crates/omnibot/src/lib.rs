// Bonsai OmniBot — Unified Telegram & Discord control for Bonsai Ecosystem

pub mod platform;
pub mod command;
pub mod user;
pub mod permission;
pub mod nlu;
pub mod mcp;
pub mod session;
pub mod event;

pub use platform::{PlatformAdapter, Platform, Message, MessageId};
pub use command::{Command, CommandRegistry, CommandContext, CommandResponse};
pub use user::{User, UserId, UserRole};
pub use permission::{CapabilityToken, Capability, Permission};
pub use nlu::{Intent, IntentClassifier};
pub use mcp::McpClient;
pub use session::{SessionManager, Session};
pub use event::Event;

use anyhow::Result;
use std::sync::Arc;
use tokio::sync::RwLock;
use dashmap::DashMap;

/// Core OmniBot instance
pub struct OmniBot {
    adapters: DashMap<Platform, Arc<dyn PlatformAdapter>>,
    commands: Arc<CommandRegistry>,
    users: Arc<RwLock<DashMap<UserId, User>>>,
    sessions: Arc<SessionManager>,
    mcp_client: Arc<McpClient>,
    nlu: Arc<IntentClassifier>,
}

impl OmniBot {
    pub async fn new(mcp_url: String) -> Result<Self> {
        Ok(Self {
            adapters: DashMap::new(),
            commands: Arc::new(CommandRegistry::new()),
            users: Arc::new(RwLock::new(DashMap::new())),
            sessions: Arc::new(SessionManager::new()),
            mcp_client: Arc::new(McpClient::new(mcp_url).await?),
            nlu: Arc::new(IntentClassifier::new()),
        })
    }

    pub async fn register_adapter(&self, platform: Platform, adapter: Arc<dyn PlatformAdapter>) {
        self.adapters.insert(platform, adapter);
    }

    pub async fn process_message(&self, message: Message) -> Result<()> {
        // Look up user
        let user = self.get_or_create_user(&message.user_id, &message.platform).await?;

        // Try to parse as command
        if let Some(cmd_name) = message.text.strip_prefix('/') {
            self.handle_command(cmd_name, user.clone(), message).await?;
        } else {
            // Use NLU to classify intent
            self.handle_natural_language(&message.text, user.clone(), message).await?;
        }

        Ok(())
    }

    async fn handle_command(&self, cmd_name: &str, user: User, message: Message) -> Result<()> {
        if let Some(cmd) = self.commands.get(cmd_name) {
            // Check permissions
            if !user.has_capability(&cmd.required_capability()) {
                self.send_reply(&message, "❌ Permission denied").await?;
                return Ok(());
            }

            // Execute command
            let ctx = CommandContext {
                user,
                message,
                mcp_client: self.mcp_client.clone(),
                session: self.sessions.get_or_create(&message.user_id).await,
            };

            let response = cmd.execute(ctx).await?;
            self.send_reply(&message, &response.text).await?;
        } else {
            self.send_reply(&message, "❓ Unknown command. Type `/help` for available commands.").await?;
        }

        Ok(())
    }

    async fn handle_natural_language(&self, text: &str, user: User, message: Message) -> Result<()> {
        // Classify intent using BonsAI V2
        match self.nlu.classify(text).await {
            Ok(intent) if intent.confidence > 0.8 => {
                // High confidence – execute
                let response = self.execute_intent(&intent, user, &message).await?;
                self.send_reply(&message, &response).await?;
            }
            Ok(intent) => {
                // Low confidence – ask for clarification
                self.send_reply(&message, &format!(
                    "I think you mean: {}. Confirm? (yes/no)",
                    intent.description()
                )).await?;
            }
            Err(_) => {
                self.send_reply(&message, "I didn't understand that. Try `/help` for commands.").await?;
            }
        }

        Ok(())
    }

    async fn execute_intent(&self, intent: &Intent, user: User, _message: &Message) -> Result<String> {
        // Permission check
        if !user.has_capability(&intent.required_capability()) {
            return Ok("❌ Permission denied".into());
        }

        // Call MCP tool
        self.mcp_client.call_tool(&intent.tool_name, &intent.parameters).await
    }

    async fn get_or_create_user(&self, user_id: &UserId, platform: &Platform) -> Result<User> {
        if let Some(user) = self.users.read().await.get(user_id) {
            return Ok(user.clone());
        }

        // Create new user with default permissions
        let user = User::new(user_id.clone(), UserRole::Viewer, platform.clone());
        self.users.write().await.insert(user_id.clone(), user.clone());
        Ok(user)
    }

    async fn send_reply(&self, message: &Message, text: &str) -> Result<()> {
        if let Some(adapter) = self.adapters.get(&message.platform) {
            adapter.send_message(&message.user_id, text).await?;
        }
        Ok(())
    }

    pub fn command_registry(&self) -> Arc<CommandRegistry> {
        self.commands.clone()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_omnibot_creation() {
        // Note: actual creation requires async context and MCP server
        // This is a placeholder
    }
}
