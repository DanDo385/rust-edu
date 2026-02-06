// Project 23: Consensus Simulation
//
// Simulates distributed consensus using a simplified voting algorithm.
// Demonstrates message passing between threads, coordination, and
// Byzantine fault tolerance in a distributed system.

use colored::Colorize;
use rand::Rng;
use std::sync::mpsc::{self, Receiver, Sender};
use std::thread;
use std::time::Duration;

fn main() {
    println!("{}", "=== Distributed Consensus Simulation ===".bright_blue().bold());
    println!();

    // Configuration
    let num_nodes = 5;
    let num_faulty = 1;
    let proposed_value = 42;

    print_configuration(num_nodes, num_faulty);
    println!();

    // Run consensus rounds
    println!("{}", "Starting Consensus Rounds:".bright_yellow());
    println!();

    for round in 1..=3 {
        println!("{}", format!("Round {}:", round).bright_cyan());
        run_consensus_round(round, num_nodes, num_faulty, proposed_value + round - 1);
        println!();
        thread::sleep(Duration::from_millis(500));
    }

    // Demonstrate Byzantine fault tolerance
    println!("{}", "Byzantine Fault Tolerance Test:".bright_yellow());
    demonstrate_byzantine_tolerance();
}

// ============================================================================
// MESSAGE TYPES
// ============================================================================

#[derive(Debug, Clone)]
enum Message {
    // Leader proposes a value
    Proposal { round: u32, value: i32 },
    // Node votes on proposal
    Vote { node_id: usize, round: u32, value: i32, accept: bool },
    // Leader announces consensus result
    Decision { round: u32, value: i32 },
}

// ============================================================================
// NODE TYPES
// ============================================================================

#[derive(Debug, Clone, Copy)]
enum NodeType {
    Honest,
    Faulty,  // Byzantine node (votes randomly or maliciously)
}

struct Node {
    id: usize,
    node_type: NodeType,
}

impl Node {
    fn new(id: usize, node_type: NodeType) -> Self {
        Node { id, node_type }
    }

    /// Node processes a proposal and decides whether to vote yes
    fn process_proposal(&self, round: u32, value: i32) -> bool {
        match self.node_type {
            NodeType::Honest => {
                // Honest node accepts reasonable values
                value > 0 && value < 1000
            }
            NodeType::Faulty => {
                // Faulty node votes randomly
                let mut rng = rand::thread_rng();
                rng.gen_bool(0.3)  // 30% chance of voting yes
            }
        }
    }
}

// ============================================================================
// CONSENSUS COORDINATOR (LEADER)
// ============================================================================

struct ConsensusCoordinator {
    round: u32,
    num_nodes: usize,
    proposal_value: i32,
}

impl ConsensusCoordinator {
    fn new(round: u32, num_nodes: usize, proposal_value: i32) -> Self {
        ConsensusCoordinator {
            round,
            num_nodes,
            proposal_value,
        }
    }

