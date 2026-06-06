use chrono::{DateTime, Utc};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::sync::Arc;
use tokio::sync::RwLock;
use uuid::Uuid;

#[derive(Debug, Clone, PartialEq, Eq, Hash, Serialize, Deserialize, PartialOrd, Ord)]
#[serde(rename_all = "snake_case")]
pub enum IssueStatus {
    Open,
    Assigned,
    InProgress,
    InReview,
    Resolved,
    Closed,
    Blocked,
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
#[serde(rename_all = "snake_case")]
pub enum IssuePriority {
    Critical = 0,
    High = 1,
    Medium = 2,
    Low = 3,
    Backlog = 4,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Issue {
    pub id: String,
    pub title: String,
    pub body: String,
    pub status: IssueStatus,
    pub priority: IssuePriority,
    pub labels: Vec<String>,
    pub assignee: Option<String>,
    pub swarm_template: Option<String>,
    pub swarm_id: Option<String>,
    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
    pub resolved_at: Option<DateTime<Utc>>,
    pub resolution: Option<String>,
    pub commit_sha: Option<String>,
    pub depends_on: Vec<String>,
    pub attempts: u32,
}

pub struct IssueTracker {
    issues: Arc<RwLock<Vec<Issue>>>,
    db_path: String,
}

impl IssueTracker {
    pub fn new(db_path: &str) -> Self {
        if let Some(parent) = std::path::Path::new(db_path).parent() {
            let _ = std::fs::create_dir_all(parent);
        }
        Self {
            issues: Arc::new(RwLock::new(Vec::new())),
            db_path: db_path.to_string(),
        }
    }

    pub async fn load(&self) {
        if let Ok(data) = tokio::fs::read_to_string(&self.db_path).await {
            if let Ok(issues) = serde_json::from_str::<Vec<Issue>>(&data) {
                *self.issues.write().await = issues;
            }
        }
    }

    pub async fn save(&self) {
        let issues = self.issues.read().await;
        if let Ok(data) = serde_json::to_string_pretty(&*issues) {
            let _ = tokio::fs::write(&self.db_path, data).await;
        }
    }

    pub async fn create(
        &self,
        title: impl Into<String>,
        body: impl Into<String>,
        priority: IssuePriority,
        labels: Vec<String>,
        created_by: impl Into<String>,
    ) -> Issue {
        let now = Utc::now();
        let issue = Issue {
            id: Uuid::new_v4().to_string(),
            title: title.into(),
            body: body.into(),
            status: IssueStatus::Open,
            priority,
            labels,
            assignee: None,
            swarm_template: None,
            swarm_id: None,
            created_by: created_by.into(),
            created_at: now,
            updated_at: now,
            resolved_at: None,
            resolution: None,
            commit_sha: None,
            depends_on: Vec::new(),
            attempts: 0,
        };
        self.issues.write().await.push(issue.clone());
        self.save().await;
        issue
    }

    pub async fn next_unassigned(&self) -> Option<Issue> {
        let issues = self.issues.read().await;
        let resolved_ids: std::collections::HashSet<&str> = issues
            .iter()
            .filter(|i| i.status == IssueStatus::Resolved || i.status == IssueStatus::Closed)
            .map(|i| i.id.as_str())
            .collect();

        issues
            .iter()
            .filter(|i| i.status == IssueStatus::Open && i.assignee.is_none())
            .filter(|i| i.depends_on.iter().all(|dep| resolved_ids.contains(dep.as_str())))
            .min_by_key(|i| (&i.priority, i.attempts, i.created_at))
            .cloned()
    }

    pub async fn assign(&self, issue_id: &str, assignee: &str, swarm_template: &str) -> bool {
        let mut issues = self.issues.write().await;
        if let Some(issue) = issues.iter_mut().find(|i| i.id == issue_id) {
            issue.status = IssueStatus::Assigned;
            issue.assignee = Some(assignee.to_string());
            issue.swarm_template = Some(swarm_template.to_string());
            issue.updated_at = Utc::now();
            issue.attempts += 1;
            drop(issues);
            self.save().await;
            return true;
        }
        false
    }

    pub async fn set_in_progress(&self, issue_id: &str) -> bool {
        self.update_status(issue_id, IssueStatus::InProgress).await
    }

    pub async fn resolve(
        &self,
        issue_id: &str,
        resolution: &str,
        commit_sha: Option<String>,
    ) -> bool {
        let mut issues = self.issues.write().await;
        if let Some(issue) = issues.iter_mut().find(|i| i.id == issue_id) {
            let now = Utc::now();
            issue.status = IssueStatus::Resolved;
            issue.resolution = Some(resolution.to_string());
            issue.commit_sha = commit_sha;
            issue.resolved_at = Some(now);
            issue.updated_at = now;
            drop(issues);
            self.save().await;
            return true;
        }
        false
    }

    pub async fn block(&self, issue_id: &str, reason: &str) -> bool {
        let mut issues = self.issues.write().await;
        if let Some(issue) = issues.iter_mut().find(|i| i.id == issue_id) {
            issue.status = IssueStatus::Blocked;
            issue.resolution = Some(format!("Blocked: {}", reason));
            issue.updated_at = Utc::now();
            drop(issues);
            self.save().await;
            return true;
        }
        false
    }

    async fn update_status(&self, issue_id: &str, status: IssueStatus) -> bool {
        let mut issues = self.issues.write().await;
        if let Some(issue) = issues.iter_mut().find(|i| i.id == issue_id) {
            issue.status = status;
            issue.updated_at = Utc::now();
            drop(issues);
            self.save().await;
            return true;
        }
        false
    }

    pub async fn get(&self, issue_id: &str) -> Option<Issue> {
        self.issues.read().await.iter().find(|i| i.id == issue_id).cloned()
    }

    pub async fn list_by_status(&self, status: IssueStatus) -> Vec<Issue> {
        self.issues.read().await.iter().filter(|i| i.status == status).cloned().collect()
    }

    pub async fn list_all(&self) -> Vec<Issue> {
        self.issues.read().await.clone()
    }

    pub async fn counts(&self) -> HashMap<String, usize> {
        let issues = self.issues.read().await;
        let mut map: HashMap<String, usize> = HashMap::new();
        for issue in issues.iter() {
            let key = format!("{:?}", issue.status).to_lowercase();
            *map.entry(key).or_default() += 1;
        }
        map
    }
}
