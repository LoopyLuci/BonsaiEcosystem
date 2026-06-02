# 🧠 Phase A: Self-Improving Rules via EternalTrainingLoop

**Status:** Design Phase  
**Date:** June 1, 2026  
**Objective:** Transform BUL from a static linter into a continuously learning, evolving system

---

## Executive Summary

The **Self-Improving Rules Engine** closes the feedback loop: every user action (accept diagnostic, reject, apply fix, manually edit) becomes a training signal. The EternalTrainingLoop (ETL) processes these signals nightly to:

1. **Adjust rule confidence** – rules with high false-positive rates are suppressed or refined
2. **Refine AI-generated rules** – propose mutations to improve precision/recall
3. **Learn from patterns** – identify which code patterns trigger the most accurate rules
4. **Share knowledge** – persist learnings in KDB so all projects benefit

**Result:** Rules become smarter every day, with confidence scores that reflect real-world accuracy.

---

## 1. Feedback Collection Architecture

### 1.1 User Action Capture

Every linting-related action generates a **feedback event**:

```typescript
// User accepts a diagnostic and applies the suggested fix
→ FeedbackEvent {
  type: "diagnostic_accepted",
  rule_id: "unused-variable",
  file: "src/main.rs",
  line: 42,
  timestamp: "2026-06-01T14:23:45Z",
  user_id: "user@company.com",
  action: "applied_fix",
  confidence_before: 0.85,
  outcome: "success" // or "failure" if the fix caused an error
}

// User sees a diagnostic and manually edits the code
→ FeedbackEvent {
  type: "diagnostic_ignored_then_fixed",
  rule_id: "style-naming-convention",
  file: "src/lib.rs",
  line: 100,
  timestamp: "2026-06-01T14:25:10Z",
  user_id: "user@company.com",
  action: "manual_edit",
  edited_line_before: "fn MyFunction() {}",
  edited_line_after: "fn my_function() {}",
  outcome: "fix_applied"
}

// User marks a diagnostic as a false positive
→ FeedbackEvent {
  type: "false_positive_reported",
  rule_id: "complexity-too-high",
  file: "src/compute.rs",
  line: 200,
  timestamp: "2026-06-01T14:27:30Z",
  user_id: "user@company.com",
  explanation: "This is recursive and intentionally complex",
  context: "function_signature"
}

// User dismisses/ignores a diagnostic (implicit feedback)
→ FeedbackEvent {
  type: "diagnostic_dismissed",
  rule_id: "TODO-comment",
  file: "src/todo.rs",
  line: 50,
  timestamp: "2026-06-01T14:30:00Z",
  user_id: "user@company.com",
  dismissal_count: 3, // User has dismissed this rule 3 times in this session
  outcome: "not_fixed"
}
```

### 1.2 Capture Points in BUL

```rust
// In crates/bonsai-lint/src/integration/feedback_collector.rs

pub struct FeedbackCollector {
    universe_client: UniverseClient,
    tdl: Arc<TrainingDataLibrary>,
}

impl FeedbackCollector {
    /// Called when IDE renders diagnostics
    pub async fn on_diagnostics_rendered(
        &self,
        diagnostics: &[Diagnostic],
        user_id: &str,
    ) -> Result<()> {
        for diag in diagnostics {
            let event = FeedbackEvent {
                event_type: FeedbackEventType::DiagnosticRendered,
                rule_id: diag.rule_id.clone(),
                file: diag.file.clone(),
                line: diag.range.start.line,
                timestamp: now(),
                user_id: user_id.to_string(),
                confidence: diag.confidence,
                severity: diag.severity.to_string(),
                ..Default::default()
            };
            self.emit_event(event).await?;
        }
        Ok(())
    }

    /// Called when user applies a quick-fix
    pub async fn on_fix_applied(
        &self,
        rule_id: &str,
        file: &Path,
        line: u32,
        fix_type: &str,
        outcome: FixOutcome, // Success, BuildFailed, TestFailed, Reverted
        user_id: &str,
    ) -> Result<()> {
        let event = FeedbackEvent {
            event_type: FeedbackEventType::DiagnosticAccepted,
            rule_id: rule_id.to_string(),
            file: file.to_string_lossy().to_string(),
            line,
            timestamp: now(),
            user_id: user_id.to_string(),
            action: "applied_fix",
            fix_type: Some(fix_type.to_string()),
            outcome: Some(outcome.to_string()),
            ..Default::default()
        };
        self.emit_event(event).await?;
        Ok(())
    }

    /// Called when user marks a diagnostic as false positive
    pub async fn on_false_positive_report(
        &self,
        rule_id: &str,
        file: &Path,
        line: u32,
        explanation: &str,
        user_id: &str,
    ) -> Result<()> {
        let event = FeedbackEvent {
            event_type: FeedbackEventType::FalsePositiveReported,
            rule_id: rule_id.to_string(),
            file: file.to_string_lossy().to_string(),
            line,
            timestamp: now(),
            user_id: user_id.to_string(),
            explanation: Some(explanation.to_string()),
            ..Default::default()
        };
        self.emit_event(event).await?;
        Ok(())
    }

    /// Called when user dismisses a diagnostic without action
    pub async fn on_diagnostic_dismissed(
        &self,
        rule_id: &str,
        file: &Path,
        line: u32,
        times_dismissed: u32,
        user_id: &str,
    ) -> Result<()> {
        let event = FeedbackEvent {
            event_type: FeedbackEventType::DiagnosticDismissed,
            rule_id: rule_id.to_string(),
            file: file.to_string_lossy().to_string(),
            line,
            timestamp: now(),
            user_id: user_id.to_string(),
            dismissal_count: Some(times_dismissed),
            ..Default::default()
        };
        self.emit_event(event).await?;
        Ok(())
    }

    async fn emit_event(&self, event: FeedbackEvent) -> Result<()> {
        // 1. Emit to Universe for observability
        self.universe_client.emit(event.clone()).await?;
        
        // 2. Store in TDL for historical analysis
        self.tdl.add_feedback_event(&event).await?;
        
        // 3. Broadcast to EternalTrainingLoop (queued for nightly processing)
        self.universe_client.queue_for_etl(&event).await?;
        
        Ok(())
    }
}
```

