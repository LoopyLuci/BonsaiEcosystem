#![cfg(feature = "discord")]

use async_trait::async_trait;
use serenity::async_trait as serenity_async_trait;
use serenity::model::application::Interaction;
use serenity::model::channel::Message;
use serenity::model::gateway::{GatewayIntents, Ready};
use serenity::prelude::*;
use std::sync::Arc;

use crate::admin_api::PlatformStates;
use crate::config::DiscordConfig;
use crate::metrics::SharedMetrics;
use crate::platforms::{InboundMessage, MessagingPlatform, ShedNotice};
use crate::router::Router;
use crate::session;

pub struct DiscordPlatform {
    token: String,
    config: DiscordConfig,
    metrics: SharedMetrics,
    router: Arc<Router>,
    platform_states: PlatformStates,
}

impl DiscordPlatform {
    pub fn new(
        token: String,
        config: DiscordConfig,
        metrics: SharedMetrics,
        router: Arc<Router>,
        platform_states: PlatformStates,
    ) -> Arc<Self> {
        Arc::new(Self {
            token,
            config,
            metrics,
            router,
            platform_states,
        })
    }
}

struct Handler {
    platform: Arc<DiscordPlatform>,
    tx: tokio::sync::mpsc::Sender<InboundMessage>,
    shed_tx: tokio::sync::mpsc::Sender<ShedNotice>,
}

#[serenity_async_trait]
impl EventHandler for Handler {
    async fn message(&self, _ctx: Context, msg: Message) {
        if msg.author.bot {
            return;
        }

        let cfg = &self.platform.config;
        let guild_id = msg.guild_id.map(|g| g.to_string()).unwrap_or_default();

        if !cfg.allowed_guild_ids.is_empty() && !cfg.allowed_guild_ids.contains(&guild_id) {
            return;
        }
        let channel_id = msg.channel_id.to_string();
        if !cfg.allowed_channel_ids.is_empty() && !cfg.allowed_channel_ids.contains(&channel_id) {
            return;
        }
        let user_id = msg.author.id.to_string();
        if !cfg.allowed_user_ids.is_empty() && !cfg.allowed_user_ids.contains(&user_id) {
            self.platform
                .metrics
                .allowlist_denials
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            return;
        }

        let inbound = InboundMessage {
            platform: "discord".to_string(),
            platform_id: channel_id.clone(),
            user_id: user_id.clone(),
            display_name: msg.author.name.clone(),
            event_id: msg.id.to_string(),
            text: msg.content.clone(),
            reply_to: None,
        };

        self.platform
            .metrics
            .messages_inbound
            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

        if self.tx.try_send(inbound).is_err() {
            self.platform
                .metrics
                .messages_queued_full
                .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            let _ = self.shed_tx.try_send(ShedNotice {
                platform: "discord".to_string(),
                chat_id: channel_id,
                user_id,
                reply_to: None,
            });
        }
    }

