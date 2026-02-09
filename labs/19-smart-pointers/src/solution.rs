//! # Lab 19: Smart Pointers
//!
//! Smart pointers own the data they point to and implement Deref and Drop traits.
//! They enable patterns impossible with regular references like shared ownership (Rc, Arc)
//! and interior mutability (RefCell). This is how Rust provides memory safety without
//! garbage collection.

use std::rc::Rc;
use std::cell::RefCell;

/// A simple node in a linked list using Box<T>.
///
/// **Teaching: Box for recursive types**
/// - Without Box, List would be infinitely sized (compile error)
/// - With Box, List contains a pointer (fixed 8 bytes) to the next List
/// - Compiler knows the size at compile time
///
/// **From the borrow checker's perspective:**
/// - Each node owns the next node via Box
/// - When a node is dropped, it recursively drops the rest
/// - This is why Box needs Drop trait (automatic cleanup)
#[derive(Debug, Clone)]
pub enum List<T> {
    /// A node with data and a pointer to the rest of the list
    Cons(T, Box<List<T>>),
    /// The end of the list
    Nil,
}

impl<T> List<T> {
    /// Create an empty list
    pub fn nil() -> Self {
        List::Nil
    }

    /// Add an element to the front of the list
    ///
    /// **Ownership note:**
    /// - We own both head and the rest of the list
    /// - head is moved into the Box (transferred)
    /// - self is consumed (ownership moved to new list)
    /// - Returns a new list with head prepended
    pub fn cons(head: T, tail: List<T>) -> Self {
        List::Cons(head, Box::new(tail))
    }

    /// Get the length of the list
    ///
    /// **Why Box helps:**
    /// - We can recursively traverse the list
    /// - Box makes the recursive type possible
    pub fn len(&self) -> usize {
        match self {
            List::Cons(_, rest) => 1 + rest.len(),
            List::Nil => 0,
        }
    }

    /// Get the first element without consuming the list
    pub fn head(&self) -> Option<&T> {
        match self {
            List::Cons(head, _) => Some(head),
            List::Nil => None,
        }
    }
}

/// A simple binary tree node using Box<T>.
///
/// **Teaching: Box for tree structures**
/// - Each node owns its children via Box
/// - Enables building recursive structures
/// - Drop is automatically implemented
#[derive(Debug)]
pub struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    /// Create a leaf node
    pub fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }

    /// Add a left child
    pub fn with_left(mut self, left: TreeNode<T>) -> Self {
        self.left = Some(Box::new(left));
        self
    }

    /// Add a right child
    pub fn with_right(mut self, right: TreeNode<T>) -> Self {
        self.right = Some(Box::new(right));
        self
    }

    /// Get the value
    pub fn value(&self) -> &T {
        &self.value
    }

    /// Recursively sum all nodes (only works for numeric types)
    pub fn sum(&self) -> i32
    where
        T: Into<i32> + Copy,
    {
        let mut total = self.value.into();
        if let Some(ref left) = self.left {
            total += left.sum();
        }
        if let Some(ref right) = self.right {
            total += right.sum();
        }
        total
    }
}

/// A reference-counted cell for shared single-threaded ownership.
///
/// **Teaching: Rc<T> for shared ownership**
/// - Multiple owners can exist simultaneously
/// - Data is freed when the last owner drops
/// - NOT thread-safe (see Arc for thread-safe version)
/// - Rc::clone() increments reference count (cheap operation)
pub fn shared_value_example() -> Rc<Vec<i32>> {
    // **Why Rc is needed:**
    // In a graph where multiple nodes point to the same data,
    // Rc allows them all to own it without copying.
    Rc::new(vec![1, 2, 3, 4, 5])
}

/// Get the current reference count for debugging
///
/// **Teaching: Reference counting mechanics**
/// - Rc tracks how many owners exist
/// - strong_count() tells us the current count
/// - When it reaches 0, data is dropped
pub fn count_references<T>(rc: &Rc<T>) -> usize {
    Rc::strong_count(rc)
}

/// A shared mutable value using Rc<RefCell<T>>.
///
/// **Teaching: Interior mutability pattern**
/// - RefCell allows mutation through immutable references
/// - Borrowing rules are checked at RUNTIME (not compile time)
/// - Panics if you borrow mutably when already borrowed immutably
/// - Useful for observer pattern, test doubles, graphs with cycles
#[derive(Clone)]
pub struct SharedCounter {
    // **Why Rc<RefCell<>> together:**
    // - Rc: Multiple owners
    // - RefCell: Mutable access through immutable reference
    value: Rc<RefCell<i32>>,
}

impl SharedCounter {
    /// Create a new shared counter
    pub fn new(initial: i32) -> Self {
        SharedCounter {
            value: Rc::new(RefCell::new(initial)),
        }
    }

    /// Get the current value
    ///
    /// **From the borrow checker's perspective:**
    /// - borrow() returns a Ref (immutable borrow)
    /// - Panics if already mutably borrowed
    /// - This is runtime borrow checking (not compile-time)
    pub fn get(&self) -> i32 {
        *self.value.borrow()
    }

