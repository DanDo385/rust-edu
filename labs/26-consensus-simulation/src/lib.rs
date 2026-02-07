// Lab 26: Consensus Simulation
//
// Simulates distributed consensus using a simplified voting algorithm.
// Demonstrates message passing between threads, coordination, and
// Byzantine fault tolerance concepts in a distributed system.
//
// ============================================================================
// OWNERSHIP & BORROWING COMMENTARY
// ============================================================================
// - Message is Clone + Debug so it can be sent across thread boundaries
// - NodeType is Copy (cheap enum) - no ownership issues when passing to threads
// - Node owns its id and node_type; process_proposal borrows &self immutably
// - ConsensusCoordinator owns round/num_nodes/proposal_value (all Copy types)
// - ConsensusResult owns the votes Vec - caller gets full ownership of results
// - Channels transfer ownership of Messages from node threads to coordinator
// - Each node thread takes ownership of its Sender<Message> clone
//
// ============================================================================
// DETERMINISTIC DESIGN FOR TESTABILITY
// ============================================================================
// The original main.rs used rand::thread_rng() for faulty node behavior.
// In this library version, faulty nodes use a configurable "faulty_accepts"
// flag so tests can be deterministic and reproducible.

use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;

// ============================================================================
// MESSAGE TYPES
// ============================================================================

/// Messages exchanged in the consensus protocol.
///
/// The protocol follows a simple leader-based voting scheme:
/// 1. Leader sends a `Proposal` to all nodes
/// 2. Each node responds with a `Vote` (accept or reject)
/// 3. Leader tallies votes and announces a `Decision`
#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    /// Leader proposes a value for consensus
    Proposal { round: u32, value: i32 },
    /// A node votes on the proposal
    Vote {
        node_id: usize,
        round: u32,
        value: i32,
        accept: bool,
    },
    /// Leader announces the consensus decision
    Decision { round: u32, value: i32 },
}

// ============================================================================
// NODE TYPES
// ============================================================================

/// Represents the behavior type of a consensus node.
///
/// In Byzantine fault tolerance, nodes are either honest (follow the protocol)
/// or faulty/Byzantine (may behave arbitrarily).
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    /// Honest node: follows the protocol faithfully
    Honest,
    /// Faulty (Byzantine) node: may vote incorrectly
    Faulty,
}

// ============================================================================
// NODE
// ============================================================================

/// A participant in the consensus protocol.
///
/// Each node has an ID and a behavior type. Honest nodes evaluate proposals
/// based on validity criteria. Faulty nodes use a configurable acceptance
/// flag for deterministic testing (in production, this would be random).
pub struct Node {
    pub id: usize,
    pub node_type: NodeType,
    /// For faulty nodes: determines whether they accept or reject proposals.
    /// Ignored for honest nodes. Defaults to false.
    pub faulty_accepts: bool,
}

impl Node {
    /// Creates a new honest node with the given ID.
    pub fn new(id: usize, node_type: NodeType) -> Self {
        Node {
            id,
            node_type,
            faulty_accepts: false,
        }
    }

    /// Creates a new faulty node with configurable acceptance behavior.
    ///
    /// When `faulty_accepts` is true, the faulty node votes yes on proposals.
    /// When false, it votes no. This replaces random behavior for testability.
    pub fn new_faulty(id: usize, faulty_accepts: bool) -> Self {
        Node {
            id,
            node_type: NodeType::Faulty,
            faulty_accepts,
        }
    }

    /// Processes a proposal and returns whether this node accepts it.
    ///
    /// - **Honest nodes**: Accept proposals where `value > 0 && value < 1000`
    /// - **Faulty nodes**: Return their configured `faulty_accepts` value
    pub fn process_proposal(&self, _round: u32, value: i32) -> bool {
        match self.node_type {
            NodeType::Honest => {
                // Honest node accepts reasonable values
                value > 0 && value < 1000
            }
            NodeType::Faulty => {
                // Faulty node uses configured behavior (deterministic for testing)
                self.faulty_accepts
            }
        }
    }
}

// ============================================================================
// CONSENSUS RESULT
// ============================================================================

/// The outcome of a consensus round, including all votes and the decision.
#[derive(Debug, Clone)]
pub struct ConsensusResult {
    /// The round number
    pub round: u32,
    /// The value that was proposed
    pub proposed_value: i32,
    /// All votes: (node_id, accepted, value)
    pub votes: Vec<(usize, bool, i32)>,
    /// Whether consensus was reached (majority voted yes)
    pub consensus_reached: bool,
    /// Number of yes votes
    pub yes_votes: usize,
    /// Total number of votes received
    pub total_votes: usize,
}

// ============================================================================
// CONSENSUS COORDINATOR (LEADER)
// ============================================================================

/// The leader that coordinates a consensus round.
///
/// The coordinator:
/// 1. Spawns a thread for each node
/// 2. Each node processes the proposal and sends a vote via channel
/// 3. Coordinator collects all votes
/// 4. Determines if majority (> 50%) voted yes
pub struct ConsensusCoordinator {
    pub round: u32,
    pub num_nodes: usize,
    pub proposal_value: i32,
}

impl ConsensusCoordinator {
    /// Creates a new consensus coordinator for a specific round.
    pub fn new(round: u32, num_nodes: usize, proposal_value: i32) -> Self {
        ConsensusCoordinator {
            round,
            num_nodes,
            proposal_value,
        }
    }

