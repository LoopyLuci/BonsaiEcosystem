//! Bonsai Training Data Library (TDL)
//!
//! A production-grade SQLite-backed training dataset management system with:
//! - Versioned datasets with full history tracking
//! - Quality scoring and metadata management
//! - Multi-format export (JSONL, Parquet)
//! - Transactional safety for concurrent access
//! - Zero-copy blob handling for large examples

pub mod db;
pub mod error;
pub mod library;
pub mod models;

pub use db::TrainingDataDb;
pub use error::{TdlError, Result};
pub use library::{ExportFormat, TrainingDataLibrary};
pub use models::{Example, Metadata, Version, VersionInfo};

use std::path::Path;

/// Initialize a new training data library at the given path.
///
/// # Errors
///
/// Returns an error if the database cannot be created or initialized.
pub async fn init_library(db_path: &Path) -> Result<TrainingDataLibrary> {
    TrainingDataLibrary::new(db_path).await
}

#[cfg(test)]
mod tests {
    use super::*;
    use tempfile::TempDir;

    #[tokio::test]
    async fn test_create_library() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.db");
        let library = TrainingDataLibrary::new(&db_path).await?;
        Ok(())
    }

    #[tokio::test]
    async fn test_create_version() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.db");
        let library = TrainingDataLibrary::new(&db_path).await?;

        let version_id = library
            .create_version("1.0.0", "test_user", "Initial version", vec![])
            .await?;

        let version = library.get_version(version_id).await?;
        assert!(version.is_some());
        let v = version.unwrap();
        assert_eq!(v.version_string, "1.0.0");
        assert_eq!(v.created_by, "test_user");

        Ok(())
    }

    #[tokio::test]
    async fn test_add_example() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.db");
        let library = TrainingDataLibrary::new(&db_path).await?;

        let version_id = library
            .create_version("1.0.0", "test_user", "Initial version", vec![])
            .await?;

        let metadata = Metadata::new()
            .with_source("test_source")
            .with_author("test_author")
            .with_domain("nlp")
            .with_tag("test");

        let example_id = library
            .add_example(
                version_id,
                "This is test content".to_string(),
                metadata,
                0.95,
            )
            .await?;

        let version = library.get_version(version_id).await?;
        assert!(version.is_some());
        let v = version.unwrap();
        assert_eq!(v.example_count, 1);
        assert_eq!(v.avg_quality_score, 0.95);

        Ok(())
    }

    #[tokio::test]
    async fn test_quality_score_validation() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.db");
        let library = TrainingDataLibrary::new(&db_path).await?;

        let version_id = library
            .create_version("1.0.0", "test_user", "Initial version", vec![])
            .await?;

        let metadata = Metadata::new();

        // Test invalid quality score > 1.0
        let result = library
            .add_example(version_id, "content".to_string(), metadata.clone(), 1.5)
            .await;
        assert!(result.is_err());

        // Test invalid quality score < 0.0
        let result = library
            .add_example(version_id, "content".to_string(), metadata, -0.1)
            .await;
        assert!(result.is_err());

        Ok(())
    }

    #[tokio::test]
    async fn test_search_by_quality() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.db");
        let library = TrainingDataLibrary::new(&db_path).await?;

        let version_id = library
            .create_version("1.0.0", "test_user", "Initial version", vec![])
            .await?;

        // Add examples with different quality scores
        for i in 0..5 {
            library
                .add_example(
                    version_id,
                    format!("Example {}", i),
                    Metadata::new(),
                    i as f32 * 0.2,
                )
                .await?;
        }

        // Search for high quality
        let high_quality = library.search_by_quality(0.6, 10).await?;
        assert!(high_quality.len() >= 2);

        Ok(())
    }

    #[tokio::test]
    async fn test_version_history() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.db");
        let library = TrainingDataLibrary::new(&db_path).await?;

        library
            .create_version("1.0.0", "user1", "First version", vec![])
            .await?;
        library
            .create_version("1.1.0", "user1", "Second version", vec![])
            .await?;
        library
            .create_version("2.0.0", "user2", "Third version", vec![])
            .await?;

        let history = library.get_version_history().await?;
        assert_eq!(history.len(), 3);

        Ok(())
    }

    #[tokio::test]
    async fn test_merge_versions() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.db");
        let library = TrainingDataLibrary::new(&db_path).await?;

        let v1_id = library
            .create_version("1.0.0", "user", "Version 1", vec![])
            .await?;
        let v2_id = library
            .create_version("1.1.0", "user", "Version 2", vec![])
            .await?;

        library
            .add_example(
                v1_id,
                "Content 1".to_string(),
                Metadata::new(),
                0.9,
            )
            .await?;
        library
            .add_example(
                v2_id,
                "Content 2".to_string(),
                Metadata::new(),
                0.8,
            )
            .await?;

        let merged_id = library.merge_versions(v1_id, v2_id, "merger").await?;
        let merged = library.get_version(merged_id).await?;

        assert!(merged.is_some());
        assert_eq!(merged.unwrap().example_count, 2);

        Ok(())
    }

    #[tokio::test]
    async fn test_export_jsonl() -> Result<()> {
        let temp_dir = TempDir::new()?;
        let db_path = temp_dir.path().join("test.db");
        let library = TrainingDataLibrary::new(&db_path).await?;

        let version_id = library
            .create_version("1.0.0", "user", "Version", vec![])
            .await?;

        library
            .add_example(
                version_id,
                "Test content".to_string(),
                Metadata::new().with_tag("test"),
                0.95,
            )
            .await?;

        let export_path = temp_dir.path().join("export.jsonl");
        library
            .export_dataset(version_id, ExportFormat::Jsonl, &export_path)
            .await?;

        assert!(export_path.exists());
        let content = std::fs::read_to_string(&export_path)?;
        assert!(content.contains("Test content"));

        Ok(())
    }

    #[tokio::test]
    async fn test_metadata_builder() {
        let metadata = Metadata::new()
            .with_source("https://example.com")
            .with_author("Alice")
            .with_domain("ai")
            .with_language("en")
            .with_tag("tag1")
            .with_tag("tag2");

        assert_eq!(metadata.source, Some("https://example.com".to_string()));
        assert_eq!(metadata.author, Some("Alice".to_string()));
        assert_eq!(metadata.domain, Some("ai".to_string()));
        assert_eq!(metadata.language, Some("en".to_string()));
        assert_eq!(metadata.tags.len(), 2);
    }
}
