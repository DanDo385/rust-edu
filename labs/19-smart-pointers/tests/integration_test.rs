//! Integration tests for Lab 19: Smart Pointers

use smart_pointers::solution::{List, TreeNode, SharedCounter, GraphNode, Resource, shared_value_example, count_references};
use std::rc::Rc;

// ============================================================================
// LIST TESTS (Box<T> for recursive types)
// ============================================================================

#[test]
fn test_list_nil() {
    let list: List<i32> = List::nil();
    assert_eq!(list.len(), 0);
    assert_eq!(list.head(), None);
}

#[test]
fn test_list_single_element() {
    let list = List::cons(42, List::nil());
    assert_eq!(list.len(), 1);
    assert_eq!(list.head(), Some(&42));
}

#[test]
fn test_list_multiple_elements() {
    let list = List::cons(1, List::cons(2, List::cons(3, List::nil())));
    assert_eq!(list.len(), 3);
    assert_eq!(list.head(), Some(&1));
}

#[test]
fn test_list_head_method() {
    let list = List::cons(10, List::cons(20, List::nil()));
    assert_eq!(list.head(), Some(&10));
}

#[test]
fn test_list_clone() {
    let list1 = List::cons(1, List::cons(2, List::nil()));
    let list2 = list1.clone();

    assert_eq!(list1.len(), list2.len());
    assert_eq!(list1.head(), list2.head());
}

#[test]
fn test_list_string_elements() {
    let list = List::cons("hello".to_string(), List::cons("world".to_string(), List::nil()));
    assert_eq!(list.len(), 2);
    assert_eq!(list.head(), Some(&"hello".to_string()));
}

#[test]
fn test_list_empty_then_cons() {
    let mut list: List<i32> = List::nil();
    assert_eq!(list.len(), 0);

    list = List::cons(5, list);
    assert_eq!(list.len(), 1);
    assert_eq!(list.head(), Some(&5));
}

#[test]
fn test_list_large_chain() {
    let mut list = List::nil();
    for i in 1..=10 {
        list = List::cons(i, list);
    }
    assert_eq!(list.len(), 10);
}

#[test]
fn test_list_ownership_move() {
    let list1 = List::cons(1, List::nil());
    let list2 = list1;  // Move
    assert_eq!(list2.len(), 1);
    // list1 is no longer valid (moved to list2)
}

// ============================================================================
// TREE TESTS (Box<T> for tree structures)
// ============================================================================

#[test]
fn test_tree_single_node() {
    let tree = TreeNode::new(42);
    assert_eq!(*tree.value(), 42);
}

#[test]
fn test_tree_with_left_child() {
    let tree = TreeNode::new(1)
        .with_left(TreeNode::new(2));
    assert_eq!(*tree.value(), 1);
}

#[test]
fn test_tree_with_right_child() {
    let tree = TreeNode::new(1)
        .with_right(TreeNode::new(3));
    assert_eq!(*tree.value(), 1);
}

#[test]
fn test_tree_with_both_children() {
    let tree = TreeNode::new(1)
        .with_left(TreeNode::new(2))
        .with_right(TreeNode::new(3));

    assert_eq!(*tree.value(), 1);
    assert_eq!(tree.sum(), 6);  // 1 + 2 + 3
}

#[test]
fn test_tree_unbalanced() {
    let tree = TreeNode::new(1)
        .with_left(TreeNode::new(2)
            .with_left(TreeNode::new(3)));

    assert_eq!(tree.sum(), 6);
}

#[test]
fn test_tree_sum_single() {
    let tree = TreeNode::new(5);
    assert_eq!(tree.sum(), 5);
}

#[test]
fn test_tree_complex_structure() {
    let tree = TreeNode::new(4)
        .with_left(
            TreeNode::new(2)
                .with_left(TreeNode::new(1))
                .with_right(TreeNode::new(3))
        )
        .with_right(
            TreeNode::new(6)
                .with_left(TreeNode::new(5))
                .with_right(TreeNode::new(7))
        );

    // Sum: 4+2+1+3+6+5+7 = 28
    assert_eq!(tree.sum(), 28);
}

#[test]
fn test_tree_deep_nesting() {
    let tree = TreeNode::new(1)
        .with_left(TreeNode::new(2)
            .with_left(TreeNode::new(3)
                .with_left(TreeNode::new(4))));

    assert_eq!(tree.sum(), 10);  // 1+2+3+4
}

