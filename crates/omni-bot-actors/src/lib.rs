//! Omni-Bot Aether Actor System
//!
//! A distributed actor framework for message-driven orchestration of the Omni-Bot control system.
//!
//! # Overview
//!
//! The Aether actor system implements the Erlang/Akka pattern with async/await and Rust's type system.
//! Each actor:
//! - Runs in its own Tokio task
//! - Receives messages through an unbounded channel
//! - Processes messages sequentially (single-threaded per actor)
//! - Can spawn new actors
//! - Maintains mutable state
//! - Supports state snapshots for persistence
//!
//! # Core Actors
//!
//! - **ChatGateway**: Message normalization from multiple platforms (Slack, Discord, Teams, etc.)
//! - **CommandParser**: Converts natural language and structured commands to Actions
//! - **ActionExecutor**: Executes Actions with error handling, retries, and timeouts
//! - **SessionManager**: Per-user session state and context management
//! - **PolicyEnforcer**: Capability validation and rate limiting
//! - **MonitoringAgent**: System health monitoring and alerting
//!
//! # Example
//!
//! ```ignore
//! use omni_bot_actors::actor::spawn_actor;
//! use omni_bot_actors::chat_gateway::ChatGateway;
//!
//! #[tokio::main]
//! async fn main() {
//!     let gateway = ChatGateway::new(100);
//!     let handle = spawn_actor(gateway);
//!
//!     // Send messages to the actor
//!     let ref_ = handle.actor_ref();
//!     ref_.send(ChatGatewayMessage::IncomingMessage {
//!         platform: Platform::Slack,
//!         raw: serde_json::json!({ }),
//!     }).ok();
//!
//!     // Wait for actor to stop
//!     handle.join().await;
//! }
//! ```

pub mod actor;
pub mod chat_gateway;
pub mod command_parser;
pub mod action_executor;
pub mod session_manager;
pub mod policy_enforcer;
pub mod monitoring;

pub use actor::{Actor, ActorId, ActorRef, ActorHandle, Snapshot, SupervisionEvent, spawn_actor};
pub use chat_gateway::{ChatGateway, ChatGatewayMessage, NormalizedMessage, Platform, ChatMetrics};
pub use command_parser::{CommandParser, CommandParserMessage, Intent, ParsedCommand, ParsingMetrics};
pub use action_executor::{ActionExecutor, ActionExecutorMessage, ExecutionResult, ExecutionRequest, ExecutionStatus, RetryStrategy, ExecutionMetrics};
pub use session_manager::{SessionManager, SessionManagerMessage, SessionState, SessionMetrics};
pub use policy_enforcer::{PolicyEnforcer, PolicyEnforcerMessage, PolicyViolation, ViolationType, ViolationSeverity, PolicyMetrics};
pub use monitoring::{MonitoringAgent, MonitoringAgentMessage, SystemMetrics, HealthReport, HealthStatus, Alert, AlertSeverity, MonitoringMetrics};

pub const VERSION: &str = env!("CARGO_PKG_VERSION");

/// Initialize the Aether actor system
pub fn init() {
    log::info!("Omni-Bot Aether Actor System v{} initialized", VERSION);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_version() {
        assert!(!VERSION.is_empty());
    }

    #[test]
    fn test_module_exports() {
        // Ensure all public types are accessible
        let _ = ChatGateway::new(100);
        let _ = CommandParser::new();
        let _ = ActionExecutor::new();
        let _ = SessionManager::new();
        let _ = PolicyEnforcer::new();
        let _ = MonitoringAgent::new();
    }
}