### 1.3 IDE Integration

In `bonsai-workspace/src/lib/components/LintPanel.svelte`, add feedback hooks:

```svelte
<script>
  // When user clicks "Apply Fix"
  async function applyFix(diag) {
    const outcome = await applyQuickFix(diag);
    
    // Emit feedback event
    await fetch('/api/feedback/fix-applied', {
      method: 'POST',
      body: JSON.stringify({
        rule_id: diag.rule_id,
        file: diag.file,
        line: diag.line,
        fix_type: diag.fix.type,
        outcome: outcome, // "success" | "build_failed" | "test_failed"
      }),
    });
  }

  // When user marks as false positive
  async function markFalsePositive(diag) {
    const explanation = prompt("Why is this a false positive?");
    if (explanation) {
      await fetch('/api/feedback/false-positive', {
        method: 'POST',
        body: JSON.stringify({
          rule_id: diag.rule_id,
          file: diag.file,
          line: diag.line,
          explanation,
        }),
      });
    }
  }

  // When diagnostic is dismissed (implicit)
  function dismissDiagnostic(diag) {
    dismissalCount[diag.rule_id] = (dismissalCount[diag.rule_id] || 0) + 1;
    
    if (dismissalCount[diag.rule_id] >= 3) {
      // After 3 dismissals, emit feedback
      fetch('/api/feedback/dismissed', {
        method: 'POST',
        body: JSON.stringify({
          rule_id: diag.rule_id,
          file: diag.file,
          line: diag.line,
          times_dismissed: dismissalCount[diag.rule_id],
        }),
      });
    }
  }
</script>
```

---

## 2. Rule Confidence Adjustment

### 2.1 Confidence Score Algorithm

Each rule maintains a **dynamic confidence score** (0.0–1.0) based on historical accuracy:

