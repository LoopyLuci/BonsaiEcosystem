use std::collections::HashMap;
use std::sync::Arc;
use serde::{Deserialize, Serialize};
use tokio::sync::RwLock;

/// Mirrors the subset of SystemEvent variants the bot rule engine reacts to.
/// Kept local to avoid a hard dep on the Tauri crate from omni-bot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum BotEvent {
    BuildFailed { crate_name: String, error: String },
    TestFailed { suite: String, failures: Vec<String> },
    CrashDetected { component: String, backtrace: String },
    TrainingComplete { phase: String, adapter_path: String },
    UpgradeReady { component: String, version: String, cas_hash: String },
    PRMerged { repo: String, pr_number: u64, commit_sha: String },
    IssueCreated { issue_id: String, title: String, labels: Vec<String> },
    ComponentHealthDegraded { component: String, health_score: f64 },
    Custom { name: String, payload: serde_json::Value },
}

impl BotEvent {
    pub fn type_name(&self) -> &'static str {
        match self {
            BotEvent::BuildFailed { .. } => "BuildFailed",
            BotEvent::TestFailed { .. } => "TestFailed",
            BotEvent::CrashDetected { .. } => "CrashDetected",
            BotEvent::TrainingComplete { .. } => "TrainingComplete",
            BotEvent::UpgradeReady { .. } => "UpgradeReady",
            BotEvent::PRMerged { .. } => "PRMerged",
            BotEvent::IssueCreated { .. } => "IssueCreated",
            BotEvent::ComponentHealthDegraded { .. } => "ComponentHealthDegraded",
            BotEvent::Custom { .. } => "Custom",
        }
    }
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EventPattern {
    pub event_type: String,
    pub filters: HashMap<String, String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Trigger {
    Event(EventPattern),
    Cron(String),
    Manual,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum RuleAction {
    SpawnSwarm { template: String, params: HashMap<String, String> },
    RunCi { workspace_root: String },
    HotReloadModel { adapter_path: String },
    RollbackComponent { component: String },
    CreateIssue { title: String, body: String, labels: Vec<String> },
    NotifyUser { message: String, level: String },
    ExecuteCommand { program: String, args: Vec<String> },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BotRule {
    pub name: String,
    pub trigger: Trigger,
    pub action: RuleAction,
    pub enabled: bool,
    pub cooldown_secs: u64,
}

#[derive(Default)]
struct RuleState {
    last_fired: HashMap<String, std::time::Instant>,
}

/// Callback invoked when a rule wants to spawn a swarm.
/// Signature: (template_name, params_json) -> ()
pub type SpawnSwarmFn = Arc<dyn Fn(String, serde_json::Value) + Send + Sync + 'static>;

pub struct BotRuleEngine {
    rules: Arc<RwLock<Vec<BotRule>>>,
    state: Arc<RwLock<RuleState>>,
    event_tx: tokio::sync::broadcast::Sender<BotEvent>,
    spawn_swarm: Option<SpawnSwarmFn>,
}

impl BotRuleEngine {
    pub fn new() -> Self {
        let (event_tx, _) = tokio::sync::broadcast::channel(256);
        Self {
            rules: Arc::new(RwLock::new(Vec::new())),
            state: Arc::new(RwLock::new(RuleState::default())),
            event_tx,
            spawn_swarm: None,
        }
    }

    /// Wire a real swarm spawner (called from the Tauri app that owns SwarmRegistry).
    pub fn with_spawn_swarm(mut self, f: SpawnSwarmFn) -> Self {
        self.spawn_swarm = Some(f);
        self
    }

    pub async fn load_default_rules(&self) {
        let mut rules = self.rules.write().await;
        *rules = default_rules();
    }

    pub fn event_sender(&self) -> tokio::sync::broadcast::Sender<BotEvent> {
        self.event_tx.clone()
    }

    pub fn publish(&self, event: BotEvent) {
        let _ = self.event_tx.send(event);
    }

    pub fn start(self: Arc<Self>) {
        let engine = self.clone();
        tokio::spawn(async move {
            let mut rx = engine.event_tx.subscribe();
            while let Ok(event) = rx.recv().await {
                let rules = engine.rules.read().await.clone();
                let spawn_fn = engine.spawn_swarm.clone();
                for rule in &rules {
                    if !rule.enabled {
                        continue;
                    }
                    if !Self::matches_trigger(&rule.trigger, &event) {
                        continue;
                    }
                    // Cooldown check
                    {
                        let state = engine.state.read().await;
                        if let Some(last) = state.last_fired.get(&rule.name) {
                            if last.elapsed().as_secs() < rule.cooldown_secs {
                                tracing::debug!("Rule '{}' skipped (cooldown)", rule.name);
                                continue;
                            }
                        }
                    }
                    // Update last fired
                    {
                        let mut state = engine.state.write().await;
                        state.last_fired.insert(rule.name.clone(), std::time::Instant::now());
                    }
                    tracing::info!("BotRule '{}' firing", rule.name);
                    Self::execute_action(&rule.action, &event, spawn_fn.as_ref()).await;
                }
            }
        });
    }

    fn matches_trigger(trigger: &Trigger, event: &BotEvent) -> bool {
        match trigger {
            Trigger::Event(pattern) => {
                if event.type_name() != pattern.event_type {
                    return false;
                }
                // Additional field filters (all must match)
                for (key, value) in &pattern.filters {
                    let event_val = Self::extract_field(event, key);
                    if event_val.as_deref() != Some(value.as_str()) {
                        return false;
                    }
                }
                true
            }
            Trigger::Manual | Trigger::Cron(_) => false,
        }
    }

    fn extract_field(event: &BotEvent, key: &str) -> Option<String> {
        match (event, key) {
            (BotEvent::BuildFailed { crate_name, .. }, "crate_name") => Some(crate_name.clone()),
            (BotEvent::TestFailed { suite, .. }, "suite") => Some(suite.clone()),
            (BotEvent::CrashDetected { component, .. }, "component") => Some(component.clone()),
            (BotEvent::TrainingComplete { phase, .. }, "phase") => Some(phase.clone()),
            (BotEvent::UpgradeReady { component, .. }, "component") => Some(component.clone()),
            (BotEvent::PRMerged { repo, .. }, "repo") => Some(repo.clone()),
            _ => None,
        }
    }

    async fn execute_action(action: &RuleAction, _event: &BotEvent, spawn_fn: Option<&SpawnSwarmFn>) {
        match action {
            RuleAction::SpawnSwarm { template, params } => {
                tracing::info!("Spawning swarm template='{}' params={:?}", template, params);
                if let Some(f) = spawn_fn {
                    let params_val: serde_json::Value = params.iter()
                        .map(|(k, v)| (k.clone(), serde_json::Value::String(v.clone())))
                        .collect::<serde_json::Map<_, _>>()
                        .into();
                    f(template.clone(), params_val);
                }
            }
            RuleAction::RunCi { workspace_root } => {
                tracing::info!("Triggering CI pipeline at {}", workspace_root);
                // Calls bonsai-ci BonsaiCi::run_full_pipeline in Phase 6
            }
            RuleAction::HotReloadModel { adapter_path } => {
                tracing::info!("Hot-reloading model adapter: {}", adapter_path);
                // Calls hot_reload::reload_adapter in Phase 6
            }
            RuleAction::RollbackComponent { component } => {
                tracing::info!("Rolling back component: {}", component);
                // Calls UpgradeDispatcher::rollback in Phase 6
            }
            RuleAction::CreateIssue { title, body, labels } => {
                tracing::info!("Creating issue: '{}' labels={:?}", title, labels);
                let _ = body;
                // Calls IssueTracker::create_issue in Phase 6
            }
            RuleAction::NotifyUser { message, level } => {
                tracing::info!("[{}] Bot notification: {}", level, message);
                // Tauri notification in Phase 6
            }
            RuleAction::ExecuteCommand { program, args } => {
                tracing::info!("Executing: {} {:?}", program, args);
                let _ = tokio::process::Command::new(program).args(args).spawn();
            }
        }
    }

    pub async fn add_rule(&self, rule: BotRule) {
        self.rules.write().await.push(rule);
    }

    pub async fn list_rules(&self) -> Vec<BotRule> {
        self.rules.read().await.clone()
    }

    pub async fn enable_rule(&self, name: &str, enabled: bool) {
        for rule in self.rules.write().await.iter_mut() {
            if rule.name == name {
                rule.enabled = enabled;
            }
        }
    }
}

fn default_rules() -> Vec<BotRule> {
    vec![
        BotRule {
            name: "TestFailed → BugFixer".into(),
            trigger: Trigger::Event(EventPattern {
                event_type: "TestFailed".into(),
                filters: HashMap::new(),
            }),
            action: RuleAction::SpawnSwarm {
                template: "bug_fixer".into(),
                params: HashMap::new(),
            },
            enabled: true,
            cooldown_secs: 120,
        },
        BotRule {
            name: "PRMerged → RunCI".into(),
            trigger: Trigger::Event(EventPattern {
                event_type: "PRMerged".into(),
                filters: HashMap::new(),
            }),
            action: RuleAction::RunCi {
                workspace_root: std::env::var("BONSAI_WORKSPACE_ROOT")
                    .unwrap_or_else(|_| "Z:/Projects/BonsaiWorkspace".into()),
            },
            enabled: true,
            cooldown_secs: 30,
        },
        BotRule {
            name: "CrashDetected → Rollback".into(),
            trigger: Trigger::Event(EventPattern {
                event_type: "CrashDetected".into(),
                filters: HashMap::new(),
            }),
            action: RuleAction::RollbackComponent {
                component: "daemon".into(),
            },
            enabled: true,
            cooldown_secs: 60,
        },
        BotRule {
            name: "TrainingComplete → HotReload".into(),
            trigger: Trigger::Event(EventPattern {
                event_type: "TrainingComplete".into(),
                filters: HashMap::new(),
            }),
            action: RuleAction::HotReloadModel {
                adapter_path: "$event.adapter_path".into(),
            },
            enabled: true,
            cooldown_secs: 0,
        },
        BotRule {
            name: "BuildFailed → CreateIssue".into(),
            trigger: Trigger::Event(EventPattern {
                event_type: "BuildFailed".into(),
                filters: HashMap::new(),
            }),
            action: RuleAction::CreateIssue {
                title: "CI Build Failure".into(),
                body: "Automated issue from BotRuleEngine".into(),
                labels: vec!["bug".into(), "ci".into()],
            },
            enabled: true,
            cooldown_secs: 300,
        },
        BotRule {
            name: "ComponentHealthDegraded → Notify".into(),
            trigger: Trigger::Event(EventPattern {
                event_type: "ComponentHealthDegraded".into(),
                filters: HashMap::new(),
            }),
            action: RuleAction::NotifyUser {
                message: "Component health degraded — check SystemHealth panel".into(),
                level: "warning".into(),
            },
            enabled: true,
            cooldown_secs: 120,
        },
    ]
}
