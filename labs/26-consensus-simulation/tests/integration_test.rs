// Lab 26: Consensus Simulation - Integration Tests
//
// Tests the distributed consensus simulation including:
// - Node creation and behavior (honest vs faulty)
// - Proposal processing rules
// - Consensus coordinator with all honest nodes
// - Consensus coordinator with mixed honest/faulty nodes
// - Consensus result counting and majority detection
// - Byzantine fault tolerance calculations
// - Message types and structure
// - Edge cases (single node, all faulty, boundary values)

use consensus_simulation::solution::{
    byzantine_tolerance, is_byzantine_safe, ConsensusCoordinator, Message, Node, NodeType,
};

// ============================================================================
// HELPER FUNCTIONS
// ============================================================================

/// Creates a vector of honest nodes with sequential IDs starting at 0.
fn make_honest_nodes(count: usize) -> Vec<Node> {
    (0..count)
        .map(|id| Node::new(id, NodeType::Honest))
        .collect()
}

/// Creates a vector of nodes where the first `num_faulty` are faulty
/// (with the given acceptance behavior) and the rest are honest.
fn make_mixed_nodes(total: usize, num_faulty: usize, faulty_accepts: bool) -> Vec<Node> {
    let mut nodes = Vec::new();
    for id in 0..total {
        if id < num_faulty {
            nodes.push(Node::new_faulty(id, faulty_accepts));
        } else {
            nodes.push(Node::new(id, NodeType::Honest));
        }
    }
    nodes
}

// ============================================================================
// NODE CREATION TESTS
// ============================================================================

#[test]
fn test_node_new_honest() {
    let node = Node::new(0, NodeType::Honest);
    assert_eq!(node.id, 0);
    assert_eq!(node.node_type, NodeType::Honest);
}

#[test]
fn test_node_new_faulty() {
    let node = Node::new(1, NodeType::Faulty);
    assert_eq!(node.id, 1);
    assert_eq!(node.node_type, NodeType::Faulty);
}

#[test]
fn test_node_new_faulty_with_accepts() {
    let node = Node::new_faulty(2, true);
    assert_eq!(node.id, 2);
    assert_eq!(node.node_type, NodeType::Faulty);
    assert!(node.faulty_accepts);
}

#[test]
fn test_node_new_faulty_with_rejects() {
    let node = Node::new_faulty(3, false);
    assert_eq!(node.id, 3);
    assert_eq!(node.node_type, NodeType::Faulty);
    assert!(!node.faulty_accepts);
}

// ============================================================================
// HONEST NODE PROPOSAL PROCESSING TESTS
// ============================================================================

#[test]
fn test_honest_node_accepts_small_positive_value() {
    let node = Node::new(0, NodeType::Honest);
    assert!(node.process_proposal(1, 1));
}

#[test]
fn test_honest_node_accepts_typical_value() {
    let node = Node::new(0, NodeType::Honest);
    assert!(node.process_proposal(1, 42));
    assert!(node.process_proposal(1, 100));
    assert!(node.process_proposal(1, 500));
}

#[test]
fn test_honest_node_accepts_max_valid_value() {
    let node = Node::new(0, NodeType::Honest);
    assert!(node.process_proposal(1, 999));
}

#[test]
fn test_honest_node_rejects_zero() {
    let node = Node::new(0, NodeType::Honest);
    assert!(!node.process_proposal(1, 0));
}

#[test]
fn test_honest_node_rejects_negative_values() {
    let node = Node::new(0, NodeType::Honest);
    assert!(!node.process_proposal(1, -1));
    assert!(!node.process_proposal(1, -100));
    assert!(!node.process_proposal(1, i32::MIN));
}

#[test]
fn test_honest_node_rejects_boundary_1000() {
    let node = Node::new(0, NodeType::Honest);
    assert!(!node.process_proposal(1, 1000));
}

