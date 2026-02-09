//! # Smart Pointers Demo

use std::rc::Rc;
use smart_pointers::solution::{self, GraphNode, List, SharedCounter, TreeNode};

fn main() {
    println!("=== Smart Pointers Demo ===\n");

    let list = List::cons(1, List::cons(2, List::cons(3, List::nil())));
    println!("list len: {}", list.len());

    let tree = TreeNode::new(10).with_left(TreeNode::new(20)).with_right(TreeNode::new(30));
    println!("tree sum: {}", tree.sum());

    let shared = solution::shared_value_example();
    let shared2 = Rc::clone(&shared);
    println!("Rc strong_count: {}", solution::count_references(&shared2));

    let counter = SharedCounter::new(0);
    counter.increment();
    counter.increment();
    println!("counter: {}", counter.get());

    let a = Rc::new(GraphNode::new(1));
    let b = Rc::new(GraphNode::new(2));
    a.connect_to(Rc::clone(&b));
    println!("node {} neighbors: {:?}", a.id(), a.neighbor_ids());
}