```rust
// crates/bonsai-lint/src/engine/rule_confidence.rs

pub struct RuleConfidenceMetrics {
    pub rule_id: String,
    pub true_positives: u32,        // User accepted/fixed
    pub false_positives: u32,       // User rejected
    pub dismissed_count: u32,       // User ignored
    pub applied_fixes: u32,         // User applied fix
    pub fix_success_rate: f32,      // Fixes that didn't cause failures
    pub last_updated: DateTime<Utc>,
}

impl RuleConfidenceMetrics {
    /// Calculate dynamic confidence based on accuracy history
    pub fn calculate_confidence(&self) -> f32 {
        let total_observations = (self.true_positives + self.false_positives + self.dismissed_count)
            .max(1) as f32;
        
        // Base accuracy: true positives / total
        let accuracy = self.true_positives as f32 / total_observations;
        
        // Fix success penalty: if fixes are failing, lower confidence
        let fix_penalty = if self.applied_fixes > 0 {
            1.0 - ((self.applied_fixes - (self.fix_success_rate * self.applied_fixes as f32))
                / self.applied_fixes as f32)
                .min(0.3) // Max 30% penalty
        } else {
            0.0
        };
        
        // Dismissal factor: many dismissals = rule is noisy
        let dismissal_factor = (self.dismissed_count as f32 / total_observations).min(0.5);
        
        let confidence = (accuracy - fix_penalty) * (1.0 - dismissal_factor * 0.5);
        confidence.clamp(0.0, 1.0)
    }

    /// Determine rule action based on confidence
    pub fn recommended_action(&self) -> RuleAction {
        let confidence = self.calculate_confidence();
        
        match confidence {
            c if c >= 0.85 => RuleAction::PromoteToError,    // High confidence → error
            c if c >= 0.70 => RuleAction::KeepAsWarning,      // Normal confidence
            c if c >= 0.50 => RuleAction::DemoteToHint,       // Low confidence → hint
            c if c >= 0.30 => RuleAction::MarkAsExperimental, // Very low
            _ => RuleAction::Disable,                          // Too many false positives
        }
    }
}

pub enum RuleAction {
    PromoteToError,
    KeepAsWarning,
    DemoteToHint,
    MarkAsExperimental,
    Disable,
}
```

### 2.2 ETL Confidence Update Loop

```rust
// crates/bonsai-etl/src/rule_confidence_adjuster.rs

pub struct RuleConfidenceAdjuster {
    tdl: Arc<TrainingDataLibrary>,
    kdb_manager: Arc<KdbManager>,
    rule_registry: Arc<RuleRegistry>,
}

impl RuleConfidenceAdjuster {
    /// Run nightly to adjust rule confidence scores
    pub async fn update_confidence_scores(&self) -> Result<()> {
        tracing::info!("Starting nightly confidence adjustment...");

        // 1. Collect feedback events from last 24 hours
        let feedback_events = self.tdl.get_feedback_events_since(now() - Duration::days(1)).await?;

        // 2. Aggregate metrics per rule
        let mut metrics: HashMap<String, RuleConfidenceMetrics> = HashMap::new();
        for event in feedback_events {
            let entry = metrics.entry(event.rule_id.clone()).or_insert_with(|| {
                RuleConfidenceMetrics {
                    rule_id: event.rule_id.clone(),
                    ..Default::default()
                }
            });

            match event.event_type {
                FeedbackEventType::DiagnosticAccepted => entry.true_positives += 1,
                FeedbackEventType::FalsePositiveReported => entry.false_positives += 1,
                FeedbackEventType::DiagnosticDismissed => entry.dismissed_count += 1,
                _ => {}
            }

            if event.outcome.as_deref() == Some("success") {
                entry.applied_fixes += 1;
                entry.fix_success_rate += 1.0;
            }
        }

        // 3. Update rule severities based on confidence
        for (rule_id, metrics) in metrics {
            let new_confidence = metrics.calculate_confidence();
            let action = metrics.recommended_action();

            tracing::info!(
                "Rule {}: confidence={:.2}, action={:?}",
                rule_id,
                new_confidence,
                action
            );

            // Update rule in registry
            self.rule_registry.update_rule_confidence(&rule_id, new_confidence).await?;

            // Apply action (change severity, disable rule, etc.)
            match action {
                RuleAction::PromoteToError => {
                    self.rule_registry.set_severity(&rule_id, Severity::Error).await?;
                }
                RuleAction::DemoteToHint => {
                    self.rule_registry.set_severity(&rule_id, Severity::Hint).await?;
                }
                RuleAction::Disable => {
                    self.rule_registry.disable_rule(&rule_id).await?;
                }
                _ => {}
            }

            // Store metrics in KDB for learning
            self.kdb_manager.store_rule_metrics(&rule_id, &metrics).await?;

            // Emit event to Universe for observability
            let update_event = RuleConfidenceUpdateEvent {
                rule_id: rule_id.clone(),
                old_confidence: 0.0, // TODO: fetch previous
                new_confidence,
                action: action.to_string(),
                timestamp: now(),
            };
            self.emit_universe_event(update_event).await?;
        }

        tracing::info!("Confidence adjustment complete");
        Ok(())
    }
}
```

---

## 3. AI-Generated Rule Refinement

### 3.1 Rule Mutation & Optimization

When a rule has low confidence, the ETL can propose **mutations** to improve it:

```rust
// crates/bonsai-etl/src/rule_refiner.rs

pub struct RuleRefiner {
    ai_client: BonsaiAIClient,
    rule_registry: Arc<RuleRegistry>,
}

impl RuleRefiner {
    /// Propose mutations for low-confidence AI-generated rules
    pub async fn refine_low_confidence_rules(&self) -> Result<()> {
        // Find rules with 0.50 < confidence < 0.70
        let candidates = self.rule_registry.get_rules_by_confidence_range(0.50, 0.70).await?;

        for rule in candidates {
            if !rule.is_ai_generated {
                continue; // Only refine AI-generated rules
            }

            tracing::info!("Refining rule: {}", rule.id);

            // 1. Analyze false positives
            let false_positives = self.collect_false_positives(&rule.id).await?;
            
            // 2. Analyze true positives
            let true_positives = self.collect_true_positives(&rule.id).await?;

            // 3. Use AI to propose mutations
            let mutations = self.ai_client.propose_rule_mutations(
                &rule,
                &false_positives,
                &true_positives,
            ).await?;

            // 4. Evaluate each mutation on held-out test set
            for mutation in mutations {
                let test_accuracy = self.evaluate_mutation(&mutation).await?;
                
                if test_accuracy > rule.confidence + 0.05 {
                    // Improvement of 5%+ → propose the mutation
                    tracing::info!(
                        "Proposing mutation of rule {}: +{:.1}% accuracy",
                        rule.id,
                        (test_accuracy - rule.confidence) * 100.0
                    );

                    let proposal = RuleMutationProposal {
                        rule_id: rule.id.clone(),
                        original_pattern: rule.pattern.clone(),
                        mutated_pattern: mutation.pattern.clone(),
                        expected_improvement: test_accuracy - rule.confidence,
                        false_positive_examples: false_positives.clone(),
                        true_positive_examples: true_positives.clone(),
                        timestamp: now(),
                    };

                    // Store proposal for user review
                    self.store_proposal(&proposal).await?;
                    
                    // Emit to Universe
                    self.emit_universe_event(&proposal).await?;
                }
            }
        }

        Ok(())
    }

    async fn collect_false_positives(&self, rule_id: &str) -> Result<Vec<CodeExample>> {
        // Query TDL for code snippets flagged as false positives
        let feedback = self.tdl.get_feedback_by_rule(rule_id).await?;
        let false_positives = feedback
            .iter()
            .filter(|f| f.event_type == FeedbackEventType::FalsePositiveReported)
            .collect::<Vec<_>>();

        // Reconstruct code from file + line
        let mut examples = Vec::new();
        for fp in false_positives {
            if let Ok(code) = self.extract_code_context(&fp.file, fp.line).await {
                examples.push(CodeExample {
                    code,
                    is_positive: false,
                    context: fp.explanation.clone(),
                });
            }
        }

        Ok(examples)
    }

    async fn collect_true_positives(&self, rule_id: &str) -> Result<Vec<CodeExample>> {
        // Query TDL for code snippets where user accepted the diagnostic
        let feedback = self.tdl.get_feedback_by_rule(rule_id).await?;
        let true_positives = feedback
            .iter()
            .filter(|f| f.event_type == FeedbackEventType::DiagnosticAccepted)
            .collect::<Vec<_>>();

        let mut examples = Vec::new();
        for tp in true_positives {
            if let Ok(code) = self.extract_code_context(&tp.file, tp.line).await {
                examples.push(CodeExample {
                    code,
                    is_positive: true,
                    context: None,
                });
            }
        }

        Ok(examples)
    }

    async fn evaluate_mutation(&self, mutation: &RuleMutation) -> Result<f32> {
        // Test the mutated rule against held-out test set
        // Return accuracy score (0.0-1.0)
        todo!("Implement mutation evaluation")
    }
}
```

---

## 4. Integration with KDB and Survival KB

### 4.1 Rule Performance KDB Module

Create a new KDB module to store rule performance metrics:

```rust
// crates/bonsai-lint/src/integration/rule_performance_kdb.rs

pub struct RulePerformanceModule {
    kdb_manager: Arc<KdbManager>,
}

impl RulePerformanceModule {
    /// Store rule metrics in a persisted KDB module
    pub async fn store_rule_metrics(
        &self,
        rule_id: &str,
        metrics: &RuleConfidenceMetrics,
    ) -> Result<()> {
        let chunk = json!({
            "rule_id": rule_id,
            "confidence": metrics.calculate_confidence(),
            "true_positives": metrics.true_positives,
            "false_positives": metrics.false_positives,
            "dismissed": metrics.dismissed_count,
            "fix_success_rate": metrics.fix_success_rate,
            "last_updated": metrics.last_updated.to_rfc3339(),
            "recommended_action": metrics.recommended_action().to_string(),
        });

        // Insert into KDB module: rule-performance.kmod
        self.kdb_manager.upsert_chunk(
            "rule-performance",
            &format!("rule-{}", rule_id),
            &chunk.to_string(),
            metrics.calculate_confidence() as f32,
        ).await?;

        Ok(())
    }

    /// Retrieve rule metrics across projects
    pub async fn retrieve_rule_metrics(&self, rule_id: &str) -> Result<Vec<RuleConfidenceMetrics>> {
        let results = self.kdb_manager.semantic_search(
            "rule-performance",
            &format!("metrics for {}", rule_id),
            10,
        ).await?;

        let mut metrics = Vec::new();
        for result in results {
            if let Ok(m) = serde_json::from_str::<RuleConfidenceMetrics>(&result.content) {
                metrics.push(m);
            }
        }

        Ok(metrics)
    }
}

// .kmod file structure:
// rule-performance.kmod/
// ├── index.bin           (HNSW vector index)
// ├── chunks.jsonl        (rule metrics)
// │   rule-unused-variable: {"rule_id": "unused-variable", "confidence": 0.92, ...}
// │   rule-style-naming: {"rule_id": "style-naming", "confidence": 0.65, ...}
// │   ...
// └── metadata.json
```

### 4.2 Survival System Integration

When rules are disabled due to low confidence, the Survival System tracks the decision:

```rust
// In crates/bonsai-survival/src/rule_survival.rs

pub struct RuleSurvivalHandler {
    survival_kb: Arc<SurvivalKB>,
}

impl RuleSurvivalHandler {
    /// Log rule disabling for recovery
    pub async fn on_rule_disabled(
        &self,
        rule_id: &str,
        reason: &str,
        metrics: &RuleConfidenceMetrics,
    ) -> Result<()> {
        let incident = SurvivalIncident {
            incident_id: uuid::Uuid::new_v4().to_string(),
            component: "linter:rule-engine".to_string(),
            severity: "low".to_string(),
            description: format!(
                "Rule {} disabled: {} (confidence={:.2})",
                rule_id,
                reason,
                metrics.calculate_confidence()
            ),
            context: json!({
                "rule_id": rule_id,
                "true_positives": metrics.true_positives,
                "false_positives": metrics.false_positives,
                "reason": reason,
            }),
            recovery_action: "enable_rule_with_refined_pattern".to_string(),
            timestamp: now(),
        };

        self.survival_kb.log_incident(&incident).await?;
        Ok(())
    }

    /// When a rule mutation is proposed and accepted, enable it
    pub async fn on_rule_mutation_approved(
        &self,
        rule_id: &str,
        old_pattern: &str,
        new_pattern: &str,
    ) -> Result<()> {
        let recovery = SurvivalRecovery {
            incident_id: format!("rule-mutation-{}", uuid::Uuid::new_v4()),
            action: "update_rule_pattern",
            parameters: json!({
                "rule_id": rule_id,
                "old_pattern": old_pattern,
                "new_pattern": new_pattern,
            }),
            timestamp: now(),
            verified: true,
        };

        self.survival_kb.log_recovery(&recovery).await?;
        Ok(())
    }
}
```

---

## 5. Universe Event Schema

### 5.1 Feedback Events

```json
{
  "event_type": "bonsai:lint:feedback",
  "timestamp": "2026-06-01T14:23:45Z",
  "user_id": "user@company.com",
  "workspace": "my-project",
  "data": {
    "feedback_event_type": "diagnostic_accepted | diagnostic_dismissed | false_positive_reported | fix_applied",
    "rule_id": "unused-variable",
    "file": "src/main.rs",
    "line": 42,
    "column": 5,
    "severity": "warning",
    "message": "Variable 'x' is never used",
    "action": "applied_fix | manual_edit | ignored",
    "fix_type": "remove_variable",
    "outcome": "success | build_failed | test_failed | reverted",
    "explanation": "optional user explanation",
    "dismissal_count": 3,
    "timestamp": "2026-06-01T14:23:45Z"
  }
}
```

### 5.2 Rule Confidence Update Events

```json
{
  "event_type": "bonsai:lint:rule-confidence-updated",
  "timestamp": "2026-06-01T22:00:00Z",
  "data": {
    "rule_id": "unused-variable",
    "old_confidence": 0.85,
    "new_confidence": 0.92,
    "true_positives": 127,
    "false_positives": 3,
    "dismissed_count": 5,
    "action": "kept_as_warning | promoted_to_error | demoted_to_hint | disabled",
    "reason": "based on feedback from past 24 hours"
  }
}
```