#[test]
fn test_honest_node_rejects_large_values() {
    let node = Node::new(0, NodeType::Honest);
    assert!(!node.process_proposal(1, 1001));
    assert!(!node.process_proposal(1, 10000));
    assert!(!node.process_proposal(1, i32::MAX));
}

#[test]
fn test_honest_node_same_behavior_across_rounds() {
    let node = Node::new(0, NodeType::Honest);
    // Honest nodes don't care about the round number
    assert!(node.process_proposal(1, 42));
    assert!(node.process_proposal(2, 42));
    assert!(node.process_proposal(100, 42));
}

// ============================================================================
// FAULTY NODE PROPOSAL PROCESSING TESTS
// ============================================================================

#[test]
fn test_faulty_node_accepts_when_configured() {
    let node = Node::new_faulty(0, true);
    assert!(node.process_proposal(1, 42));
    // Faulty node also accepts invalid values when configured to accept
    assert!(node.process_proposal(1, -1));
    assert!(node.process_proposal(1, 0));
    assert!(node.process_proposal(1, 5000));
}

#[test]
fn test_faulty_node_rejects_when_configured() {
    let node = Node::new_faulty(0, false);
    assert!(!node.process_proposal(1, 42));
    // Faulty node rejects even valid values when configured to reject
    assert!(!node.process_proposal(1, 100));
    assert!(!node.process_proposal(1, 999));
}

#[test]
fn test_faulty_node_ignores_value_validity() {
    // A faulty accepting node accepts even invalid values
    let accepting = Node::new_faulty(0, true);
    assert!(accepting.process_proposal(1, -999));

    // A faulty rejecting node rejects even valid values
    let rejecting = Node::new_faulty(1, false);
    assert!(!rejecting.process_proposal(1, 42));
}

// ============================================================================
// CONSENSUS WITH ALL HONEST NODES TESTS
// ============================================================================

#[test]
fn test_consensus_all_honest_valid_value() {
    let coordinator = ConsensusCoordinator::new(1, 5, 42);
    let nodes = make_honest_nodes(5);
    let result = coordinator.run(nodes);

    assert!(result.consensus_reached);
    assert_eq!(result.yes_votes, 5);
    assert_eq!(result.total_votes, 5);
    assert_eq!(result.proposed_value, 42);
    assert_eq!(result.round, 1);
}

#[test]
fn test_consensus_all_honest_reject_invalid_value() {
    // Value 0 is rejected by honest nodes
    let coordinator = ConsensusCoordinator::new(1, 5, 0);
    let nodes = make_honest_nodes(5);
    let result = coordinator.run(nodes);

    assert!(!result.consensus_reached);
    assert_eq!(result.yes_votes, 0);
    assert_eq!(result.total_votes, 5);
}

#[test]
fn test_consensus_all_honest_reject_negative_value() {
    let coordinator = ConsensusCoordinator::new(1, 3, -10);
    let nodes = make_honest_nodes(3);
    let result = coordinator.run(nodes);

    assert!(!result.consensus_reached);
    assert_eq!(result.yes_votes, 0);
}

#[test]
fn test_consensus_all_honest_reject_too_large_value() {
    let coordinator = ConsensusCoordinator::new(1, 4, 1000);
    let nodes = make_honest_nodes(4);
    let result = coordinator.run(nodes);

    assert!(!result.consensus_reached);
    assert_eq!(result.yes_votes, 0);
}

#[test]
fn test_consensus_all_honest_different_rounds() {
    for round in 1..=5 {
        let coordinator = ConsensusCoordinator::new(round, 3, 100);
        let nodes = make_honest_nodes(3);
        let result = coordinator.run(nodes);

        assert!(result.consensus_reached);
        assert_eq!(result.round, round);
    }
}

// ============================================================================
// CONSENSUS WITH FAULTY NODES TESTS
// ============================================================================