    /// Increment the value
    ///
    /// **Ownership note:**
    /// - Takes &self (immutable reference to self)
    /// - But can mutate the internal value!
    /// - borrow_mut() returns RefMut (mutable borrow)
    /// - Panics if already borrowed (immutably or mutably)
    pub fn increment(&self) {
        // **Why this is safe but different from normal Rust:**
        // - Compile-time: Normal Rust would forbid &self to mutate
        // - Runtime: RefCell allows it, panics if you violate rules
        // - Use case: Complex patterns where compile-time rules are too restrictive
        *self.value.borrow_mut() += 1;
    }

    /// Reset the counter
    pub fn reset(&self) {
        *self.value.borrow_mut() = 0;
    }

    /// Set a specific value
    pub fn set(&self, new_value: i32) {
        *self.value.borrow_mut() = new_value;
    }
}

/// A graph node that stores references to other nodes.
///
/// **Teaching: Complex ownership with Rc**
/// - Nodes can have multiple parents/neighbors
/// - Rc allows shared ownership
/// - Would create memory leaks with circular references (advanced topic)
#[derive(Clone)]
pub struct GraphNode {
    id: u32,
    neighbors: Rc<RefCell<Vec<Rc<GraphNode>>>>,
}

impl GraphNode {
    /// Create a new graph node
    pub fn new(id: u32) -> Self {
        GraphNode {
            id,
            neighbors: Rc::new(RefCell::new(Vec::new())),
        }
    }

    /// Get the node's ID
    pub fn id(&self) -> u32 {
        self.id
    }

    /// Connect this node to another
    pub fn connect_to(&self, other: Rc<GraphNode>) {
        // **Why Rc<RefCell<>>:**
        // - neighbors: Rc allows multiple nodes to share the list
        // - neighbors: RefCell allows adding neighbors without &mut self
        self.neighbors.borrow_mut().push(other);
    }

    /// Get the number of neighbors
    pub fn neighbor_count(&self) -> usize {
        self.neighbors.borrow().len()
    }

    /// Get neighbor IDs
    pub fn neighbor_ids(&self) -> Vec<u32> {
        self.neighbors
            .borrow()
            .iter()
            .map(|n| n.id)
            .collect()
    }
}

/// Demonstrates the Drop trait for custom cleanup.
///
/// **Teaching: Drop trait**
/// - Automatically called when a value goes out of scope
/// - Used for cleanup (closing files, releasing locks, etc.)
/// - Manually drop with drop() function
#[derive(Clone)]
pub struct Resource {
    name: String,
}

impl Resource {
    /// Create a new resource
    pub fn new(name: String) -> Self {
        println!("[Resource] Acquired: {}", name);
        Resource { name }
    }

    /// Get resource name
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl Drop for Resource {
    fn drop(&mut self) {
        // **Why Drop is useful:**
        // - Automatic cleanup when value goes out of scope
        // - No manual cleanup needed
        // - Different from C++ destructors: happens automatically
        println!("[Resource] Released: {}", self.name);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_list_cons_nil() {
        let list: List<i32> = List::nil();
        assert_eq!(list.len(), 0);
    }

    #[test]
    fn test_list_cons_single() {
        let list = List::cons(1, List::Nil);
        assert_eq!(list.len(), 1);
    }

    #[test]
    fn test_list_cons_multiple() {
        let list = List::cons(1, List::cons(2, List::cons(3, List::Nil)));
        assert_eq!(list.len(), 3);
    }

    #[test]
    fn test_tree_single_node() {
        let tree = TreeNode::new(5);
        assert_eq!(*tree.value(), 5);
    }

    #[test]
    fn test_tree_with_children() {
        let tree = TreeNode::new(1)
            .with_left(TreeNode::new(2))
            .with_right(TreeNode::new(3));
        assert_eq!(tree.sum(), 6);
    }

    #[test]
    fn test_shared_counter_basic() {
        let counter = SharedCounter::new(0);
        assert_eq!(counter.get(), 0);

        counter.increment();
        assert_eq!(counter.get(), 1);
    }

    #[test]
    fn test_shared_counter_multiple_owners() {
        let counter = SharedCounter::new(0);
        let counter2 = counter.clone();

        counter.increment();
        assert_eq!(counter2.get(), 1);  // Both see the same value!
    }

    #[test]
    fn test_graph_node_creation() {
        let node = Rc::new(GraphNode::new(1));
        assert_eq!(node.id(), 1);
    }

    #[test]
    fn test_graph_node_neighbors() {
        let node1 = Rc::new(GraphNode::new(1));
        let node2 = Rc::new(GraphNode::new(2));

        node1.connect_to(Rc::clone(&node2));
        assert_eq!(node1.neighbor_count(), 1);
    }

    #[test]
    fn test_resource_drop() {
        // Resource Drop will be called automatically when this goes out of scope
        let _resource = Resource::new("test".to_string());
        // Drop happens here
    }
}