    /// Runs a consensus round with the given node configurations.
    ///
    /// Each entry in `nodes` defines a node's behavior. The coordinator
    /// spawns a thread per node, collects votes via channels, and determines
    /// whether consensus was reached by majority vote.
    ///
    /// # Ownership Notes
    /// - Each `Node` is moved into its thread (ownership transfer)
    /// - `Sender<Message>` is cloned for each thread (Arc internally)
    /// - The original sender is dropped so the channel closes properly
    /// - `ConsensusResult` is returned as an owned value
    pub fn run(&self, nodes: Vec<Node>) -> ConsensusResult {
        // Create channel for node-to-coordinator communication
        let (coordinator_tx, coordinator_rx): (Sender<Message>, Receiver<Message>) =
            mpsc::channel();

        // Spawn a thread for each node
        let mut node_handles = vec![];

        for node in nodes {
            let tx = coordinator_tx.clone();
            let round = self.round;
            let value = self.proposal_value;

            let handle = thread::spawn(move || {
                // Node processes the proposal
                let accept = node.process_proposal(round, value);

                // Send vote back to coordinator via channel
                let vote = Message::Vote {
                    node_id: node.id,
                    round,
                    value,
                    accept,
                };

                let _ = tx.send(vote);
            });

            node_handles.push(handle);
        }

        // Drop the original sender so the channel closes when all node threads finish
        // This is critical: without this, coordinator_rx.iter() would block forever
        drop(coordinator_tx);

        // Collect votes from all nodes
        let mut votes = vec![];
        for msg in coordinator_rx {
            if let Message::Vote {
                node_id,
                round: msg_round,
                value,
                accept,
            } = msg
            {
                if msg_round == self.round {
                    votes.push((node_id, accept, value));
                }
            }
        }

        // Wait for all node threads to complete
        for handle in node_handles {
            handle.join().unwrap();
        }

        // Count votes and determine consensus
        let yes_votes = votes.iter().filter(|(_, accept, _)| *accept).count();
        let total_votes = votes.len();
        let majority = total_votes / 2 + 1;
        let consensus_reached = yes_votes >= majority;

        ConsensusResult {
            round: self.round,
            proposed_value: self.proposal_value,
            votes,
            consensus_reached,
            yes_votes,
            total_votes,
        }
    }
}

// ============================================================================
// BYZANTINE FAULT TOLERANCE HELPERS
// ============================================================================

/// Calculates the maximum number of Byzantine faults that can be tolerated
/// given the total number of nodes.
///
/// The BFT formula requires 3f + 1 nodes to tolerate f faults.
/// Therefore: f = (n - 1) / 3
pub fn byzantine_tolerance(num_nodes: usize) -> usize {
    if num_nodes == 0 {
        return 0;
    }
    (num_nodes - 1) / 3
}

/// Returns whether a system with `num_nodes` total nodes can safely
/// tolerate `num_faulty` Byzantine nodes.
pub fn is_byzantine_safe(num_nodes: usize, num_faulty: usize) -> bool {
    num_faulty <= byzantine_tolerance(num_nodes)
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. CHANNELS (mpsc::channel)
//    Multi-producer, single-consumer channel.
//    Sender can be cloned (multi-producer), Receiver cannot.
//    When all Senders are dropped, the channel closes.
//    recv() blocks until a message is available or channel closes.
//
// 2. THREAD SPAWNING
//    std::thread::spawn creates an OS thread (~2MB stack).
//    Closure must be 'static + Send (values moved into thread).
//    Thread::join() waits for completion and returns Result.
//
// 3. OWNERSHIP TRANSFER TO THREADS
//    The `move` keyword in the closure transfers ownership.
//    Node, Sender, round, and value are all moved into the thread.
//    The main thread can no longer access them after the spawn.
//
// 4. CHANNEL CLOSING PATTERN
//    Dropping coordinator_tx is essential:
//    - Each node thread has its own clone of the sender
//    - When all clones are dropped (threads finish), channel closes
//    - coordinator_rx.iter() then terminates
//    - Without this, the coordinator would block forever

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_honest_node_accepts_valid_value() {
        let node = Node::new(0, NodeType::Honest);
        assert!(node.process_proposal(1, 42));
    }

    #[test]
    fn test_honest_node_rejects_zero() {
        let node = Node::new(0, NodeType::Honest);
        assert!(!node.process_proposal(1, 0));
    }

    #[test]
    fn test_honest_node_rejects_negative() {
        let node = Node::new(0, NodeType::Honest);
        assert!(!node.process_proposal(1, -1));
    }

    #[test]
    fn test_honest_node_rejects_too_large() {
        let node = Node::new(0, NodeType::Honest);
        assert!(!node.process_proposal(1, 1000));
        assert!(!node.process_proposal(1, 5000));
    }

    #[test]
    fn test_faulty_node_configurable() {
        let accepting = Node::new_faulty(0, true);
        assert!(accepting.process_proposal(1, 42));

        let rejecting = Node::new_faulty(1, false);
        assert!(!rejecting.process_proposal(1, 42));
    }

    #[test]
    fn test_byzantine_tolerance_values() {
        assert_eq!(byzantine_tolerance(0), 0);
        assert_eq!(byzantine_tolerance(1), 0);
        assert_eq!(byzantine_tolerance(4), 1);
        assert_eq!(byzantine_tolerance(7), 2);
        assert_eq!(byzantine_tolerance(10), 3);
    }
}