#[test]
fn test_consensus_one_faulty_rejecting_out_of_five() {
    // 1 faulty (rejecting) + 4 honest = 4/5 yes = consensus
    let coordinator = ConsensusCoordinator::new(1, 5, 42);
    let nodes = make_mixed_nodes(5, 1, false);
    let result = coordinator.run(nodes);

    assert!(result.consensus_reached);
    assert_eq!(result.yes_votes, 4);
    assert_eq!(result.total_votes, 5);
}

#[test]
fn test_consensus_two_faulty_rejecting_out_of_five() {
    // 2 faulty (rejecting) + 3 honest = 3/5 yes = consensus (majority)
    let coordinator = ConsensusCoordinator::new(1, 5, 42);
    let nodes = make_mixed_nodes(5, 2, false);
    let result = coordinator.run(nodes);

    assert!(result.consensus_reached);
    assert_eq!(result.yes_votes, 3);
    assert_eq!(result.total_votes, 5);
}

#[test]
fn test_consensus_fails_with_majority_faulty_rejecting() {
    // 3 faulty (rejecting) + 2 honest = 2/5 yes = no consensus
    let coordinator = ConsensusCoordinator::new(1, 5, 42);
    let nodes = make_mixed_nodes(5, 3, false);
    let result = coordinator.run(nodes);

    assert!(!result.consensus_reached);
    assert_eq!(result.yes_votes, 2);
    assert_eq!(result.total_votes, 5);
}

#[test]
fn test_consensus_all_faulty_rejecting() {
    let coordinator = ConsensusCoordinator::new(1, 4, 42);
    let mut nodes = Vec::new();
    for id in 0..4 {
        nodes.push(Node::new_faulty(id, false));
    }
    let result = coordinator.run(nodes);

    assert!(!result.consensus_reached);
    assert_eq!(result.yes_votes, 0);
    assert_eq!(result.total_votes, 4);
}

#[test]
fn test_consensus_all_faulty_accepting() {
    let coordinator = ConsensusCoordinator::new(1, 4, 42);
    let mut nodes = Vec::new();
    for id in 0..4 {
        nodes.push(Node::new_faulty(id, true));
    }
    let result = coordinator.run(nodes);

    assert!(result.consensus_reached);
    assert_eq!(result.yes_votes, 4);
    assert_eq!(result.total_votes, 4);
}

#[test]
fn test_consensus_faulty_accepting_with_invalid_value() {
    // Honest nodes reject value 0, faulty accepting nodes accept it
    // 2 faulty (accepting) + 3 honest = 2/5 yes = no consensus
    let coordinator = ConsensusCoordinator::new(1, 5, 0);
    let nodes = make_mixed_nodes(5, 2, true);
    let result = coordinator.run(nodes);

    assert!(!result.consensus_reached);
    assert_eq!(result.yes_votes, 2);
}

#[test]
fn test_consensus_faulty_accepting_majority_with_invalid_value() {
    // 3 faulty (accepting) + 2 honest = 3/5 yes = consensus (even though value is invalid!)
    // This demonstrates Byzantine behavior: faulty nodes can force consensus on bad values
    let coordinator = ConsensusCoordinator::new(1, 5, 0);
    let nodes = make_mixed_nodes(5, 3, true);
    let result = coordinator.run(nodes);

    assert!(result.consensus_reached);
    assert_eq!(result.yes_votes, 3);
}

// ============================================================================
// CONSENSUS RESULT COUNTING TESTS
// ============================================================================

#[test]
fn test_result_total_votes_equals_node_count() {
    let coordinator = ConsensusCoordinator::new(1, 7, 42);
    let nodes = make_honest_nodes(7);
    let result = coordinator.run(nodes);

    assert_eq!(result.total_votes, 7);
    assert_eq!(result.votes.len(), 7);
}

#[test]
fn test_result_yes_votes_plus_no_equals_total() {
    let coordinator = ConsensusCoordinator::new(1, 5, 42);
    let nodes = make_mixed_nodes(5, 2, false);
    let result = coordinator.run(nodes);

    let no_votes = result.total_votes - result.yes_votes;
    assert_eq!(result.yes_votes + no_votes, result.total_votes);
}