### 5.3 Rule Mutation Proposal Events

```json
{
  "event_type": "bonsai:lint:rule-mutation-proposed",
  "timestamp": "2026-06-01T22:15:00Z",
  "data": {
    "rule_id": "complexity-too-high",
    "original_pattern": "function_length > 100",
    "mutated_pattern": "function_length > 150",
    "current_confidence": 0.58,
    "expected_confidence": 0.68,
    "false_positives": ["src/compute.rs:200", "src/algo.rs:450"],
    "approval_required": true,
    "approval_link": "/api/approve-mutation/rule-mutation-uuid"
  }
}
```

---

## 6. MCP Tools and CLI Commands

### 6.1 MCP Tools for Rule Review

Register these tools in `bonsai-mcp-server`:

```json
{
  "name": "bonsai_review_rule_mutations",
  "description": "Review and approve proposed rule mutations from ETL",
  "inputSchema": {
    "type": "object",
    "properties": {
      "filter": {
        "type": "string",
        "description": "Filter by rule_id or 'all' for all pending"
      },
      "action": {
        "type": "string",
        "enum": ["list", "approve", "reject"],
        "description": "Action to take"
      },
      "mutation_id": {
        "type": "string",
        "description": "ID of mutation to approve/reject"
      },
      "feedback": {
        "type": "string",
        "description": "Feedback for rejected mutations"
      }
    },
    "required": ["filter"]
  }
}
```

Handler:

```rust
// crates/bonsai-mcp-server/src/lint_rule_reviewer.rs

pub async fn handle_review_rule_mutations(
    filter: &str,
    action: &str,
    mutation_id: Option<&str>,
    feedback: Option<&str>,
) -> Result<Value> {
    match action {
        "list" => {
            // Get pending mutations
            let mutations = get_pending_mutations(filter).await?;
            Ok(json!({
                "pending_mutations": mutations,
                "count": mutations.len(),
            }))
        }
        "approve" => {
            let mut_id = mutation_id.ok_or_else(|| anyhow!("mutation_id required"))?;
            approve_mutation(mut_id).await?;
            Ok(json!({
                "status": "approved",
                "mutation_id": mut_id,
            }))
        }
        "reject" => {
            let mut_id = mutation_id.ok_or_else(|| anyhow!("mutation_id required"))?;
            reject_mutation(mut_id, feedback).await?;
            Ok(json!({
                "status": "rejected",
                "mutation_id": mut_id,
            }))
        }
        _ => Err(anyhow!("Invalid action")),
    }
}
```

### 6.2 CLI Commands

```bash
# List pending rule mutations
$ bonsai lint rule-mutations list

Pending Rule Mutations:
  1. unused-variable
     Pattern: "let \w+ =" → "let _ =" (for unused vars)
     Confidence improvement: 0.68 → 0.82 (+20%)
     False positives reduced: 15 → 3
     Status: PENDING_APPROVAL

  2. style-naming
     Pattern: "fn [A-Z]" → "fn [a-z_]+"
     Confidence improvement: 0.55 → 0.71 (+29%)
     False positives reduced: 42 → 8
     Status: PENDING_APPROVAL

# Approve a mutation
$ bonsai lint rule-mutations approve unused-variable
✓ Mutation approved. Rule will be updated in next nightly ETL run.

# View rule confidence history
$ bonsai lint rule-confidence-history unused-variable

Rule: unused-variable
  2026-06-01: confidence 0.92 (127 TP, 3 FP, 5 dismissed)
  2026-05-31: confidence 0.85 (110 TP, 8 FP, 12 dismissed)
  2026-05-30: confidence 0.78 (98 TP, 15 FP, 20 dismissed)
  [Trend: Improving +7% over 3 days]

# View team-wide rule insights
$ bonsai lint rule-insights --team

Rules by Accuracy:
  1. unused-variable (92% TP, 3% FP)    ← High confidence, keep as error
  2. style-naming (71% TP, 8% FP)       ← Medium confidence, hint level
  3. complexity-too-high (42% TP, 42% FP) ← Disabled, needs refinement
```

---

## 7. Implementation Steps

### Phase A.1: Feedback Collection (Week 1)

**Objective:** Capture user actions as training signals

**Steps:**
1. Create `FeedbackCollector` struct
2. Add feedback hooks to LintPanel.svelte
3. Wire feedback events to Universe + TDL
4. Create TDL schema for feedback storage
5. Tests: 5 unit tests for feedback capture