    async fn interaction_create(&self, ctx: Context, interaction: Interaction) {
        if let Interaction::Command(command) = &interaction {
            use serenity::builder::{CreateInteractionResponse, CreateInteractionResponseMessage};
            use serenity::model::application::CommandDataOptionValue;

            let cmd_name = command.data.name.as_str();

            // ACK within 3s — deferred so we have time for Buddy calls
            let ack = tokio::time::timeout(
                tokio::time::Duration::from_secs(2),
                command.create_response(
                    &ctx.http,
                    CreateInteractionResponse::Defer(
                        CreateInteractionResponseMessage::new().ephemeral(false),
                    ),
                ),
            )
            .await;
            if ack.is_err() || ack.unwrap().is_err() {
                tracing::warn!("[discord] slash command ACK timed out");
                return;
            }

            let reply_text = match cmd_name {
                "help" => "**Bonsai commands**\n\
                     `/ask <question>` — Ask Bonsai anything\n\
                     `/status` — Show health status\n\
                     `/help` — This message\n\n\
                     You can also DM or mention me directly in any allowed channel."
                    .to_string(),
                "status" => {
                    // Call the bot's own health endpoint via BuddyClient workspace URL
                    let url = format!(
                        "{}/health/full",
                        self.platform.router.buddy.workspace_api_url()
                    );
                    match reqwest::Client::new()
                        .get(&url)
                        .timeout(std::time::Duration::from_secs(5))
                        .send()
                        .await
                    {
                        Ok(resp) if resp.status().is_success() => {
                            match resp.json::<serde_json::Value>().await {
                                Ok(v) => {
                                    let status = v["status"].as_str().unwrap_or("unknown");
                                    let emoji = if status == "healthy" {
                                        "🟢"
                                    } else if status == "degraded" {
                                        "🟡"
                                    } else {
                                        "🔴"
                                    };
                                    format!("{emoji} **Bonsai status**: {status}")
                                }
                                Err(_) => "⚠️ Could not parse health response.".to_string(),
                            }
                        }
                        _ => "⚠️ Health endpoint unreachable.".to_string(),
                    }
                }
                "ask" => {
                    let question = command
                        .data
                        .options
                        .iter()
                        .find(|o| o.name == "question")
                        .and_then(|o| {
                            if let CommandDataOptionValue::String(s) = &o.value {
                                Some(s.clone())
                            } else {
                                None
                            }
                        })
                        .unwrap_or_default();

                    if question.is_empty() {
                        "Please provide a question.".to_string()
                    } else {
                        let user_id = command.user.id.to_string();
                        let platform_id = command.channel_id.to_string();
                        let event_id = format!("slash-{}", command.id);

                        let inbound = crate::platforms::InboundMessage {
                            platform: "discord".to_string(),
                            platform_id: platform_id.clone(),
                            user_id: user_id.clone(),
                            display_name: command.user.name.clone(),
                            event_id,
                            text: question,
                            reply_to: None,
                        };

                        self.platform
                            .metrics
                            .messages_inbound
                            .fetch_add(1, std::sync::atomic::Ordering::Relaxed);

                        // Use a one-shot channel to capture the reply
                        let (reply_tx, mut reply_rx) = tokio::sync::mpsc::channel::<String>(1);
                        let router = self.platform.router.clone();
                        let plat =
                            self.platform.clone() as Arc<dyn crate::platforms::MessagingPlatform>;
                        let plat_capture: Arc<dyn crate::platforms::MessagingPlatform> =
                            Arc::new(SlashReplyCapture {
                                inner: plat,
                                tx: reply_tx,
                            });

                        tokio::time::timeout(tokio::time::Duration::from_secs(28), async move {
                            router.handle(inbound, &plat_capture).await
                        })
                        .await
                        .ok();

                        reply_rx
                            .try_recv()
                            .unwrap_or_else(|_| "(no response)".to_string())
                    }
                }
                _ => return,
            };

            let follow_up = serenity::builder::EditInteractionResponse::new().content(&reply_text);
            if let Err(e) = command.edit_response(&ctx.http, follow_up).await {
                tracing::warn!("[discord] slash follow-up failed: {e}");
            }
            return;
        }

        if let Interaction::Component(component) = interaction {
            let custom_id = &component.data.custom_id;

            // Parse: "confirm_approve:{token}:{nonce}" or "confirm_deny:{token}:{nonce}"
            let (approved, token, nonce) =
                if let Some(rest) = custom_id.strip_prefix("confirm_approve:") {
                    let (tok, n) = split_token_nonce(rest);
                    (true, tok, n)
                } else if let Some(rest) = custom_id.strip_prefix("confirm_deny:") {
                    let (tok, n) = split_token_nonce(rest);
                    (false, tok, n)
                } else {
                    return;
                };

            // Validate nonce against stored nonce to reject stale interactions
            let db = &self.platform.router.db;
            let pending = session::load_unresolved_confirms(db).await;
            let stored_nonce = pending
                .iter()
                .find(|p| p.token == token)
                .map(|p| p.prompt_nonce);

            if stored_nonce != Some(nonce) {
                // Stale interaction — acknowledge silently without executing
                let _ = component
                    .create_response(
                        &ctx.http,
                        serenity::builder::CreateInteractionResponse::Acknowledge,
                    )
                    .await;
                return;
            }

            // Acknowledge within Discord's 3-second window
            let ack_result = tokio::time::timeout(
                tokio::time::Duration::from_secs(2),
                component.create_response(
                    &ctx.http,
                    serenity::builder::CreateInteractionResponse::Acknowledge,
                ),
            )
            .await;
            if ack_result.is_err() {
                tracing::warn!("[discord] interaction ACK timed out — skipping follow-up");
                return;
            }

            // Resolve in our DB
            let _ = session::resolve_confirm(db, token.clone()).await;
            if approved {
                self.platform
                    .router
                    .metrics
                    .confirms_resolved
                    .fetch_add(1, std::sync::atomic::Ordering::Relaxed);
            }

            // Send confirm_response to Buddy and get the reply (bounded to 25s follow-up window)
            let reply = tokio::time::timeout(
                tokio::time::Duration::from_secs(25),
                self.platform.router.send_confirm_response(&token, approved),
            )
            .await
            .unwrap_or_else(|_| Ok("⏱️ Response timed out. Please check back later.".to_string()))
            .unwrap_or_else(|_| {
                if approved {
                    "✅ Confirmed. Processing...".to_string()
                } else {
                    "❌ Denied. No action taken.".to_string()
                }
            });

            // Send follow-up message
            let channel_id_str = component.channel_id.to_string();
            let user_id_str = component
                .member
                .as_ref()
                .map(|m| m.user.id.to_string())
                .unwrap_or_default();
            let _ =
                send_discord_message(&self.platform.token, &channel_id_str, &user_id_str, &reply)
                    .await;
        }
    }

