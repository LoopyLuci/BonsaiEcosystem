//! Bonsai CLI - Command-line interface for all systems

use clap::{Parser, Subcommand};
use std::path::PathBuf;

#[derive(Parser)]
#[command(name = "bonsai")]
#[command(about = "Bonsai Ecosystem CLI", long_about = None)]
pub struct Cli {
    #[command(subcommand)]
    pub command: Option<Commands>,
}

#[derive(Subcommand)]
pub enum Commands {
    /// Manage models
    Model {
        #[command(subcommand)]
        action: ModelAction,
    },
    /// Run inference
    Infer {
        #[arg(short, long)]
        model: String,
        #[arg(short, long)]
        prompt: String,
    },
    /// Knowledge operations
    Knowledge {
        #[command(subcommand)]
        action: KnowledgeAction,
    },
    /// System monitoring
    Monitor {
        #[arg(short, long, default_value = "10")]
        interval: u64,
    },
}

#[derive(Subcommand)]
pub enum ModelAction {
    /// List available models
    List,
    /// Pull a model
    Pull {
        #[arg(short, long)]
        name: String,
    },
    /// Push a model
    Push {
        #[arg(short, long)]
        path: PathBuf,
    },
}

#[derive(Subcommand)]
pub enum KnowledgeAction {
    /// Query knowledge base
    Query {
        #[arg(short, long)]
        subject: String,
        #[arg(short, long)]
        predicate: String,
    },
    /// Add knowledge
    Add {
        #[arg(short, long)]
        subject: String,
        #[arg(short, long)]
        predicate: String,
        #[arg(short, long)]
        object: String,
    },
}

pub async fn execute(cli: Cli) -> anyhow::Result<()> {
    match cli.command {
        Some(Commands::Model { action }) => handle_model(action).await,
        Some(Commands::Infer { model, prompt }) => handle_infer(&model, &prompt).await,
        Some(Commands::Knowledge { action }) => handle_knowledge(action).await,
        Some(Commands::Monitor { interval }) => handle_monitor(interval).await,
        None => {
            println!("Use 'bonsai --help' for usage information");
            Ok(())
        }
    }
}

async fn handle_model(action: ModelAction) -> anyhow::Result<()> {
    match action {
        ModelAction::List => println!("Available models: (stub)"),
        ModelAction::Pull { name } => println!("Pulling model: {}", name),
        ModelAction::Push { path } => println!("Pushing model from: {}", path.display()),
    }
    Ok(())
}

async fn handle_infer(model: &str, prompt: &str) -> anyhow::Result<()> {
    println!("Inference with model: {}", model);
    println!("Prompt: {}", prompt);
    Ok(())
}

async fn handle_knowledge(action: KnowledgeAction) -> anyhow::Result<()> {
    match action {
        KnowledgeAction::Query { subject, predicate } => {
            println!("Querying: {} {}", subject, predicate);
        }
        KnowledgeAction::Add {
            subject,
            predicate,
            object,
        } => {
            println!("Adding: {} {} {}", subject, predicate, object);
        }
    }
    Ok(())
}

async fn handle_monitor(interval: u64) -> anyhow::Result<()> {
    println!("Monitoring system metrics every {} seconds", interval);
    Ok(())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cli_parse() {
        let cli: Cli = Cli::parse_from(&["bonsai", "monitor", "--interval", "5"]);
        assert!(matches!(cli.command, Some(Commands::Monitor { .. })));
    }
}
