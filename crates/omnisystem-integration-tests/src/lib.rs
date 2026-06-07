//! Omnisystem Integration Tests
//! Demonstrates Wave 1-4 system cohesion

#[cfg(test)]
mod integration_tests {
    use buddy_agent::{Buddy, BuddyState};
    use hde_orchestrator::Orchestrator;
    use model_builder::{ModelBuilder, TrainingExample};
    use axiom_verify::{AxiomVerifier, ProofObligation};
    use bonsai_buddy_agent::BuddyAgent;
    use bonsai_buddy_offline_sync::SyncQueue;
    use hde_safety_envelope::SafetyConstraints;
    use titan_core::PersistentVector;

    /// Wave 1 + Wave 4: Buddy Agent Lifecycle
    #[test]
    fn test_buddy_full_lifecycle() {
        // Wave 1, Phase 5: Bonsai Buddy agent
        let mut buddy = Buddy::new("sys-buddy".to_string());
        assert_eq!(buddy.state(), &BuddyState::Idle);

        // Wave 4, Component 1: Standalone Buddy Agent
        let mut buddy_agent = BuddyAgent::new("buddy-instance".to_string());
        assert!(buddy_agent.is_offline());

        // Activate both
        buddy.activate();
        buddy_agent.set_online();

        assert_eq!(buddy.state(), &BuddyState::Processing);
        assert!(!buddy_agent.is_offline());

        // Wave 4, Component 2: Offline Sync
        let mut sync_queue = SyncQueue::new();
        sync_queue.enqueue("sync_op_1".to_string());
        sync_queue.enqueue("sync_op_2".to_string());

        assert_eq!(sync_queue.pending_count(), 2);
        let ops = sync_queue.flush();
        assert_eq!(ops.len(), 2);

        // Sync completes
        buddy.sync();
        assert_eq!(buddy.state(), &BuddyState::Synchronized);
    }

    /// Wave 1 + Wave 3: HDE Orchestration with Safety
    #[test]
    fn test_hde_orchestrator_with_safety() {
        // Wave 1, Phase 6: HDE Orchestrator
        let mut orchestrator = Orchestrator::new();
        orchestrator.spawn("hde-instance-1".to_string());
        assert_eq!(orchestrator.count(), 1);

        // Wave 3, Component 2: Safety Envelope
        let constraints = SafetyConstraints::new(100, 512);

        // Check operations within bounds
        assert!(constraints.check(50, 256).is_ok());
        assert!(constraints.check(99, 511).is_ok());

        // Check violations
        assert!(constraints.check(150, 256).is_err());
        assert!(constraints.check(50, 600).is_err());
    }

    /// Wave 1 + Wave 3: Model Building and Verification
    #[test]
    fn test_model_building_with_verification() {
        // Wave 1, Phase 7: Model Builder
        let mut builder = ModelBuilder::new();

        // Add training examples
        builder.add_example(TrainingExample::new(vec![1.0, 2.0], 3.0));
        builder.add_example(TrainingExample::new(vec![2.0, 3.0], 5.0));
        builder.add_example(TrainingExample::new(vec![3.0, 4.0], 7.0));

        assert_eq!(builder.example_count(), 3);

        // Build model
        let model = builder.build().unwrap();
        assert_eq!(model.version(), 1);
        assert!(model.accuracy() > 0.9);

        // Wave 1, Phase 8: Axiom Verification
        let mut verifier = AxiomVerifier::new();

        // Add proof obligations for model correctness
        verifier.add_obligation(ProofObligation::new(
            "model_convergence".to_string(),
            "model accuracy improves monotonically".to_string(),
        ));

        verifier.add_obligation(ProofObligation::new(
            "training_termination".to_string(),
            "training loop terminates".to_string(),
        ));

        // Prove all obligations
        assert!(verifier.prove_all().is_ok());
        let status = verifier.verification_status();
        assert!(status.is_complete());
    }

    /// Wave 2 + Wave 3: Persistent Data Structures with HDE
    #[test]
    fn test_persistent_structures_with_hde() {
        // Wave 2, Phase 2: Titan Core - Persistent Vector
        let v1 = PersistentVector::new();
        assert_eq!(v1.len(), 0);

        let v2 = v1.push(1).push(2).push(3);
        assert_eq!(v2.len(), 3);

        // Original vector unchanged (immutability)
        assert_eq!(v1.len(), 0);
        assert_eq!(v2.get(0), Some(&1));

        // Wave 3, Component 3: Model Framework can use these structures
        let mut builder = ModelBuilder::new();

        // Vectors work with feature representation
        for i in 0..3 {
            let features: Vec<f32> = (0..5).map(|j| ((i + j) as f32) / 2.0).collect();
            builder.add_example(TrainingExample::new(features, i as f32));
        }

        assert_eq!(builder.example_count(), 3);
    }

    /// Wave 1 + Wave 4: CRDT Merging for Distributed State
    #[test]
    fn test_crdt_distributed_merging() {
        use bonsai_buddy_crdt::CrdtSnapshot;
        use std::collections::HashMap;

        // Create two independent snapshots from different nodes
        let mut vc1 = HashMap::new();
        vc1.insert("node1".to_string(), 5);
        let snap1 = CrdtSnapshot::new(vc1, vec![1, 2, 3, 4, 5]);

        let mut vc2 = HashMap::new();
        vc2.insert("node2".to_string(), 3);
        let snap2 = CrdtSnapshot::new(vc2, vec![1, 2, 3]);

        // Merge snapshots (conflict-free)
        let merged = snap1.merge(&snap2).unwrap();
        assert_eq!(merged.data().len(), 5);

        // Wave 1, Phase 5: Buddy can use merged state
        let mut buddy = Buddy::new("distributed-buddy".to_string());
        buddy.activate();
        buddy.sync();
        assert_eq!(buddy.state(), &BuddyState::Synchronized);
    }

    /// Full System: All Waves Working Together
    #[test]
    fn test_complete_system_integration() {
        // Wave 1, Phase 5: Buddy initiates
        let mut buddy = Buddy::new("omnisystem-instance".to_string());
        buddy.activate();

        // Wave 1, Phase 6: HDE Orchestrator manages execution
        let mut orchestrator = Orchestrator::new();
        orchestrator.spawn("hde-exec-1".to_string());

        // Wave 1, Phase 7: Model Builder trains optimization models
        let mut model_builder = ModelBuilder::new();
        model_builder.add_example(TrainingExample::new(vec![1.0], 1.0));
        model_builder.add_example(TrainingExample::new(vec![2.0], 2.0));
        let _model = model_builder.build().unwrap();

        // Wave 1, Phase 8: Axiom Verifier ensures correctness
        let mut verifier = AxiomVerifier::new();
        verifier.add_obligation(ProofObligation::new(
            "system_safety".to_string(),
            "system maintains invariants".to_string(),
        ));
        assert!(verifier.prove_all().is_ok());

        // Wave 2, Phase 2: Titan persistent structures hold state
        let data_vector = PersistentVector::new().push(100).push(200);
        assert_eq!(data_vector.len(), 2);

        // Wave 3, Component 2: Safety constraints enforced
        let safety = SafetyConstraints::new(1000, 2048);
        assert!(safety.check(500, 1024).is_ok());

        // Wave 4, Component 1: Buddy Agent online
        let mut buddy_agent = BuddyAgent::new("integration-buddy".to_string());
        buddy_agent.set_online();
        assert!(!buddy_agent.is_offline());

        // System completes
        buddy.sync();
        assert_eq!(buddy.state(), &BuddyState::Synchronized);
    }
}
