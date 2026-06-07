//! Omnisystem Examples
//! Demonstrates common usage patterns and architectural idioms

use titan_core::PersistentVector;
use buddy_agent::{Buddy, BuddyState};
use hde_runtime::HdeRuntime;
use model_builder::{ModelBuilder, TrainingExample};
use axiom_verify::{AxiomVerifier, ProofObligation};
use bonsai_buddy_crdt::CrdtSnapshot;
use std::collections::HashMap;

/// Example 1: Building a Persistent Data Structure
///
/// Shows how to use Titan's immutable vectors for functional programming style.
pub fn example_persistent_vector() {
    let v1 = PersistentVector::<i32>::new();

    // Build up values functionally - original unchanged
    let v2 = v1.push(1).push(2).push(3);

    // Can use both versions independently
    assert_eq!(v1.len(), 0);  // Original unchanged
    assert_eq!(v2.len(), 3);  // New version has data

    // Structural sharing means O(log32 n) operations
    let v3 = v2.push(4);
    assert_eq!(v2.len(), 3);
    assert_eq!(v3.len(), 4);
}

/// Example 2: Building a Safe Service Execution
///
/// Shows how to set up HDE runtime with safety constraints and AI advisor.
pub fn example_hde_execution() {
    // Create runtime with 100ms latency and 512MB memory budget
    let mut runtime = HdeRuntime::new(100, 512);

    // Set up AI advisor context if available
    let context = hde_ai_advisor::AdvisoryContext::new("opt-model-v1".to_string());
    runtime.set_advisor_context(context);

    // Execute with safety constraints
    let baseline = vec![1, 2, 3, 4, 5];
    let result = runtime.execute(baseline, 50, 256);

    match result {
        hde_runtime::ExecutionResult::Success(output) => {
            println!("Execution succeeded with optimized output: {:?}", output);
        }
        hde_runtime::ExecutionResult::SafetyViolation(msg) => {
            println!("Execution rejected: {}", msg);
        }
        hde_runtime::ExecutionResult::ValidationFailure(msg) => {
            println!("Validation failed: {}", msg);
        }
    }
}

/// Example 3: Building and Verifying Models
///
/// Shows end-to-end model training and formal verification.
pub fn example_model_verification() {
    // Build a model from training data
    let mut builder = ModelBuilder::new();

    // Add training examples
    for i in 1..=5 {
        let features = vec![i as f32];
        let label = (i as f32) * 2.0;
        builder.add_example(TrainingExample::new(features, label));
    }

    // Train the model
    let model = builder.build().expect("Model training failed");
    println!("Model version {}: accuracy {:.2}%",
             model.version(),
             model.accuracy() * 100.0);

    // Formally verify the model with Axiom
    let mut verifier = AxiomVerifier::new();

    // Add proof obligations for model properties
    verifier.add_obligation(ProofObligation::new(
        "training_convergence".to_string(),
        "model accuracy improves with training".to_string(),
    ));

    verifier.add_obligation(ProofObligation::new(
        "error_bounds".to_string(),
        "prediction error stays bounded".to_string(),
    ));

    // Prove all obligations
    if verifier.prove_all().is_ok() {
        let status = verifier.verification_status();
        println!("Verification complete: {} of {} obligations proven",
                 status.proven, status.total);
    }
}

/// Example 4: Multi-Agent Coordination
///
/// Shows how buddies coordinate work in a distributed system.
pub fn example_buddy_coordination() {
    // Create two buddy instances
    let mut buddy_primary = Buddy::new("primary-buddy".to_string());
    let mut buddy_backup = Buddy::new("backup-buddy".to_string());

    // Primary takes lead
    buddy_primary.activate();
    assert_eq!(buddy_primary.state(), &BuddyState::Processing);

    // Backup stays idle
    assert_eq!(buddy_backup.state(), &BuddyState::Idle);

    // When primary completes and syncs
    buddy_primary.sync();
    assert_eq!(buddy_primary.state(), &BuddyState::Synchronized);

    // Backup can take over for next phase
    buddy_backup.activate();
    assert_eq!(buddy_backup.state(), &BuddyState::Processing);
}

