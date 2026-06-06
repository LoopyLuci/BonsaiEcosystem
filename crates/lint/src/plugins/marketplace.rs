/// Bonsai Plugin Marketplace
/// Community plugin distribution and discovery

use anyhow::Result;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BonsaiPlugin {
    pub id: String,
    pub name: String,
    pub version: String,
    pub author: String,
    pub description: String,
    pub rules: Vec<String>,
    pub languages: Vec<String>,
    pub rating: f32,
    pub rating_count: u32,
    pub downloads: u32,
    pub tags: Vec<String>,
    pub repository: Option<String>,
    pub license: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PluginMetadata {
    pub plugin_id: String,
    pub installed: bool,
    pub installed_version: Option<String>,
    pub update_available: bool,
    pub latest_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PublishResult {
    pub plugin_id: String,
    pub version: String,
    pub published_at: String,
}

pub struct PluginMarketplace {
    registry_url: String,
    cache: HashMap<String, BonsaiPlugin>,
}

impl PluginMarketplace {
    pub async fn new(registry_url: String) -> Result<Self> {
        tracing::info!("Initializing plugin marketplace at: {}", registry_url);

        Ok(Self {
            registry_url,
            cache: HashMap::new(),
        })
    }

    /// Search plugins by query
    pub async fn search_plugins(&self, query: &str) -> Result<Vec<BonsaiPlugin>> {
        tracing::debug!("Searching plugins: {}", query);

        // TODO: Replace with actual marketplace API call
        // let url = format!("{}/search?q={}", self.registry_url, urlencoding::encode(query));
        // let plugins = reqwest::get(&url).await?.json().await?;

        Ok(Vec::new())
    }

    /// Get plugins by language
    pub async fn get_plugins_for_language(&self, language: &str) -> Result<Vec<BonsaiPlugin>> {
        tracing::debug!("Fetching plugins for language: {}", language);

        // TODO: Replace with actual API call
        // let url = format!("{}/language/{}", self.registry_url, language);
        // let plugins = reqwest::get(&url).await?.json().await?;

        Ok(Vec::new())
    }

    /// Get top-rated plugins
    pub async fn get_top_plugins(&self, limit: usize) -> Result<Vec<BonsaiPlugin>> {
        tracing::debug!("Fetching top {} plugins", limit);

        // TODO: Replace with actual API call
        // let url = format!("{}/top?limit={}", self.registry_url, limit);
        // let plugins = reqwest::get(&url).await?.json().await?;

        Ok(Vec::new())
    }

    /// Install a plugin
    pub async fn install_plugin(&self, plugin_id: &str, version: Option<&str>) -> Result<BonsaiPlugin> {
        tracing::info!("Installing plugin: {}", plugin_id);

        let plugin = self.fetch_plugin(plugin_id, version).await?;

        // TODO: Extract and register plugin
        // self.extract_and_register(&plugin).await?;

        tracing::info!("Plugin {} v{} installed", plugin.name, plugin.version);
        Ok(plugin)
    }

    /// Uninstall a plugin
    pub async fn uninstall_plugin(&self, plugin_id: &str) -> Result<()> {
        tracing::info!("Uninstalling plugin: {}", plugin_id);

        // TODO: Remove plugin files and clean up registry
        // self.remove_plugin_files(plugin_id).await?;

        Ok(())
    }

    /// Publish a plugin to marketplace
    pub async fn publish_plugin(&self, plugin: BonsaiPlugin) -> Result<PublishResult> {
        tracing::info!("Publishing plugin: {} v{}", plugin.name, plugin.version);

        let url = format!("{}/publish", self.registry_url);

        // TODO: Replace with actual API call
        // let response = reqwest::Client::new()
        //     .post(&url)
        //     .json(&plugin)
        //     .send()
        //     .await?;
        // let result: PublishResult = response.json().await?;

        let result = PublishResult {
            plugin_id: plugin.id,
            version: plugin.version,
            published_at: chrono::Utc::now().to_rfc3339(),
        };

        Ok(result)
    }

    /// Rate a plugin
    pub async fn rate_plugin(&self, plugin_id: &str, rating: f32) -> Result<()> {
        let rating = rating.clamp(1.0, 5.0);
        tracing::info!("Rating plugin {}: {:.1} stars", plugin_id, rating);

        // TODO: Replace with actual API call
        // let url = format!("{}/rate", self.registry_url);
        // reqwest::Client::new()
        //     .post(&url)
        //     .json(&serde_json::json!({"plugin_id": plugin_id, "rating": rating}))
        //     .send()
        //     .await?;

        Ok(())
    }

    /// Check for plugin updates
    pub async fn check_updates(&self, installed_plugins: Vec<PluginMetadata>) -> Result<Vec<PluginMetadata>> {
        let mut updates = Vec::new();

        for plugin in installed_plugins {
            // TODO: Check latest version from marketplace
            // let latest = self.fetch_latest_version(&plugin.plugin_id).await?;
            // if latest > plugin.installed_version {
            //     updates.push(PluginMetadata {
            //         update_available: true,
            //         latest_version: latest,
            //         ...plugin
            //     });
            // }
        }

        Ok(updates)
    }

    async fn fetch_plugin(&self, plugin_id: &str, _version: Option<&str>) -> Result<BonsaiPlugin> {
        tracing::debug!("Fetching plugin metadata: {}", plugin_id);

        // TODO: Replace with actual API call
        // let url = format!("{}/plugins/{}", self.registry_url, plugin_id);
        // let plugin = reqwest::get(&url).await?.json().await?;

        let plugin = BonsaiPlugin {
            id: plugin_id.to_string(),
            name: "Example Plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "author".to_string(),
            description: "Example plugin".to_string(),
            rules: vec![],
            languages: vec!["rust".to_string()],
            rating: 4.5,
            rating_count: 100,
            downloads: 1000,
            tags: vec![],
            repository: None,
            license: "MIT".to_string(),
        };

        Ok(plugin)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_marketplace_creation() {
        let marketplace = PluginMarketplace::new("https://plugins.bonsai.sh".to_string())
            .await
            .unwrap();
        assert_eq!(marketplace.registry_url, "https://plugins.bonsai.sh");
    }

    #[tokio::test]
    async fn test_search_plugins() {
        let marketplace = PluginMarketplace::new("https://plugins.bonsai.sh".to_string())
            .await
            .unwrap();
        let plugins = marketplace.search_plugins("performance").await.unwrap();
        assert!(plugins.is_empty()); // No mock data
    }

    #[tokio::test]
    async fn test_install_plugin() {
        let marketplace = PluginMarketplace::new("https://plugins.bonsai.sh".to_string())
            .await
            .unwrap();
        let plugin = marketplace.install_plugin("test-plugin", None).await.unwrap();
        assert_eq!(plugin.id, "test-plugin");
    }

    #[tokio::test]
    async fn test_publish_plugin() {
        let marketplace = PluginMarketplace::new("https://plugins.bonsai.sh".to_string())
            .await
            .unwrap();
        let plugin = BonsaiPlugin {
            id: "my-plugin".to_string(),
            name: "My Plugin".to_string(),
            version: "1.0.0".to_string(),
            author: "me".to_string(),
            description: "My plugin".to_string(),
            rules: vec!["rule1".to_string()],
            languages: vec!["rust".to_string()],
            rating: 0.0,
            rating_count: 0,
            downloads: 0,
            tags: vec![],
            repository: None,
            license: "MIT".to_string(),
        };
        let result = marketplace.publish_plugin(plugin).await.unwrap();
        assert_eq!(result.plugin_id, "my-plugin");
    }
}
