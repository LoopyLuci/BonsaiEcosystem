//! Bias Detector - identifies biased language
use ahf_core::{BiasReport, Error, Result};
use regex::Regex;
use serde::{Deserialize, Serialize};
use std::sync::Arc;
use tokio::sync::RwLock;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BiasPattern {
    pub name: String,
    pub regex: String,
    pub category: BiasCategory,
    pub severity: f32,
    pub explanation: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum BiasCategory {
    Gender,
    Race,
    Age,
    Religion,
    Disability,
    Socioeconomic,
    Nationality,
    Other(String),
}

#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct BiasStats {
    pub total_checks: u64,
    pub total_flagged: u64,
}

pub struct BiasDetector {
    patterns: Arc<RwLock<Vec<(Regex, BiasPattern)>>>,
    stats: Arc<RwLock<BiasStats>>,
}

impl BiasDetector {
    pub fn new() -> Self {
        let mut detector = Self {
            patterns: Arc::new(RwLock::new(Vec::new())),
            stats: Arc::new(RwLock::new(BiasStats::default())),
        };
        detector.load_default_patterns();
        detector
    }

    fn load_default_patterns(&mut self) {
        let defaults = vec![
            BiasPattern {
                name: "gender_stereotype".into(),
                regex: r"(?i)\b(women|men)\s+(are\s+not\s+good|cannot)\b".into(),
                category: BiasCategory::Gender,
                severity: 0.9,
                explanation: "Gender stereotype detected".into(),
            },
            BiasPattern {
                name: "racial_stereotype".into(),
                regex: r"(?i)\b(black|white|asian)\s+people\s+are\s+(less|more)\s+intelligent\b".into(),
                category: BiasCategory::Race,
                severity: 0.95,
                explanation: "Racial stereotype detected".into(),
            },
        ];

        let compiled: Vec<(Regex, BiasPattern)> = defaults
            .into_iter()
            .filter_map(|p| Regex::new(&p.regex).ok().map(|r| (r, p)))
            .collect();

        futures::executor::block_on(async {
            let mut patterns = self.patterns.write().await;
            *patterns = compiled;
        });
    }

    pub async fn add_pattern(&self, pattern: BiasPattern) -> Result<()> {
        let regex = Regex::new(&pattern.regex)
            .map_err(|e| Error::SerializationError(format!("Invalid regex: {}", e)))?;
        let mut patterns = self.patterns.write().await;
        patterns.push((regex, pattern));
        Ok(())
    }

    pub async fn detect(&self, text: &str) -> BiasReport {
        let patterns = self.patterns.read().await;
        let mut matched_patterns = Vec::new();
        let mut max_severity = 0.0f32;

        for (regex, pattern) in patterns.iter() {
            if regex.is_match(text) {
                matched_patterns.push(pattern.name.clone());
                max_severity = max_severity.max(pattern.severity);
            }
        }

        let bias_score = if matched_patterns.is_empty() {
            0.0
        } else {
            (max_severity * 0.7 + (matched_patterns.len() as f32 * 0.1).min(0.3)).min(1.0)
        };

        {
            let mut stats = self.stats.write().await;
            stats.total_checks += 1;
            if bias_score > 0.5 {
                stats.total_flagged += 1;
            }
        }

        BiasReport {
            bias_score,
            matched_patterns,
            ml_flagged: false,
            explanation: String::new(),
        }
    }

    pub async fn stats(&self) -> BiasStats {
        self.stats.read().await.clone()
    }
}

impl Default for BiasDetector {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_detect_bias() {
        let detector = BiasDetector::new();
        let report = detector.detect("Women are not good at math.").await;
        assert!(report.bias_score > 0.5);
    }

    #[tokio::test]
    async fn test_no_bias() {
        let detector = BiasDetector::new();
        let report = detector.detect("The sky is blue.").await;
        assert_eq!(report.bias_score, 0.0);
    }
}