// ============================================================================
// RC SHARED OWNERSHIP TESTS
// ============================================================================

#[test]
fn test_shared_value_example() {
    let value = shared_value_example();
    assert_eq!(value.len(), 5);
    assert_eq!(value[0], 1);
}

#[test]
fn test_count_references_single() {
    let value = shared_value_example();
    assert_eq!(count_references(&value), 1);
}

#[test]
fn test_count_references_cloned() {
    let value = shared_value_example();
    let value2 = Rc::clone(&value);

    assert_eq!(count_references(&value), 2);
    assert_eq!(count_references(&value2), 2);
}

#[test]
fn test_count_references_multiple_clones() {
    let value = shared_value_example();
    let v2 = Rc::clone(&value);
    let v3 = Rc::clone(&value);
    let v4 = Rc::clone(&value);

    assert_eq!(count_references(&value), 4);
}

// ============================================================================
// SHARED COUNTER TESTS (Rc<RefCell<T>> for interior mutability)
// ============================================================================

#[test]
fn test_shared_counter_creation() {
    let counter = SharedCounter::new(0);
    assert_eq!(counter.get(), 0);
}

#[test]
fn test_shared_counter_increment() {
    let counter = SharedCounter::new(5);
    counter.increment();
    assert_eq!(counter.get(), 6);
}

#[test]
fn test_shared_counter_multiple_increments() {
    let counter = SharedCounter::new(0);
    for _ in 0..5 {
        counter.increment();
    }
    assert_eq!(counter.get(), 5);
}

#[test]
fn test_shared_counter_reset() {
    let counter = SharedCounter::new(10);
    counter.reset();
    assert_eq!(counter.get(), 0);
}

#[test]
fn test_shared_counter_set() {
    let counter = SharedCounter::new(0);
    counter.set(42);
    assert_eq!(counter.get(), 42);
}

#[test]
fn test_shared_counter_clone() {
    let counter = SharedCounter::new(10);
    let counter2 = counter.clone();

    // Both point to the same value
    assert_eq!(counter.get(), 10);
    assert_eq!(counter2.get(), 10);
}

#[test]
fn test_shared_counter_shared_increment() {
    let counter = SharedCounter::new(0);
    let counter2 = counter.clone();

    counter.increment();
    // counter2 sees the change (they share the same value)
    assert_eq!(counter2.get(), 1);
}

#[test]
fn test_shared_counter_multiple_mutations() {
    let counter = SharedCounter::new(0);
    let c2 = counter.clone();
    let c3 = counter.clone();

    counter.increment();
    c2.increment();
    c3.increment();

    // All see the final value
    assert_eq!(counter.get(), 3);
    assert_eq!(c2.get(), 3);
    assert_eq!(c3.get(), 3);
}

#[test]
fn test_shared_counter_negative() {
    let counter = SharedCounter::new(5);
    counter.set(-10);
    assert_eq!(counter.get(), -10);
}

// ============================================================================
// GRAPH NODE TESTS (Rc<RefCell<>> for complex ownership)
// ============================================================================

#[test]
fn test_graph_node_creation() {
    let node = Rc::new(GraphNode::new(1));
    assert_eq!(node.id(), 1);
    assert_eq!(node.neighbor_count(), 0);
}

#[test]
fn test_graph_node_single_neighbor() {
    let node1 = Rc::new(GraphNode::new(1));
    let node2 = Rc::new(GraphNode::new(2));

    node1.connect_to(Rc::clone(&node2));
    assert_eq!(node1.neighbor_count(), 1);
}

#[test]
fn test_graph_node_multiple_neighbors() {
    let node1 = Rc::new(GraphNode::new(1));
    let node2 = Rc::new(GraphNode::new(2));
    let node3 = Rc::new(GraphNode::new(3));

    node1.connect_to(Rc::clone(&node2));
    node1.connect_to(Rc::clone(&node3));

    assert_eq!(node1.neighbor_count(), 2);
}

