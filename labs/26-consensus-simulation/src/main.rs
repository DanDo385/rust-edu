//! # Consensus Simulation Demo

use consensus_simulation::solution::{ConsensusCoordinator, Node, NodeType};

fn main() {
    println!("=== Consensus Simulation Demo ===\n");

    let nodes = vec![
        Node::new(0, NodeType::Honest),
        Node::new(1, NodeType::Honest),
        Node::new(2, NodeType::Honest),
        Node::new_faulty(3, false),
    ];

    let coordinator = ConsensusCoordinator::new(1, nodes.len(), 42);
    let result = coordinator.run(nodes);

    println!("round: {}", result.round);
    println!("yes votes: {}/{}", result.yes_votes, result.total_votes);
    println!("consensus reached: {}", result.consensus_reached);
}