#[test]
fn test_result_contains_all_node_ids() {
    let coordinator = ConsensusCoordinator::new(1, 4, 42);
    let nodes = make_honest_nodes(4);
    let result = coordinator.run(nodes);

    let mut node_ids: Vec<usize> = result.votes.iter().map(|(id, _, _)| *id).collect();
    node_ids.sort();
    assert_eq!(node_ids, vec![0, 1, 2, 3]);
}

#[test]
fn test_result_proposed_value_preserved() {
    let coordinator = ConsensusCoordinator::new(3, 4, 777);
    let nodes = make_honest_nodes(4);
    let result = coordinator.run(nodes);

    assert_eq!(result.proposed_value, 777);
    assert_eq!(result.round, 3);
}

#[test]
fn test_result_vote_values_match_proposal() {
    let coordinator = ConsensusCoordinator::new(1, 3, 42);
    let nodes = make_honest_nodes(3);
    let result = coordinator.run(nodes);

    for (_, _, value) in &result.votes {
        assert_eq!(*value, 42);
    }
}

// ============================================================================
// MAJORITY THRESHOLD TESTS
// ============================================================================

#[test]
fn test_majority_threshold_exact_half_plus_one() {
    // 4 nodes: majority = 4/2 + 1 = 3
    // 2 faulty rejecting + 2 honest = 2/4 yes = no consensus (needs 3)
    let coordinator = ConsensusCoordinator::new(1, 4, 42);
    let nodes = make_mixed_nodes(4, 2, false);
    let result = coordinator.run(nodes);

    assert!(!result.consensus_reached);
    assert_eq!(result.yes_votes, 2);
}

#[test]
fn test_majority_threshold_odd_count() {
    // 3 nodes: majority = 3/2 + 1 = 2
    // 1 faulty rejecting + 2 honest = 2/3 yes = consensus (2 >= 2)
    let coordinator = ConsensusCoordinator::new(1, 3, 42);
    let nodes = make_mixed_nodes(3, 1, false);
    let result = coordinator.run(nodes);

    assert!(result.consensus_reached);
    assert_eq!(result.yes_votes, 2);
}

#[test]
fn test_single_node_consensus() {
    // 1 honest node: majority = 1/2 + 1 = 1
    let coordinator = ConsensusCoordinator::new(1, 1, 42);
    let nodes = make_honest_nodes(1);
    let result = coordinator.run(nodes);

    assert!(result.consensus_reached);
    assert_eq!(result.yes_votes, 1);
    assert_eq!(result.total_votes, 1);
}

#[test]
fn test_two_nodes_both_honest() {
    // 2 honest nodes: majority = 2/2 + 1 = 2
    let coordinator = ConsensusCoordinator::new(1, 2, 42);
    let nodes = make_honest_nodes(2);
    let result = coordinator.run(nodes);

    assert!(result.consensus_reached);
    assert_eq!(result.yes_votes, 2);
}

#[test]
fn test_two_nodes_one_faulty_rejecting() {
    // 2 nodes, 1 faulty rejecting: 1/2 yes, majority = 2, no consensus
    let coordinator = ConsensusCoordinator::new(1, 2, 42);
    let nodes = make_mixed_nodes(2, 1, false);
    let result = coordinator.run(nodes);

    assert!(!result.consensus_reached);
    assert_eq!(result.yes_votes, 1);
}

// ============================================================================
// BYZANTINE FAULT TOLERANCE CALCULATION TESTS
// ============================================================================

#[test]
fn test_byzantine_tolerance_zero_nodes() {
    assert_eq!(byzantine_tolerance(0), 0);
}

#[test]
fn test_byzantine_tolerance_one_node() {
    assert_eq!(byzantine_tolerance(1), 0);
}

