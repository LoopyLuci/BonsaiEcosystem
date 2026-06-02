//! Bonsai Collaboration System
//!
//! Enables teams to:
//! - Create and manage team-specific rule profiles
//! - Vote on rule improvements and mutations
//! - Share rules across teams and organizations
//! - Track voting history and consensus

pub mod team_profiles;
pub mod voting;
pub mod shared_library;

pub use team_profiles::{TeamRuleProfile, TeamRuleConfig, TeamProfileManager};
pub use voting::{RuleVote, VotingEngine, VoteType, VoteSummary, ProposalDecision};
pub use shared_library::{SharedRule, RuleLibrary};

use anyhow::Result;
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

/// Configuration for the collaboration system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationConfig {
    /// Database path for storing profiles, votes, etc.
    pub db_path: PathBuf,

    /// Organization ID for shared rules
    pub organization_id: String,

    /// Team ID for this deployment
    pub team_id: String,

    /// Voting approval threshold (0.0-1.0)
    pub approval_threshold: f32,

    /// Minimum votes required for a proposal to be evaluated
    pub min_votes_required: u32,

    /// Enable collaborative features
    pub enabled: bool,
}

impl Default for CollaborationConfig {
    fn default() -> Self {
        Self {
            db_path: PathBuf::from(".bonsai/collaboration"),
            organization_id: "default".to_string(),
            team_id: "default".to_string(),
            approval_threshold: 0.66,  // 66% approval
            min_votes_required: 3,
            enabled: true,
        }
    }
}

/// Main collaboration system orchestrator.
pub struct CollaborationManager {
    config: CollaborationConfig,
    profiles: TeamProfileManager,
    voting: VotingEngine,
    library: RuleLibrary,
}

impl CollaborationManager {
    pub async fn new(config: CollaborationConfig) -> Result<Self> {
        let profiles = TeamProfileManager::new(config.db_path.clone()).await?;
        let voting = VotingEngine::new(config.db_path.clone()).await?;
        let library = RuleLibrary::new(config.db_path.clone()).await?;

        Ok(Self {
            config,
            profiles,
            voting,
            library,
        })
    }

    /// Get team profile for this deployment.
    pub async fn get_team_profile(&self) -> Result<Option<TeamRuleProfile>> {
        self.profiles.get_profile(&self.config.team_id).await
    }

    /// Create a new team profile.
    pub async fn create_team_profile(
        &self,
        name: String,
        inherit_from: Option<String>,
    ) -> Result<TeamRuleProfile> {
        self.profiles
            .create_profile(
                self.config.team_id.clone(),
                self.config.organization_id.clone(),
                name,
                inherit_from,
            )
            .await
    }

    /// Vote on a rule mutation proposal.
    pub async fn vote_on_proposal(
        &self,
        proposal_id: &str,
        voter_id: &str,
        vote: VoteType,
    ) -> Result<()> {
        self.voting
            .vote(proposal_id, voter_id, vote, self.config.organization_id.clone())
            .await
    }

    /// Get voting summary for a proposal.
    pub async fn get_vote_summary(&self, proposal_id: &str) -> Result<VoteSummary> {
        self.voting.get_summary(proposal_id).await
    }

    /// Evaluate a proposal (check if it should be auto-approved).
    pub async fn evaluate_proposal(&self, proposal_id: &str) -> Result<ProposalDecision> {
        let summary = self.get_vote_summary(proposal_id).await?;
        let approval_rate = summary.approval_rate();

        if summary.vote_count() < self.config.min_votes_required {
            return Ok(ProposalDecision::Pending);
        }

        if approval_rate >= self.config.approval_threshold {
            Ok(ProposalDecision::Approved {
                consensus: approval_rate,
            })
        } else {
            Ok(ProposalDecision::Rejected {
                consensus: approval_rate,
            })
        }
    }

    /// Publish a rule to the shared library.
    pub async fn publish_rule(
        &self,
        rule: SharedRule,
    ) -> Result<String> {
        self.library.publish_rule(rule).await
    }

    /// Search shared rules.
    pub async fn search_rules(
        &self,
        query: &str,
    ) -> Result<Vec<SharedRule>> {
        self.library.search(query).await
    }

    /// Get collaboration statistics.
    pub async fn stats(&self) -> Result<CollaborationStats> {
        let profile_count = self.profiles.profile_count().await?;
        let vote_count = self.voting.vote_count().await?;
        let rule_count = self.library.rule_count().await?;

        Ok(CollaborationStats {
            profile_count,
            vote_count,
            rule_count,
            timestamp: Utc::now(),
        })
    }
}

/// Statistics about the collaboration system.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CollaborationStats {
    pub profile_count: usize,
    pub vote_count: usize,
    pub rule_count: usize,
    pub timestamp: chrono::DateTime<Utc>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_collaboration_config_default() {
        let config = CollaborationConfig::default();
        assert_eq!(config.approval_threshold, 0.66);
        assert!(config.enabled);
    }
}