#[test]
fn test_graph_node_neighbor_ids() {
    let node1 = Rc::new(GraphNode::new(1));
    let node2 = Rc::new(GraphNode::new(2));
    let node3 = Rc::new(GraphNode::new(3));

    node1.connect_to(Rc::clone(&node2));
    node1.connect_to(Rc::clone(&node3));

    let ids = node1.neighbor_ids();
    assert_eq!(ids.len(), 2);
    assert!(ids.contains(&2));
    assert!(ids.contains(&3));
}

#[test]
fn test_graph_directed_edge() {
    let node1 = Rc::new(GraphNode::new(1));
    let node2 = Rc::new(GraphNode::new(2));

    node1.connect_to(Rc::clone(&node2));

    // node1 has node2 as neighbor
    assert_eq!(node1.neighbor_count(), 1);
    // but node2 doesn't have node1 as neighbor (directed)
    assert_eq!(node2.neighbor_count(), 0);
}

#[test]
fn test_graph_bidirectional() {
    let node1 = Rc::new(GraphNode::new(1));
    let node2 = Rc::new(GraphNode::new(2));

    node1.connect_to(Rc::clone(&node2));
    node2.connect_to(Rc::clone(&node1));

    assert_eq!(node1.neighbor_count(), 1);
    assert_eq!(node2.neighbor_count(), 1);
}

#[test]
fn test_graph_star_topology() {
    let center = Rc::new(GraphNode::new(0));

    for i in 1..=5 {
        let node = Rc::new(GraphNode::new(i));
        center.connect_to(node);
    }

    assert_eq!(center.neighbor_count(), 5);
}

#[test]
fn test_graph_clone_shares_neighbors() {
    let node1 = Rc::new(GraphNode::new(1));
    let node2 = Rc::new(GraphNode::new(2));

    node1.connect_to(Rc::clone(&node2));

    let node1_clone = Rc::clone(&node1);
    // Both references see the same neighbors
    assert_eq!(node1_clone.neighbor_count(), 1);
}

// ============================================================================
// RESOURCE AND DROP TESTS
// ============================================================================

#[test]
fn test_resource_creation() {
    let _resource = Resource::new("test".to_string());
    // Drop will be called automatically
}

#[test]
fn test_resource_name() {
    let resource = Resource::new("myresource".to_string());
    assert_eq!(resource.name(), "myresource");
}

#[test]
fn test_resource_clone() {
    let r1 = Resource::new("resource1".to_string());
    let r2 = r1.clone();

    assert_eq!(r1.name(), r2.name());
}

#[test]
fn test_resource_scope() {
    {
        let _resource = Resource::new("scoped".to_string());
        // Drop called when leaving scope
    }
}

// ============================================================================
// INTEGRATION TESTS (Multiple smart pointers together)
// ============================================================================

#[test]
fn test_list_of_shared_counters() {
    let c1 = SharedCounter::new(1);
    let c2 = SharedCounter::new(2);

    let list = List::cons(c1, List::cons(c2, List::nil()));
    assert_eq!(list.len(), 2);
}

#[test]
fn test_tree_of_shared_counters() {
    let c1 = SharedCounter::new(1);
    let c2 = SharedCounter::new(2);
    let c3 = SharedCounter::new(3);

    let tree = TreeNode::new(c1)
        .with_left(TreeNode::new(c2))
        .with_right(TreeNode::new(c3));

    assert_eq!(tree.value().get(), 1);
}

#[test]
fn test_graph_of_shared_resources() {
    let node1 = Rc::new(GraphNode::new(1));
    let node2 = Rc::new(GraphNode::new(2));

    node1.connect_to(Rc::clone(&node2));
    node2.connect_to(Rc::clone(&node1));

    assert_eq!(node1.neighbor_count(), 1);
    assert_eq!(node2.neighbor_count(), 1);
}

#[test]
fn test_complex_ownership_patterns() {
    // Multiple shared counters
    let c1 = SharedCounter::new(10);
    let c2 = c1.clone();
    let c3 = c1.clone();

    // Modify through one reference
    c1.increment();

    // All see the change
    assert_eq!(c1.get(), 11);
    assert_eq!(c2.get(), 11);
    assert_eq!(c3.get(), 11);
}

#[test]
fn test_list_ownership_and_modification() {
    let c = SharedCounter::new(0);
    let list = List::cons(c.clone(), List::nil());

    c.increment();
    assert_eq!(list.head().unwrap().get(), 1);
}