    async fn ready(&self, ctx: Context, ready: Ready) {
        tracing::info!("[discord] Connected as {}", ready.user.name);
        self.platform
            .platform_states
            .insert("discord".to_string(), "connected".to_string());

        // Register global slash commands
        use serenity::builder::{CreateCommand, CreateCommandOption};
        use serenity::model::application::CommandOptionType;

        let commands = vec![
            CreateCommand::new("ask")
                .description("Ask Bonsai a question")
                .add_option(
                    CreateCommandOption::new(
                        CommandOptionType::String,
                        "question",
                        "Your question",
                    )
                    .required(true),
                ),
            CreateCommand::new("status").description("Show Bonsai health status"),
            CreateCommand::new("help").description("List available commands and capabilities"),
        ];

        if let Err(e) =
            serenity::model::application::Command::set_global_commands(&ctx.http, commands).await
        {
            tracing::error!("[discord] Failed to register slash commands: {e}");
        } else {
            tracing::info!("[discord] Slash commands registered (/ask, /status, /help)");
        }
    }
}

fn split_token_nonce(s: &str) -> (String, i64) {
    // Format: "{token}:{nonce}" where nonce is the last colon-separated segment
    if let Some(pos) = s.rfind(':') {
        let token = s[..pos].to_string();
        let nonce = s[pos + 1..].parse::<i64>().unwrap_or(-1);
        (token, nonce)
    } else {
        (s.to_string(), -1)
    }
}

