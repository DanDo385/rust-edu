//! # Lab 19: Smart Pointers
//!
//! Student-facing API for `Box`, `Rc`, and `RefCell` patterns.

use std::cell::RefCell;
use std::rc::Rc;

#[derive(Debug, Clone)]
pub enum List<T> {
    Cons(T, Box<List<T>>),
    Nil,
}

impl<T> List<T> {
    pub fn nil() -> Self {
        // TODO: Return empty list variant.
        todo!("Create empty list")
    }

    pub fn cons(head: T, tail: List<T>) -> Self {
        // TODO: Prepend element by boxing the tail.
        let _ = (head, tail);
        todo!("Construct list node")
    }

    pub fn len(&self) -> usize {
        // TODO: Recursively compute list length.
        let _ = self;
        todo!("Compute list length")
    }

    pub fn head(&self) -> Option<&T> {
        // TODO: Return first element reference if present.
        let _ = self;
        todo!("Access list head")
    }
}

#[derive(Debug)]
pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    pub fn new(value: T) -> Self {
        // TODO: Construct leaf node with no children.
        let _ = value;
        todo!("Create tree node")
    }

    pub fn with_left(mut self, left: TreeNode<T>) -> Self {
        // TODO: Attach left child.
        let _ = (&mut self, left);
        todo!("Attach left child")
    }

    pub fn with_right(mut self, right: TreeNode<T>) -> Self {
        // TODO: Attach right child.
        let _ = (&mut self, right);
        todo!("Attach right child")
    }

    pub fn value(&self) -> &T {
        // TODO: Borrow node value.
        todo!("Borrow tree node value")
    }

    pub fn sum(&self) -> i32
    where
        T: Into<i32> + Copy,
    {
        // TODO: Sum value + recursive child sums.
        todo!("Sum tree values")
    }
}

pub fn shared_value_example() -> Rc<Vec<i32>> {
    // TODO: Return Rc-wrapped vector.
    todo!("Create shared Rc value")
}

pub fn count_references<T>(rc: &Rc<T>) -> usize {
    // TODO: Return strong reference count.
    let _ = rc;
    todo!("Count Rc strong references")
}

#[derive(Clone)]
pub struct SharedCounter {
    value: Rc<RefCell<i32>>,
}

impl SharedCounter {
    pub fn new(initial: i32) -> Self {
        // TODO: Construct Rc<RefCell<i32>> with initial value.
        let _ = initial;
        todo!("Create shared counter")
    }

    pub fn get(&self) -> i32 {
        // TODO: Borrow immutably and read value.
        todo!("Get counter value")
    }

    pub fn increment(&self) {
        // TODO: Borrow mutably and increment by 1.
        todo!("Increment counter")
    }

    pub fn reset(&self) {
        // TODO: Set value to 0.
        todo!("Reset counter")
    }

    pub fn set(&self, new_value: i32) {
        // TODO: Set value explicitly.
        let _ = new_value;
        todo!("Set counter value")
    }
}

#[derive(Clone)]
pub struct GraphNode {
    id: u32,
    neighbors: Rc<RefCell<Vec<Rc<GraphNode>>>>,
}

impl GraphNode {
    pub fn new(id: u32) -> Self {
        // TODO: Construct graph node with empty neighbors.
        let _ = id;
        todo!("Create graph node")
    }

    pub fn id(&self) -> u32 {
        // TODO: Return node identifier.
        todo!("Get graph node id")
    }

    pub fn connect_to(&self, other: Rc<GraphNode>) {
        // TODO: Push neighbor into shared neighbor list.
        let _ = other;
        todo!("Connect graph nodes")
    }

    pub fn neighbor_count(&self) -> usize {
        // TODO: Return current neighbor count.
        todo!("Count neighbors")
    }

    pub fn neighbor_ids(&self) -> Vec<u32> {
        // TODO: Collect neighbor IDs.
        todo!("List neighbor IDs")
    }
}

#[derive(Clone)]
pub struct Resource {
    name: String,
}

impl Resource {
    pub fn new(name: String) -> Self {
        // TODO: Construct resource and print acquisition message.
        let _ = name;
        todo!("Create resource")
    }

    pub fn name(&self) -> &str {
        // TODO: Borrow resource name.
        todo!("Get resource name")
    }
}

#[doc(hidden)]
pub mod solution;