**Deliverables:**
- `crates/bonsai-lint/src/integration/feedback_collector.rs` (200 LOC)
- Updated `bonsai-workspace/src/lib/components/LintPanel.svelte` (100 LOC)
- TDL feedback table schema

---

### Phase A.2: Confidence Scoring (Week 1-2)

**Objective:** Calculate dynamic rule confidence from historical feedback

**Steps:**
1. Create `RuleConfidenceMetrics` struct
2. Implement confidence calculation algorithm
3. Wire into rule registry
4. Add tests for edge cases (no feedback, perfect accuracy, 100% false positives)

**Deliverables:**
- `crates/bonsai-lint/src/engine/rule_confidence.rs` (250 LOC)
- Unit tests (8 tests)
- Design doc: Confidence Algorithm

---

### Phase A.3: ETL Integration (Week 2-3)

**Objective:** Nightly ETL job adjusts rule confidence and proposes mutations

**Steps:**
1. Create `RuleConfidenceAdjuster` in `bonsai-etl`
2. Implement confidence update loop
3. Query TDL for feedback events
4. Update rule severities based on confidence
5. Create ETL cron job
6. Tests: 6 unit tests

**Deliverables:**
- `crates/bonsai-etl/src/rule_confidence_adjuster.rs` (300 LOC)
- ETL scheduler integration

---

### Phase A.4: Rule Refinement (Week 3-4)

**Objective:** AI-generated rules are mutated and refined

**Steps:**
1. Create `RuleRefiner` struct
2. Integrate with BonsAI client
3. Implement mutation evaluation
4. Create mutation proposal storage
5. Tests: 4 unit tests

**Deliverables:**
- `crates/bonsai-etl/src/rule_refiner.rs` (350 LOC)
- Mutation evaluation framework

---

### Phase A.5: KDB & Survival Integration (Week 4)

**Objective:** Rule metrics persist across restarts and are shared

**Steps:**
1. Create `RulePerformanceModule` for KDB storage
2. Integrate with Survival System
3. Create rule-performance.kmod template
4. Wire incident logging

**Deliverables:**
- `crates/bonsai-lint/src/integration/rule_performance_kdb.rs` (200 LOC)
- `crates/bonsai-survival/src/rule_survival.rs` (150 LOC)

---

### Phase A.6: MCP Tools & CLI (Week 4-5)

**Objective:** Humans can review and approve learned rules

**Steps:**
1. Create `lint_rule_reviewer.rs` MCP handlers
2. Implement CLI commands
3. Add approval workflow
4. Wire to rule registry updates
5. Tests: 3 integration tests

**Deliverables:**
- `crates/bonsai-mcp-server/src/lint_rule_reviewer.rs` (250 LOC)
- CLI commands (in bonsai-cli)
- MCP tool definitions

---

### Phase A.7: Universe & Observability (Week 5)

**Objective:** All events are logged and observable

**Steps:**
1. Define Universe event schemas
2. Add event emission to all ETL components
3. Create observability dashboard design
4. Tests: 2 integration tests

**Deliverables:**
- Universe event schema (JSON)
- Dashboard wireframes
- Event emission in all modules

---

## 8. Data Structures

### 8.1 Rule Confidence Metrics

```rust
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleConfidenceMetrics {
    pub rule_id: String,
    pub true_positives: u32,       // User accepted
    pub false_positives: u32,      // User rejected
    pub dismissed_count: u32,      // User ignored
    pub applied_fixes: u32,
    pub fix_success_rate: f32,     // (0.0-1.0)
    pub last_updated: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct FeedbackEvent {
    pub event_id: String,
    pub event_type: FeedbackEventType,
    pub rule_id: String,
    pub file: String,
    pub line: u32,
    pub column: u32,
    pub timestamp: DateTime<Utc>,
    pub user_id: String,
    pub action: Option<String>,
    pub outcome: Option<String>,
    pub explanation: Option<String>,
    pub dismissal_count: Option<u32>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum FeedbackEventType {
    DiagnosticRendered,
    DiagnosticAccepted,
    DiagnosticDismissed,
    FalsePositiveReported,
    FixApplied,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RuleMutationProposal {
    pub proposal_id: String,
    pub rule_id: String,
    pub original_pattern: String,
    pub mutated_pattern: String,
    pub expected_improvement: f32, // (0.0-1.0)
    pub false_positive_examples: Vec<CodeExample>,
    pub true_positive_examples: Vec<CodeExample>,
    pub timestamp: DateTime<Utc>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct CodeExample {
    pub code: String,
    pub is_positive: bool,
    pub context: Option<String>,
}
```

