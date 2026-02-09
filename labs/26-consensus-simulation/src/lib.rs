//! # Lab 26: Consensus Simulation
//!
//! Student-facing API for distributed voting and Byzantine tolerance helpers.

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Proposal { round: u32, value: i32 },
    Vote { node_id: usize, round: u32, value: i32, accept: bool },
    Decision { round: u32, value: i32 },
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum NodeType {
    Honest,
    Faulty,
}

pub struct Node {
    pub id: usize,
    pub node_type: NodeType,
    pub faulty_accepts: bool,
}

impl Node {
    pub fn new(id: usize, node_type: NodeType) -> Self {
        let _ = (id, node_type);
        todo!("Create node")
    }

    pub fn new_faulty(id: usize, faulty_accepts: bool) -> Self {
        let _ = (id, faulty_accepts);
        todo!("Create faulty node")
    }

    pub fn process_proposal(&self, round: u32, value: i32) -> bool {
        let _ = (round, value);
        todo!("Process proposal according to node type")
    }
}

#[derive(Debug, Clone)]
pub struct ConsensusResult {
    pub round: u32,
    pub proposed_value: i32,
    pub votes: Vec<(usize, bool, i32)>,
    pub consensus_reached: bool,
    pub yes_votes: usize,
    pub total_votes: usize,
}

pub struct ConsensusCoordinator {
    pub round: u32,
    pub num_nodes: usize,
    pub proposal_value: i32,
}

impl ConsensusCoordinator {
    pub fn new(round: u32, num_nodes: usize, proposal_value: i32) -> Self {
        let _ = (round, num_nodes, proposal_value);
        todo!("Create consensus coordinator")
    }

    pub fn run(&self, nodes: Vec<Node>) -> ConsensusResult {
        let _ = nodes;
        todo!("Run consensus round")
    }
}

pub fn byzantine_tolerance(num_nodes: usize) -> usize {
    let _ = num_nodes;
    todo!("Compute Byzantine tolerance")
}

pub fn is_byzantine_safe(num_nodes: usize, num_faulty: usize) -> bool {
    let _ = (num_nodes, num_faulty);
    todo!("Check Byzantine safety")
}

#[doc(hidden)]
pub mod solution;