#[test]
fn test_byzantine_tolerance_three_nodes() {
    // 3f+1 = 4 minimum for f=1, so 3 nodes can tolerate 0 faults
    assert_eq!(byzantine_tolerance(3), 0);
}

#[test]
fn test_byzantine_tolerance_four_nodes() {
    // 3f+1 = 4, so 4 nodes tolerate 1 fault
    assert_eq!(byzantine_tolerance(4), 1);
}

#[test]
fn test_byzantine_tolerance_seven_nodes() {
    // 3f+1 = 7 for f=2
    assert_eq!(byzantine_tolerance(7), 2);
}

#[test]
fn test_byzantine_tolerance_ten_nodes() {
    // (10-1)/3 = 3
    assert_eq!(byzantine_tolerance(10), 3);
}

#[test]
fn test_is_byzantine_safe_within_tolerance() {
    assert!(is_byzantine_safe(4, 0));
    assert!(is_byzantine_safe(4, 1));
    assert!(is_byzantine_safe(7, 2));
    assert!(is_byzantine_safe(10, 3));
}

#[test]
fn test_is_byzantine_safe_exceeds_tolerance() {
    assert!(!is_byzantine_safe(4, 2));
    assert!(!is_byzantine_safe(7, 3));
    assert!(!is_byzantine_safe(3, 1));
}

#[test]
fn test_is_byzantine_safe_zero_faulty_always_safe() {
    for n in 1..=10 {
        assert!(is_byzantine_safe(n, 0));
    }
}

// ============================================================================
// MESSAGE TYPE TESTS
// ============================================================================

#[test]
fn test_message_proposal() {
    let msg = Message::Proposal {
        round: 1,
        value: 42,
    };
    match msg {
        Message::Proposal { round, value } => {
            assert_eq!(round, 1);
            assert_eq!(value, 42);
        }
        _ => panic!("Expected Proposal variant"),
    }
}

#[test]
fn test_message_vote() {
    let msg = Message::Vote {
        node_id: 3,
        round: 2,
        value: 100,
        accept: true,
    };
    match msg {
        Message::Vote {
            node_id,
            round,
            value,
            accept,
        } => {
            assert_eq!(node_id, 3);
            assert_eq!(round, 2);
            assert_eq!(value, 100);
            assert!(accept);
        }
        _ => panic!("Expected Vote variant"),
    }
}

#[test]
fn test_message_decision() {
    let msg = Message::Decision {
        round: 5,
        value: 999,
    };
    match msg {
        Message::Decision { round, value } => {
            assert_eq!(round, 5);
            assert_eq!(value, 999);
        }
        _ => panic!("Expected Decision variant"),
    }
}

#[test]
fn test_message_clone() {
    let msg = Message::Vote {
        node_id: 0,
        round: 1,
        value: 42,
        accept: true,
    };
    let cloned = msg.clone();
    assert_eq!(msg, cloned);
}

#[test]
fn test_message_equality() {
    let msg1 = Message::Proposal {
        round: 1,
        value: 42,
    };
    let msg2 = Message::Proposal {
        round: 1,
        value: 42,
    };
    let msg3 = Message::Proposal {
        round: 2,
        value: 42,
    };
    assert_eq!(msg1, msg2);
    assert_ne!(msg1, msg3);
}

// ============================================================================
// NODE TYPE TESTS
// ============================================================================

#[test]
fn test_node_type_copy() {
    let nt = NodeType::Honest;
    let nt2 = nt; // Copy
    assert_eq!(nt, nt2);
}

#[test]
fn test_node_type_equality() {
    assert_eq!(NodeType::Honest, NodeType::Honest);
    assert_eq!(NodeType::Faulty, NodeType::Faulty);
    assert_ne!(NodeType::Honest, NodeType::Faulty);
}

// ============================================================================
// COORDINATOR CREATION TESTS
// ============================================================================