---

## 9. Architecture Diagram

```
┌─────────────────────────────────────────────────────────────────┐
│                     BUL Self-Improving Loop                     │
└─────────────────────────────────────────────────────────────────┘

User Actions (IDE)
    ↓
    • Accept diagnostic → apply fix
    • Reject as false positive
    • Ignore/dismiss warning
    • Manually edit code
    ↓
FeedbackCollector (Real-time)
    ├→ Universe (observability)
    ├→ TDL (training data)
    └→ ETL Queue
    ↓
EternalTrainingLoop (Nightly, 22:00 UTC)
    ├─ Stage 1: Collect Feedback Events
    │  └→ Query TDL for past 24h events
    │
    ├─ Stage 2: Aggregate Metrics
    │  └→ Calculate confidence per rule
    │
    ├─ Stage 3: Adjust Severities
    │  ├→ High confidence (>0.85) → Error
    │  ├→ Normal (0.70-0.85) → Warning
    │  ├→ Low (0.50-0.70) → Hint
    │  └→ Very low (<0.50) → Disabled
    │
    ├─ Stage 4: Refine AI-Generated Rules
    │  ├→ Analyze false positives
    │  ├→ Propose mutations via BonsAI
    │  └→ Evaluate mutations
    │
    └─ Stage 5: Store & Emit Events
       ├→ Store metrics in KDB
       ├→ Log to Survival KB
       ├→ Emit Universe events
       └→ Alert for mutation approvals
    ↓
Human Review (Optional)
    • Review proposed mutations
    • Approve/reject mutations
    • Provide feedback on false positives
    ↓
Rule Registry Update
    • Update severities
    • Apply approved mutations
    • Disable low-confidence rules
    ↓
Next Lint Run
    • Use updated rules
    • Improved accuracy
    • Fewer false positives
    ↓
Cycle repeats...
```

---

## 10. Integration Points Summary

| Component | Integration | Purpose |
|-----------|-----------|---------|
| **LintPanel.svelte** | FeedbackCollector hooks | Capture user actions |
| **Universe** | FeedbackEvent emission | Observability & history |
| **TDL** | FeedbackEvent storage | Training data repository |
| **KDB** | Rule metrics storage | Persistence across projects |
| **Survival KB** | Incident logging | Recovery & audit trail |
| **BonsAI** | Mutation proposal | ML-driven refinement |
| **Rule Registry** | Confidence updates | Dynamic rule adjustment |
| **MCP Server** | Rule reviewer tools | Human approval workflow |
| **CLI** | Rule commands | Team insights & metrics |

---

## 11. Success Metrics

After Phase A deployment, measure:

1. **Reduced False Positive Rate** – Track ratio of true positives / (true positives + false positives)
   - **Target:** <5% false positive rate after 30 days

2. **Rule Confidence Convergence** – Rules with stable, high confidence (>0.85)
   - **Target:** 80% of active rules at confidence >0.85

3. **Mutation Impact** – Improvement from approved mutations
   - **Target:** Each mutation improves accuracy by >10%

4. **User Engagement** – Feedback events per user per day
   - **Target:** 5+ feedback actions per active user per day

5. **Time to Accuracy** – Days for rules to reach stable confidence
   - **Target:** <30 days for new rules, <14 days for mutations

---

## 12. Rollout Strategy

### Phase A Rollout Timeline

**Week 1-2: Alpha** (Internal team)
- Deploy feedback collector
- Collect baseline metrics
- Iterate on confidence algorithm

**Week 3-4: Beta** (Early adopters)
- ETL + rule refinement
- Gather mutation feedback
- Refine approval workflow

**Week 5: General Availability**
- All features enabled
- Dashboard published
- Documentation & training

### Backwards Compatibility

- ✅ Feedback collection is non-breaking
- ✅ Disabled rules still show in history
- ✅ Original rule definitions are preserved
- ✅ Mutations create new rule versions

---

## Conclusion

Phase A transforms BUL from a **static linter into a learning system** that improves every day based on real-world usage. The feedback loop is closed: every user action makes the linter smarter.

**Key Achievement:** Rules automatically adapt to your codebase and team conventions without manual tuning.

**Next Phases Enabled:**
- Phase B: Persistent Salsa + Collaborative linting
- Phase C: Axiom-verified rules + Predictive linting
- Phase D: Grammar checking + Plugin marketplace
- Phase E: Formal SLAs + Auto-calibration

---

**Ready to build. 🚀**