/// Example 5: Distributed State Merging with CRDT
///
/// Shows how to merge state from multiple nodes without conflicts.
pub fn example_crdt_merging() {
    // Create snapshots from two independent nodes
    let mut vc_node1 = HashMap::new();
    vc_node1.insert("node1".to_string(), 10);
    vc_node1.insert("shared".to_string(), 1);

    let snap_node1 = CrdtSnapshot::new(
        vc_node1,
        vec![1, 2, 3, 4, 5]
    );

    let mut vc_node2 = HashMap::new();
    vc_node2.insert("node2".to_string(), 8);
    vc_node2.insert("shared".to_string(), 2);

    let snap_node2 = CrdtSnapshot::new(
        vc_node2,
        vec![6, 7, 8, 9, 10]
    );

    // Merge without conflicts (CRDT guarantees)
    match snap_node1.merge(&snap_node2) {
        Ok(merged) => {
            println!("Merged state: {:?}", merged.data());
        }
        Err(e) => {
            println!("Merge failed: {}", e);
        }
    }
}

/// Example 6: Complex Pipeline
///
/// Shows a realistic scenario: load data → optimize → verify → commit
pub fn example_optimization_pipeline() {
    println!("=== Omnisystem Optimization Pipeline ===\n");

    // Step 1: Load baseline data
    println!("1. Loading baseline data...");
    let baseline_data = vec![100u8, 200, 150, 175, 125];
    println!("   Baseline: {:?}", baseline_data);

    // Step 2: Set up HDE runtime
    println!("\n2. Setting up HDE runtime...");
    let hde = HdeRuntime::new(200, 1024);
    println!("   Runtime ready (200ms latency, 1GB memory)");

    // Step 3: Train optimization model
    println!("\n3. Training optimization model...");
    let mut builder = ModelBuilder::new();
    builder.add_example(TrainingExample::new(vec![100.0], 95.0));
    builder.add_example(TrainingExample::new(vec![150.0], 140.0));
    builder.add_example(TrainingExample::new(vec![200.0], 190.0));

    let model = builder.build().expect("Training failed");
    println!("   Model trained: v{} accuracy {:.1}%",
             model.version(), model.accuracy() * 100.0);

    // Step 4: Execute optimization
    println!("\n4. Executing optimization under safety constraints...");
    let execution_latency = 75;
    let execution_memory = 512;

    match hde.execute(baseline_data, execution_latency, execution_memory) {
        hde_runtime::ExecutionResult::Success(optimized) => {
            println!("   ✓ Optimization succeeded");
            println!("   Original: [100, 200, 150, 175, 125]");
            println!("   Optimized: {:?}", optimized);
        }
        hde_runtime::ExecutionResult::SafetyViolation(msg) => {
            println!("   ✗ Safety violation: {}", msg);
        }
        hde_runtime::ExecutionResult::ValidationFailure(msg) => {
            println!("   ✗ Validation failed: {}", msg);
        }
    }

    // Step 5: Formal verification
    println!("\n5. Formal verification with Axiom...");
    let mut verifier = AxiomVerifier::new();
    verifier.add_obligation(ProofObligation::new(
        "optimization_safe".to_string(),
        "optimization maintains safety properties".to_string(),
    ));

    if verifier.prove_all().is_ok() {
        let status = verifier.verification_status();
        if status.is_complete() {
            println!("   ✓ All {} obligations proven", status.total);
        }
    }

    println!("\n=== Pipeline Complete ===");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_example_persistent_vector() {
        example_persistent_vector();
    }

    #[test]
    fn test_example_hde_execution() {
        example_hde_execution();
    }

    #[test]
    fn test_example_model_verification() {
        example_model_verification();
    }

    #[test]
    fn test_example_buddy_coordination() {
        example_buddy_coordination();
    }

    #[test]
    fn test_example_crdt_merging() {
        example_crdt_merging();
    }

    #[test]
    fn test_example_optimization_pipeline() {
        example_optimization_pipeline();
    }
}
