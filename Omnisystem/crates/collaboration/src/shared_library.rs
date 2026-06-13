/// Shared rule library - publish and discover community rules.

use anyhow::Result;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

/// A shared rule in the library.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SharedRule {
    pub rule_id: String,
    pub name: String,
    pub description: String,
    pub rule_content: String,  // YAML/TOML rule definition

    /// Organization that owns this rule (None = community rule)
    pub organization_id: Option<String>,

    pub created_by: String,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,

    // Social metrics
    pub rating: f32,        // 0-5 stars
    pub rating_count: u32,
    pub downloads: u32,

    // Categorization
    pub tags: Vec<String>,
    pub language: String,
    pub domain: String,  // "web", "systems", "data", etc.
}

impl SharedRule {
    pub fn new(
        rule_id: String,
        name: String,
        description: String,
        rule_content: String,
        created_by: String,
        language: String,
        domain: String,
    ) -> Self {
        Self {
            rule_id,
            name,
            description,
            rule_content,
            organization_id: None,
            created_by,
            created_at: Utc::now(),
            updated_at: Utc::now(),
            rating: 0.0,
            rating_count: 0,
            downloads: 0,
            tags: Vec::new(),
            language,
            domain,
        }
    }

    /// Rate this rule (1-5 stars).
    pub fn add_rating(&mut self, rating: f32) {
        let total = self.rating * self.rating_count as f32;
        self.rating_count += 1;
        self.rating = (total + rating.clamp(1.0, 5.0)) / self.rating_count as f32;
    }

    /// Increment download counter.
    pub fn increment_downloads(&mut self) {
        self.downloads += 1;
    }
}

/// Rule library for sharing and discovering rules.
pub struct RuleLibrary {
    rules: Arc<DashMap<String, SharedRule>>,
}

impl RuleLibrary {
    pub async fn new(_db_path: PathBuf) -> Result<Self> {
        // TODO: Load rules from database
        Ok(Self {
            rules: Arc::new(DashMap::new()),
        })
    }

    /// Publish a rule to the library.
    pub async fn publish_rule(&self, rule: SharedRule) -> Result<String> {
        let rule_id = rule.rule_id.clone();

        self.rules.insert(rule_id.clone(), rule);

        // TODO: Persist to database

        tracing::info!("Published rule: {}", rule_id);
        Ok(rule_id)
    }

    /// Get a rule by ID.
    pub async fn get_rule(&self, rule_id: &str) -> Result<Option<SharedRule>> {
        Ok(self
            .rules
            .get(rule_id)
            .map(|entry| entry.clone()))
    }

    /// Search rules by query string.
    pub async fn search(&self, query: &str) -> Result<Vec<SharedRule>> {
        let query_lower = query.to_lowercase();

        let results: Vec<_> = self
            .rules
            .iter()
            .filter(|entry| {
                let rule = entry.value();
                rule.name.to_lowercase().contains(&query_lower)
                    || rule.description.to_lowercase().contains(&query_lower)
                    || rule.tags.iter().any(|t| t.to_lowercase().contains(&query_lower))
            })
            .map(|entry| entry.value().clone())
            .collect();

        Ok(results)
    }

    /// Get rules by language.
    pub async fn get_rules_for_language(&self, language: &str) -> Result<Vec<SharedRule>> {
        let rules: Vec<_> = self
            .rules
            .iter()
            .filter(|entry| entry.value().language == language)
            .map(|entry| entry.value().clone())
            .collect();

        Ok(rules)
    }

    /// Get rules by domain.
    pub async fn get_rules_for_domain(&self, domain: &str) -> Result<Vec<SharedRule>> {
        let rules: Vec<_> = self
            .rules
            .iter()
            .filter(|entry| entry.value().domain == domain)
            .map(|entry| entry.value().clone())
            .collect();

        Ok(rules)
    }