    /// Runs a consensus round
    fn run(&self, node_types: Vec<NodeType>) -> ConsensusResult {
        // Create channels for communication
        let (coordinator_tx, coordinator_rx): (Sender<Message>, Receiver<Message>) = mpsc::channel();

        // Spawn node threads
        let mut node_handles = vec![];

        for (id, &node_type) in node_types.iter().enumerate() {
            let tx = coordinator_tx.clone();
            let round = self.round;
            let value = self.proposal_value;

            let handle = thread::spawn(move || {
                let node = Node::new(id, node_type);
                simulate_node(node, round, value, tx);
            });

            node_handles.push(handle);
        }

        // Drop the original sender so the channel closes when all nodes finish
        drop(coordinator_tx);

        // Collect votes
        let mut votes = vec![];
        for msg in coordinator_rx {
            if let Message::Vote { node_id, round: msg_round, value, accept } = msg {
                if msg_round == self.round {
                    votes.push((node_id, accept, value));
                }
            }
        }

        // Wait for all nodes to complete
        for handle in node_handles {
            handle.join().unwrap();
        }

        // Count votes
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
// NODE SIMULATION
// ============================================================================

fn simulate_node(node: Node, round: u32, proposed_value: i32, tx: Sender<Message>) {
    // Simulate network delay
    let mut rng = rand::thread_rng();
    let delay_ms = rng.gen_range(10..100);
    thread::sleep(Duration::from_millis(delay_ms));

    // Process proposal
    let accept = node.process_proposal(round, proposed_value);

    // Send vote
    let vote = Message::Vote {
        node_id: node.id,
        round,
        value: proposed_value,
        accept,
    };

    let _ = tx.send(vote);
}

// ============================================================================
// CONSENSUS RESULT
// ============================================================================

struct ConsensusResult {
    round: u32,
    proposed_value: i32,
    votes: Vec<(usize, bool, i32)>,  // (node_id, accept, value)
    consensus_reached: bool,
    yes_votes: usize,
    total_votes: usize,
}

impl ConsensusResult {
    fn print(&self) {
        println!("  Proposal: value = {}", self.proposed_value.to_string().bright_white());
        println!();

        // Print votes
        println!("  Votes:");
        for (node_id, accept, _) in &self.votes {
            let vote_str = if *accept { "YES".green() } else { "NO".red() };
            let node_str = format!("Node {}", node_id);
            println!("    {} voted {}", node_str.bright_cyan(), vote_str);
        }

        println!();

        // Print result
        let percentage = (self.yes_votes as f64 / self.total_votes as f64) * 100.0;

        if self.consensus_reached {
            println!(
                "  {} {}",
                "CONSENSUS REACHED".bright_green().bold(),
                format!("({}/{} votes, {:.1}%)", self.yes_votes, self.total_votes, percentage).bright_white()
            );
            println!("  Agreed value: {}", self.proposed_value.to_string().bright_green().bold());
        } else {
            println!(
                "  {} {}",
                "NO CONSENSUS".bright_red().bold(),
                format!("({}/{} votes, {:.1}%)", self.yes_votes, self.total_votes, percentage).bright_white()
            );
        }
    }
}

// ============================================================================
// DEMONSTRATIONS
// ============================================================================

fn print_configuration(num_nodes: usize, num_faulty: usize) {
    println!("{}", "Configuration:".bright_yellow());
    println!("  Total nodes: {}", num_nodes);
    println!("  Faulty nodes: {}", num_faulty);

    let byzantine_tolerance = (num_nodes - 1) / 3;
    let min_nodes = 3 * byzantine_tolerance + 1;

    println!("  Byzantine tolerance: {} fault(s) (3f+1 = {} minimum)", byzantine_tolerance, min_nodes);

    if num_faulty <= byzantine_tolerance {
        println!("  Status: {} (can tolerate {} faulty nodes)", "SAFE".bright_green(), byzantine_tolerance);
    } else {
        println!(
            "  Status: {} (too many faulty nodes: {} > {})",
            "UNSAFE".bright_red(),
            num_faulty,
            byzantine_tolerance
        );
    }
}

fn run_consensus_round(round: u32, num_nodes: usize, num_faulty: usize, value: i32) {
    // Create node types (some honest, some faulty)
    let mut node_types = vec![NodeType::Honest; num_nodes];

    // Randomly select faulty nodes
    let mut rng = rand::thread_rng();
    let mut faulty_indices: Vec<usize> = (0..num_nodes).collect();

    // Shuffle and take first num_faulty as faulty nodes
    use rand::seq::SliceRandom;
    faulty_indices.shuffle(&mut rng);

    for &idx in faulty_indices.iter().take(num_faulty) {
        node_types[idx] = NodeType::Faulty;
    }

    // Run consensus
    let coordinator = ConsensusCoordinator::new(round, num_nodes, value);
    let result = coordinator.run(node_types);

    // Print result
    result.print();
}

fn demonstrate_byzantine_tolerance() {
    println!();

    let test_cases = vec![
        (4, 0, "No faulty nodes"),
        (4, 1, "1 faulty node (Byzantine tolerance limit)"),
        (4, 2, "2 faulty nodes (exceeds tolerance)"),
        (7, 2, "2 faulty nodes with 7 total (within tolerance)"),
    ];

    for (num_nodes, num_faulty, description) in test_cases {
        println!("{}", format!("Test: {}", description).bright_cyan());

        let byzantine_tolerance = (num_nodes - 1) / 3;
        let safe = num_faulty <= byzantine_tolerance;

        if safe {
            println!("  {} System can tolerate this configuration", "✓".green());
        } else {
            println!("  {} System CANNOT tolerate this configuration", "✗".red());
        }

        println!("  Running consensus...");

        // Simplified consensus check
        let mut node_types = vec![NodeType::Honest; num_nodes];
        for i in 0..num_faulty {
            if i < num_nodes {
                node_types[i] = NodeType::Faulty;
            }
        }

        let coordinator = ConsensusCoordinator::new(1, num_nodes, 100);
        let result = coordinator.run(node_types);

        if result.consensus_reached {
            println!("  {} Consensus achieved: {}/{} votes", "✓".green(), result.yes_votes, result.total_votes);
        } else {
            println!("  {} Consensus failed: {}/{} votes", "✗".red(), result.yes_votes, result.total_votes);
        }

        println!();
    }
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. CHANNELS (mpsc::channel)
//    Multi-producer, single-consumer channel.
//    Implemented using a lock-free queue for performance.
//    Sender clones share the same channel (Arc internally).
//    When all Senders are dropped, the channel closes.
//
// 2. THREAD SPAWNING
//    std::thread::spawn creates an OS thread (~2MB stack).
//    Closure must be Send (can be transferred to another thread).
//    Thread::join() waits for thread completion and gets result.
//
// 3. MESSAGE PASSING
//    send() copies or moves the message to the channel.
//    recv() blocks until a message is available.
//    Channels have infinite capacity by default (grows as needed).
//
// 4. OWNERSHIP IN THREADS
//    Values moved into thread closure are owned by that thread.
//    Sender can be cloned, Receiver cannot.
//    Thread closure must be 'static or borrow with lifetimes.
//
// 5. SYNCHRONIZATION
//    No explicit locks needed with message passing!
//    Channels handle synchronization internally.
//    This is "share memory by communicating" not "communicate by sharing memory".

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Consensus allows distributed nodes to agree on a value
// 2. Message passing enables thread coordination
// 3. Byzantine fault tolerance requires 3f+1 nodes for f faults
// 4. Channels provide safe communication between threads
// 5. Majority voting is a simple consensus mechanism
// 6. Leader proposes, nodes vote, majority decides
// 7. Faulty nodes can disrupt consensus
// 8. Real consensus algorithms are more complex (Paxos, Raft)
// 9. Network delays affect consensus performance
// 10. Rust's channels prevent data races at compile time

// ============================================================================
// CONSENSUS ALGORITHMS IN PRACTICE
// ============================================================================
// RAFT (etcd, Consul):
//   - Leader election with randomized timeouts
//   - Log replication with majority consensus
//   - Safety: at most one leader per term
//   - Liveness: eventually elects a leader
//
// PBFT (Practical Byzantine Fault Tolerance):
//   - 3-phase protocol: pre-prepare, prepare, commit
//   - Tolerates f Byzantine faults with 3f+1 nodes
//   - Used in Hyperledger Fabric
//
// PROOF OF WORK (Bitcoin):
//   - Longest chain rule
//   - Probabilistic finality (6 confirmations)
//   - Energy intensive but censorship resistant
//
// PROOF OF STAKE (Ethereum 2.0):
//   - Validator selection based on stake
//   - Economic incentives for honest behavior
//   - More energy efficient than PoW

// ============================================================================
// IMPROVING THIS SIMULATION
// ============================================================================
// 1. Add leader election (select random leader)
// 2. Implement view changes (new leader if current fails)
// 3. Add timeout handling (detect crashed nodes)
// 4. Persistent log (save decisions to disk)
// 5. Network simulation (add delays, packet loss)
// 6. Byzantine node strategies (double voting, conflicting messages)
// 7. Signature verification (cryptographic authenticity)
// 8. Multi-round consensus (sequence of values)
// 9. Reconfiguration (add/remove nodes)
// 10. Performance metrics (latency, throughput)

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Not dropping coordinator sender (channel never closes)
// ❌ Using unwrap() on recv() (panics on channel close)
// ❌ Forgetting to join() threads (orphaned threads)
// ❌ Shared mutable state instead of message passing
// ❌ Not handling Byzantine behavior
// ❌ Ignoring network delays and failures
// ❌ Assuming synchronous communication (async is reality)
// ❌ Not implementing timeout for consensus
// ❌ Single point of failure (centralized leader)
// ❌ Not testing with faulty nodes
