/// Voting system for rule improvements and mutations.

use anyhow::Result;
use chrono::{DateTime, Utc};
use dashmap::DashMap;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;
use std::sync::Arc;

/// A vote on a rule mutation proposal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleVote {
    pub vote_id: String,
    pub proposal_id: String,
    pub rule_id: String,
    pub voter_id: String,
    pub vote: VoteType,
    pub timestamp: DateTime<Utc>,
}

/// Type of vote.
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub enum VoteType {
    Approve { reason: String },
    Reject { reason: String },
    Abstain,
}

/// Summary of votes for a proposal.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct VoteSummary {
    pub proposal_id: String,
    pub rule_id: String,
    pub approvals: usize,
    pub rejections: usize,
    pub abstentions: usize,
}

impl VoteSummary {
    pub fn vote_count(&self) -> u32 {
        (self.approvals + self.rejections) as u32
    }

    pub fn approval_rate(&self) -> f32 {
        let total = self.approvals + self.rejections;
        if total == 0 {
            return 0.0;
        }
        self.approvals as f32 / total as f32
    }
}

/// Result of proposal evaluation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ProposalDecision {
    Approved { consensus: f32 },
    Rejected { consensus: f32 },
    Pending,
}

/// Voting engine for rule proposals.
pub struct VotingEngine {
    votes: Arc<DashMap<String, Vec<RuleVote>>>,
}

impl VotingEngine {
    pub async fn new(_db_path: PathBuf) -> Result<Self> {
        // TODO: Load votes from database
        Ok(Self {
            votes: Arc::new(DashMap::new()),
        })
    }

    /// Cast a vote on a proposal.
    pub async fn vote(
        &self,
        proposal_id: &str,
        voter_id: &str,
        vote: VoteType,
        _organization_id: String,
    ) -> Result<()> {
        let vote_id = format!("vote-{}-{}", proposal_id, uuid::Uuid::new_v4());

        // Extract rule_id from proposal (TODO: lookup from database)
        let rule_id = proposal_id.split('-').next().unwrap_or("unknown").to_string();

        let rule_vote = RuleVote {
            vote_id,
            proposal_id: proposal_id.to_string(),
            rule_id,
            voter_id: voter_id.to_string(),
            vote,
            timestamp: Utc::now(),
        };

        self.votes
            .entry(proposal_id.to_string())
            .or_insert_with(Vec::new)
            .push(rule_vote.clone());

        tracing::info!(
            "Recorded vote on proposal {}: {:?}",
            proposal_id,
            rule_vote.vote
        );

        // TODO: Persist to database

        Ok(())
    }

    /// Get vote summary for a proposal.
    pub async fn get_summary(&self, proposal_id: &str) -> Result<VoteSummary> {
        let votes = self
            .votes
            .get(proposal_id)
            .map(|entry| entry.clone())
            .unwrap_or_default();

        let mut approvals = 0;
        let mut rejections = 0;
        let mut abstentions = 0;
        let mut rule_id = "unknown".to_string();

        for vote in &votes {
            rule_id = vote.rule_id.clone();
            match &vote.vote {
                VoteType::Approve { .. } => approvals += 1,
                VoteType::Reject { .. } => rejections += 1,
                VoteType::Abstain => abstentions += 1,
            }
        }

        Ok(VoteSummary {
            proposal_id: proposal_id.to_string(),
            rule_id,
            approvals,
            rejections,
            abstentions,
        })
    }

    /// Evaluate a proposal (check if it should auto-approve/reject).
    pub async fn evaluate_proposal(
        &self,
        proposal_id: &str,
        approval_threshold: f32,
        min_votes: u32,
    ) -> Result<ProposalDecision> {
        let summary = self.get_summary(proposal_id).await?;

        if summary.vote_count() < min_votes {
            return Ok(ProposalDecision::Pending);
        }

        let approval_rate = summary.approval_rate();
        if approval_rate >= approval_threshold {
            Ok(ProposalDecision::Approved {
                consensus: approval_rate,
            })
        } else {
            Ok(ProposalDecision::Rejected {
                consensus: approval_rate,
            })
        }
    }

    /// Get all votes for a rule.
    pub async fn get_votes_for_rule(&self, rule_id: &str) -> Result<Vec<RuleVote>> {
        let votes = self
            .votes
            .iter()
            .flat_map(|entry| {
                entry
                    .value()
                    .iter()
                    .filter(|v| v.rule_id == rule_id)
                    .cloned()
                    .collect::<Vec<_>>()
            })
            .collect();

        Ok(votes)
    }

    /// Get total vote count.
    pub async fn vote_count(&self) -> Result<usize> {
        let count = self
            .votes
            .iter()
            .map(|entry| entry.value().len())
            .sum();

        Ok(count)
    }

    /// Clear all votes (for testing).
    pub async fn clear(&self) -> Result<()> {
        self.votes.clear();
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[tokio::test]
    async fn test_voting_engine_creation() {
        let tmp_dir = std::env::temp_dir().join("test_voting");
        let engine = VotingEngine::new(tmp_dir).await.unwrap();
        assert_eq!(engine.vote_count().await.unwrap(), 0);
    }

    #[tokio::test]
    async fn test_vote_approval() {
        let tmp_dir = std::env::temp_dir().join("test_voting");
        let engine = VotingEngine::new(tmp_dir).await.unwrap();

        engine
            .vote(
                "prop-1",
                "voter-1",
                VoteType::Approve {
                    reason: "Good".to_string(),
                },
                "org-1".to_string(),
            )
            .await
            .unwrap();

        engine
            .vote(
                "prop-1",
                "voter-2",
                VoteType::Approve {
                    reason: "Looks good".to_string(),
                },
                "org-1".to_string(),
            )
            .await
            .unwrap();

        let summary = engine.get_summary("prop-1").await.unwrap();
        assert_eq!(summary.approvals, 2);
        assert_eq!(summary.rejections, 0);
        assert_eq!(summary.approval_rate(), 1.0);
    }

    #[test]
    fn test_vote_summary_approval_rate() {
        let summary = VoteSummary {
            proposal_id: "prop-1".to_string(),
            rule_id: "rule-1".to_string(),
            approvals: 2,
            rejections: 1,
            abstentions: 0,
        };

        assert!(summary.approval_rate() > 0.66);
    }
}