    /// Get top-rated rules.
    pub async fn get_top_rules(&self, limit: usize) -> Result<Vec<SharedRule>> {
        let mut rules: Vec<_> = self
            .rules
            .iter()
            .map(|entry| entry.value().clone())
            .collect();

        rules.sort_by(|a, b| b.rating.partial_cmp(&a.rating).unwrap_or(std::cmp::Ordering::Equal));
        rules.truncate(limit);

        Ok(rules)
    }

    /// Rate a rule.
    pub async fn rate_rule(&self, rule_id: &str, rating: f32) -> Result<()> {
        if let Some(mut rule) = self.rules.get_mut(rule_id) {
            rule.add_rating(rating);
            rule.updated_at = Utc::now();

            // TODO: Persist to database

            tracing::info!("Rated rule {}: {:.1} stars", rule_id, rating);
            Ok(())
        } else {
            Err(anyhow::anyhow!("Rule not found: {}", rule_id))
        }
    }

    /// Download a rule (increments counter).
    pub async fn download_rule(&self, rule_id: &str) -> Result<Option<SharedRule>> {
        if let Some(mut rule) = self.rules.get_mut(rule_id) {
            rule.increment_downloads();
            rule.updated_at = Utc::now();

            // TODO: Persist to database

            tracing::info!("Downloaded rule {} (count: {})", rule_id, rule.downloads);
            return Ok(Some(rule.clone()));
        }

        Ok(None)
    }

    /// Get number of rules in library.
    pub async fn rule_count(&self) -> Result<usize> {
        Ok(self.rules.len())
    }

    /// List all rules.
    pub async fn list_rules(&self) -> Result<Vec<SharedRule>> {
        let rules: Vec<_> = self
            .rules
            .iter()
            .map(|entry| entry.value().clone())
            .collect();

        Ok(rules)
    }

    /// Delete a rule.
    pub async fn delete_rule(&self, rule_id: &str) -> Result<()> {
        self.rules.remove(rule_id);

        // TODO: Remove from database

        tracing::info!("Deleted rule: {}", rule_id);
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_rule_library_creation() {
        let tmp_dir = std::env::temp_dir().join("test_rules");
        let library = RuleLibrary::new(tmp_dir).await.unwrap();
        assert_eq!(library.rule_count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_publish_rule() {
        let tmp_dir = std::env::temp_dir().join("test_rules");
        let library = RuleLibrary::new(tmp_dir).await.unwrap();

        let rule = SharedRule::new(
            "unused-import".to_string(),
            "Unused Import".to_string(),
            "Detects unused import statements".to_string(),
            "pattern: ...".to_string(),
            "author".to_string(),
            "rust".to_string(),
            "web".to_string(),
        );

        library.publish_rule(rule).await.unwrap();

        assert_eq!(library.rule_count().await.unwrap(), 1);
    }

    #[tokio::test]
    async fn test_search_rules() {
        let tmp_dir = std::env::temp_dir().join("test_rules");
        let library = RuleLibrary::new(tmp_dir).await.unwrap();

        let rule = SharedRule::new(
            "unused-import".to_string(),
            "Unused Import".to_string(),
            "Detects unused import statements".to_string(),
            "pattern: ...".to_string(),
            "author".to_string(),
            "rust".to_string(),
            "web".to_string(),
        );

        library.publish_rule(rule).await.unwrap();

        let results = library.search("unused").await.unwrap();
        assert_eq!(results.len(), 1);
    }

    #[test]
    fn test_rule_rating() {
        let mut rule = SharedRule::new(
            "test-rule".to_string(),
            "Test".to_string(),
            "Test rule".to_string(),
            "pattern: ...".to_string(),
            "author".to_string(),
            "rust".to_string(),
            "web".to_string(),
        );

        rule.add_rating(5.0);
        rule.add_rating(4.0);

        assert_eq!(rule.rating_count, 2);
        assert_eq!(rule.rating, 4.5);
    }
}
