//! # Merkle Tree Demo

use merkle_tree_alternative::solution::MerkleTree;

fn main() {
    println!("=== Merkle Tree Demo ===");

    let data = vec!["tx1", "tx2", "tx3", "tx4"];
    let tree = MerkleTree::new(&data);
    println!("leaf_count={} node_count={}", tree.leaf_count(), tree.node_count());
    println!("root={}", tree.root());

    if let Some(proof) = tree.generate_proof(0) {
        let ok = MerkleTree::verify_proof(tree.root(), data[0], &proof);
        println!("proof for tx1 valid={}", ok);
    }
}