#[test]
fn test_coordinator_stores_fields() {
    let coordinator = ConsensusCoordinator::new(3, 7, 100);
    assert_eq!(coordinator.round, 3);
    assert_eq!(coordinator.num_nodes, 7);
    assert_eq!(coordinator.proposal_value, 100);
}

// ============================================================================
// LARGER SCALE TESTS
// ============================================================================

#[test]
fn test_consensus_with_many_honest_nodes() {
    let coordinator = ConsensusCoordinator::new(1, 20, 42);
    let nodes = make_honest_nodes(20);
    let result = coordinator.run(nodes);

    assert!(result.consensus_reached);
    assert_eq!(result.yes_votes, 20);
    assert_eq!(result.total_votes, 20);
}

#[test]
fn test_consensus_with_byzantine_tolerance_limit() {
    // 7 nodes can tolerate 2 faults
    // 2 faulty (rejecting) + 5 honest = 5/7 yes = consensus
    let coordinator = ConsensusCoordinator::new(1, 7, 42);
    let nodes = make_mixed_nodes(7, 2, false);
    let result = coordinator.run(nodes);

    assert!(result.consensus_reached);
    assert_eq!(result.yes_votes, 5);
}

#[test]
fn test_consensus_exceeding_byzantine_tolerance() {
    // 4 nodes can tolerate 1 fault
    // 3 faulty (rejecting) + 1 honest = 1/4 yes = no consensus
    let coordinator = ConsensusCoordinator::new(1, 4, 42);
    let nodes = make_mixed_nodes(4, 3, false);
    let result = coordinator.run(nodes);

    assert!(!result.consensus_reached);
    assert_eq!(result.yes_votes, 1);
}

#[test]
fn test_multiple_rounds_independent() {
    // Each round should be independent
    let result1 = {
        let coordinator = ConsensusCoordinator::new(1, 3, 42);
        coordinator.run(make_honest_nodes(3))
    };

    let result2 = {
        let coordinator = ConsensusCoordinator::new(2, 3, 0);
        coordinator.run(make_honest_nodes(3))
    };

    assert!(result1.consensus_reached);
    assert!(!result2.consensus_reached);
    assert_eq!(result1.round, 1);
    assert_eq!(result2.round, 2);
}

// ============================================================================
// EDGE CASE: MIXED FAULTY BEHAVIOR TESTS
// ============================================================================

#[test]
fn test_mixed_faulty_some_accept_some_reject() {
    // 5 nodes: 1 faulty-accepting + 1 faulty-rejecting + 3 honest
    // With value 42 (valid): honest=yes(3), faulty-accept=yes(1), faulty-reject=no(1)
    // = 4/5 yes = consensus
    let coordinator = ConsensusCoordinator::new(1, 5, 42);
    let nodes = vec![
        Node::new_faulty(0, true),  // accepts
        Node::new_faulty(1, false), // rejects
        Node::new(2, NodeType::Honest),
        Node::new(3, NodeType::Honest),
        Node::new(4, NodeType::Honest),
    ];
    let result = coordinator.run(nodes);

    assert!(result.consensus_reached);
    assert_eq!(result.yes_votes, 4);
    assert_eq!(result.total_votes, 5);
}

#[test]
fn test_faulty_nodes_on_boundary_value() {
    // value = 999 (just barely valid for honest nodes)
    let coordinator = ConsensusCoordinator::new(1, 3, 999);
    let nodes = make_honest_nodes(3);
    let result = coordinator.run(nodes);

    assert!(result.consensus_reached);
    assert_eq!(result.yes_votes, 3);
}

#[test]
fn test_faulty_nodes_on_invalid_boundary_value() {
    // value = 1000 (just barely invalid for honest nodes)
    // 1 faulty accepting + 2 honest rejecting = 1/3 no consensus
    let coordinator = ConsensusCoordinator::new(1, 3, 1000);
    let nodes = make_mixed_nodes(3, 1, true);
    let result = coordinator.run(nodes);

    assert!(!result.consensus_reached);
    assert_eq!(result.yes_votes, 1);
}