#[async_trait]
impl MessagingPlatform for DiscordPlatform {
    fn name(&self) -> &'static str {
        "discord"
    }

    async fn run(
        self: Arc<Self>,
        tx: tokio::sync::mpsc::Sender<InboundMessage>,
        shed_tx: tokio::sync::mpsc::Sender<ShedNotice>,
    ) {
        self.platform_states
            .insert("discord".to_string(), "connecting".to_string());

        let intents = GatewayIntents::GUILD_MESSAGES
            | GatewayIntents::DIRECT_MESSAGES
            | GatewayIntents::MESSAGE_CONTENT
            | GatewayIntents::GUILD_MESSAGE_REACTIONS;

        let handler = Handler {
            platform: self.clone(),
            tx,
            shed_tx,
        };

        match Client::builder(&self.token, intents)
            .event_handler(handler)
            .await
        {
            Err(e) => tracing::error!("[discord] Client build error: {e}"),
            Ok(mut client) => {
                if let Err(e) = client.start().await {
                    tracing::error!("[discord] Gateway error: {e}");
                }
            }
        }
    }

    async fn send_reply(
        &self,
        chat_id: &str,
        user_id: &str,
        text: &str,
        _reply_to: Option<&str>,
    ) -> Result<(), String> {
        send_discord_message(&self.token, chat_id, user_id, text).await
    }

    async fn send_confirm_prompt(
        &self,
        chat_id: &str,
        _user_id: &str,
        token: &str,
        prompt: &str,
        nonce: i64,
    ) -> Result<String, String> {
        use serenity::builder::{CreateActionRow, CreateButton, CreateMessage};
        use serenity::http::Http;
        use serenity::model::application::ButtonStyle;
        use serenity::model::id::ChannelId;

        let http = Http::new(&self.token);
        let channel_id: u64 = chat_id.parse().map_err(|e| format!("channel id: {e}"))?;

        let approve_id = format!("confirm_approve:{token}:{nonce}");
        let deny_id = format!("confirm_deny:{token}:{nonce}");

        let msg = CreateMessage::new()
            .content(format!("⚠️ **Confirmation required**\n{prompt}"))
            .components(vec![CreateActionRow::Buttons(vec![
                CreateButton::new(&approve_id)
                    .label("✅ Approve")
                    .style(ButtonStyle::Success),
                CreateButton::new(&deny_id)
                    .label("❌ Deny")
                    .style(ButtonStyle::Danger),
            ])]);

        let sent = ChannelId::new(channel_id)
            .send_message(&http, msg)
            .await
            .map_err(|e| format!("discord confirm: {e}"))?;

        Ok(sent.id.to_string())
    }
}

/// Intercepts send_reply to capture the slash-command response text.
struct SlashReplyCapture {
    inner: Arc<dyn MessagingPlatform>,
    tx: tokio::sync::mpsc::Sender<String>,
}

#[async_trait]
impl MessagingPlatform for SlashReplyCapture {
    fn name(&self) -> &'static str {
        "discord"
    }

    async fn run(
        self: Arc<Self>,
        _tx: tokio::sync::mpsc::Sender<InboundMessage>,
        _shed: tokio::sync::mpsc::Sender<ShedNotice>,
    ) {
    }

    async fn send_reply(
        &self,
        _chat_id: &str,
        _user_id: &str,
        text: &str,
        _reply_to: Option<&str>,
    ) -> Result<(), String> {
        let _ = self.tx.try_send(text.to_string());
        Ok(())
    }

    async fn send_confirm_prompt(
        &self,
        chat_id: &str,
        user_id: &str,
        token: &str,
        prompt: &str,
        nonce: i64,
    ) -> Result<String, String> {
        self.inner
            .send_confirm_prompt(chat_id, user_id, token, prompt, nonce)
            .await
    }
}

async fn send_discord_message(
    token: &str,
    chat_id: &str,
    _user_id: &str,
    text: &str,
) -> Result<(), String> {
    use serenity::http::Http;
    use serenity::model::id::ChannelId;

    let http = Http::new(token);
    let channel_id: u64 = chat_id.parse().map_err(|e| format!("channel id: {e}"))?;
    let cid = ChannelId::new(channel_id);

    for chunk in crate::formatter::format(text, "discord").chunks {
        cid.say(&http, &chunk)
            .await
            .map_err(|e| format!("discord send: {e}"))?;
    }
    Ok(())
}
