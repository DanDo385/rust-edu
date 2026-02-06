// Project 16: Smart Pointers
//
// Smart pointers are data structures that act like pointers but have additional
// metadata and capabilities. They implement the Deref and Drop traits, enabling
// automatic dereferencing and custom cleanup logic. This project explores Rust's
// built-in smart pointers and when to use each.

use std::rc::Rc;
use std::sync::Arc;
use std::cell::RefCell;
use std::thread;

fn main() {
    println!("=== Rust Smart Pointers ===\n");

    demonstrate_box();
    demonstrate_rc();
    demonstrate_arc();
    demonstrate_refcell();
    demonstrate_tree_structure();
    demonstrate_reference_counting();
}

// ============================================================================
// BOX<T>: HEAP ALLOCATION
// ============================================================================
// Box<T> is the simplest smart pointer. It stores data on the HEAP instead
// of the STACK. Use Box when:
// 1. You have data of unknown size at compile time (recursive types)
// 2. You have large data and want to transfer ownership without copying
// 3. You want trait objects (dynamic dispatch)

fn demonstrate_box() {
    println!("--- Box<T>: Heap Allocation ---");

    // Simple heap allocation
    let boxed_int = Box::new(5);
    println!("Boxed integer: {}", boxed_int);

    // Box automatically dereferences
    let sum = *boxed_int + 10;
    println!("Dereferenced sum: {}", sum);

    // Large data on heap (avoids stack overflow for large arrays)
    let boxed_array = Box::new([0; 1000]);
    println!("Boxed array first element: {}", boxed_array[0]);

    // RECURSIVE TYPES: The killer feature of Box
    // Without Box, this would be infinitely sized (compile error)
    #[derive(Debug)]
    enum List {
        Cons(i32, Box<List>),  // Recursive! Box makes the size known
        Nil,
    }

    use List::{Cons, Nil};

    let list = Cons(1, Box::new(Cons(2, Box::new(Cons(3, Box::new(Nil))))));
    println!("Recursive list: {:?}", list);

    // WHY THIS WORKS:
    // - Without Box: List contains List contains List... (infinite size!)
    // - With Box: List contains a POINTER to List (fixed size: 8 bytes on 64-bit)
    // - The compiler knows the size of a pointer at compile time

    // TRAIT OBJECTS: Dynamic dispatch
    trait Drawable {
        fn draw(&self);
    }

    struct Circle;
    struct Square;

    impl Drawable for Circle {
        fn draw(&self) {
            println!("Drawing a circle");
        }
    }

    impl Drawable for Square {
        fn draw(&self) {
            println!("Drawing a square");
        }
    }

    // Box allows storing different types that implement the same trait
    let shapes: Vec<Box<dyn Drawable>> = vec![
        Box::new(Circle),
        Box::new(Square),
    ];

    for shape in shapes {
        shape.draw();  // Dynamic dispatch at runtime
    }

    println!();
}

// ============================================================================
// RC<T>: REFERENCE COUNTED SHARED OWNERSHIP (SINGLE-THREADED)
// ============================================================================
// Rc<T> (Reference Counted) enables multiple owners of the same data.
// When the last owner goes out of scope, the data is freed.
// ⚠️ ONLY for single-threaded code! Not Send or Sync.

fn demonstrate_rc() {
    println!("--- Rc<T>: Reference Counted Ownership ---");

    // Create an Rc
    let data = Rc::new(vec![1, 2, 3]);
    println!("Initial reference count: {}", Rc::strong_count(&data));

    // Clone creates a new reference (NOT a deep copy!)
    let data_ref1 = Rc::clone(&data);
    println!("After first clone: {}", Rc::strong_count(&data));

    let data_ref2 = Rc::clone(&data);
    println!("After second clone: {}", Rc::strong_count(&data));

    // All three variables point to the SAME data
    println!("data:      {:?}", data);
    println!("data_ref1: {:?}", data_ref1);
    println!("data_ref2: {:?}", data_ref2);

    // Drop one reference
    drop(data_ref1);
    println!("After dropping data_ref1: {}", Rc::strong_count(&data));

    // WHY USE RC?
    // Imagine a graph where multiple nodes point to the same node.
    // Without Rc, who owns that shared node? With Rc, they all do!

    // COMMON PATTERN: Multiple readers
    struct Database {
        data: Rc<Vec<String>>,
    }

    let shared_data = Rc::new(vec![
        String::from("Alice"),
        String::from("Bob"),
        String::from("Charlie"),
    ]);

    let db1 = Database {
        data: Rc::clone(&shared_data),
    };

    let db2 = Database {
        data: Rc::clone(&shared_data),
    };

    println!("DB1 has {} entries", db1.data.len());
    println!("DB2 has {} entries", db2.data.len());
    println!("Reference count: {}", Rc::strong_count(&shared_data));

    // When db1, db2, and shared_data go out of scope, the Vec is freed

    println!();
}

// ============================================================================
// ARC<T>: ATOMIC REFERENCE COUNTED (THREAD-SAFE)
// ============================================================================
// Arc<T> is the thread-safe version of Rc<T>. It uses atomic operations
// for the reference count, making it safe to share across threads.
// Use Arc when you need shared ownership across threads.

fn demonstrate_arc() {
    println!("--- Arc<T>: Thread-Safe Reference Counting ---");

    // Create shared data
    let data = Arc::new(vec![1, 2, 3, 4, 5]);
    println!("Initial Arc count: {}", Arc::strong_count(&data));

    let mut handles = vec![];

    // Spawn 3 threads, each getting a clone of the Arc
    for i in 0..3 {
        let data_clone = Arc::clone(&data);

        let handle = thread::spawn(move || {
            // Each thread can read the shared data
            println!("Thread {} sees: {:?}", i, data_clone);

            // Calculate sum
            let sum: i32 = data_clone.iter().sum();
            println!("Thread {} calculated sum: {}", i, sum);
        });

        handles.push(handle);
    }

    // Wait for all threads to complete
    for handle in handles {
        handle.join().unwrap();
    }

    println!("Final Arc count (back to 1): {}", Arc::strong_count(&data));

    // ARC VS RC:
    // - Arc uses atomic operations (slower but thread-safe)
    // - Rc uses regular operations (faster but single-threaded only)
    // - Arc is Send + Sync, Rc is neither
    // - Use Rc unless you need thread safety

    println!();
}

// ============================================================================
// REFCELL<T>: INTERIOR MUTABILITY
// ============================================================================
// RefCell<T> allows mutation even when there are immutable references.
// It enforces borrowing rules at RUNTIME instead of compile time.
// ⚠️ Will PANIC if you violate borrowing rules at runtime!
// ⚠️ SINGLE-THREADED only! Use Mutex for thread-safe interior mutability.

fn demonstrate_refcell() {
    println!("--- RefCell<T>: Interior Mutability ---");

    // Problem: How to mutate data that has immutable references?
    // Normal Rust: Can't do this!
    // let x = 5;
    // let r = &x;
    // x = 6;  // ❌ ERROR: cannot assign to `x` because it is borrowed

    // Solution: RefCell moves the check to runtime
    let x = RefCell::new(5);

    // Borrow immutably (runtime borrow check)
    {
        let r1 = x.borrow();  // Returns Ref<T>
        let r2 = x.borrow();  // Multiple immutable borrows OK
        println!("r1: {}, r2: {}", r1, r2);
    }  // Borrows dropped here

    // Borrow mutably (runtime borrow check)
    {
        let mut r = x.borrow_mut();  // Returns RefMut<T>
        *r += 10;
        println!("After mutation: {}", *r);
    }  // Mutable borrow dropped here

    // Read the final value
    println!("Final value: {}", x.borrow());

    // COMMON PATTERN: Rc<RefCell<T>>
    // This gives you shared ownership AND interior mutability!
    #[derive(Debug)]
    struct SharedCounter {
        count: Rc<RefCell<i32>>,
    }

    let counter = Rc::new(RefCell::new(0));

    let c1 = SharedCounter {
        count: Rc::clone(&counter),
    };

    let c2 = SharedCounter {
        count: Rc::clone(&counter),
    };

    // Both c1 and c2 can mutate the shared counter
    *c1.count.borrow_mut() += 1;
    println!("After c1 increment: {}", c1.count.borrow());

    *c2.count.borrow_mut() += 1;
    println!("After c2 increment: {}", c2.count.borrow());

    println!("Final counter value: {}", counter.borrow());

    // RUNTIME PANIC EXAMPLE (commented out to avoid crash):
    // let bad = RefCell::new(5);
    // let r1 = bad.borrow_mut();
    // let r2 = bad.borrow_mut();  // ❌ PANIC: already borrowed mutably!

    println!();
}

// ============================================================================
// BUILDING A TREE STRUCTURE
// ============================================================================
// Trees are a perfect use case for smart pointers. Let's build a binary tree
// where nodes can be shared (Rc) and mutated (RefCell).

#[derive(Debug)]
struct TreeNode {
    value: i32,
    left: Option<Rc<RefCell<TreeNode>>>,
    right: Option<Rc<RefCell<TreeNode>>>,
}

impl TreeNode {
    fn new(value: i32) -> Rc<RefCell<Self>> {
        Rc::new(RefCell::new(TreeNode {
            value,
            left: None,
            right: None,
        }))
    }

    fn insert_left(&mut self, value: i32) -> Rc<RefCell<TreeNode>> {
        let node = TreeNode::new(value);
        self.left = Some(Rc::clone(&node));
        node
    }

    fn insert_right(&mut self, value: i32) -> Rc<RefCell<TreeNode>> {
        let node = TreeNode::new(value);
        self.right = Some(Rc::clone(&node));
        node
    }
}

fn print_tree(node: &Option<Rc<RefCell<TreeNode>>>, prefix: String, is_left: bool) {
    if let Some(n) = node {
        let borrowed = n.borrow();
        println!("{}{}── {}", prefix, if is_left { "├" } else { "└" }, borrowed.value);

        let extension = if is_left { "│   " } else { "    " };

        if borrowed.left.is_some() {
            print_tree(&borrowed.left, format!("{}{}", prefix, extension), true);
        }

        if borrowed.right.is_some() {
            print_tree(&borrowed.right, format!("{}{}", prefix, extension), false);
        }
    }
}

fn demonstrate_tree_structure() {
    println!("--- Tree Structure with Smart Pointers ---");

    // Build a tree:
    //       10
    //      /  \
    //     5    15
    //    / \   / \
    //   3   7 12  20

    let root = TreeNode::new(10);

    {
        let mut root_mut = root.borrow_mut();
        let left = root_mut.insert_left(5);
        let right = root_mut.insert_right(15);

        // Add children to left subtree
        left.borrow_mut().insert_left(3);
        left.borrow_mut().insert_right(7);

        // Add children to right subtree
        right.borrow_mut().insert_left(12);
        right.borrow_mut().insert_right(20);
    }

    println!("Binary Tree:");
    print_tree(&Some(root), String::new(), false);

    // WHY THIS WORKS:
    // - Rc allows multiple children to share the same parent
    // - RefCell allows mutation even though parent might have multiple references
    // - When the root goes out of scope, entire tree is cleaned up automatically!

    println!();
}

// ============================================================================
// REFERENCE COUNTING IN ACTION
// ============================================================================
// Let's watch reference counts change as we clone and drop.

fn demonstrate_reference_counting() {
    println!("--- Reference Counting Behavior ---");

    let data = Rc::new(String::from("Shared data"));
    println!("1. Created data: count = {}", Rc::strong_count(&data));

    {
        let ref1 = Rc::clone(&data);
        println!("2. Cloned to ref1: count = {}", Rc::strong_count(&data));

        {
            let ref2 = Rc::clone(&data);
            println!("3. Cloned to ref2: count = {}", Rc::strong_count(&data));

            let ref3 = Rc::clone(&data);
            println!("4. Cloned to ref3: count = {}", Rc::strong_count(&data));

        }  // ref2 and ref3 dropped here

        println!("5. ref2 and ref3 dropped: count = {}", Rc::strong_count(&data));

    }  // ref1 dropped here

    println!("6. ref1 dropped: count = {}", Rc::strong_count(&data));
    println!("7. Value still accessible: {}", data);

}  // data dropped here, memory freed

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Box<T> is for simple heap allocation and recursive types
// 2. Rc<T> enables shared ownership with reference counting (single-threaded)
// 3. Arc<T> is the thread-safe version of Rc (use atomic operations)
// 4. RefCell<T> provides interior mutability with runtime borrow checking
// 5. Combine Rc<RefCell<T>> for shared ownership + mutation
// 6. Combine Arc<Mutex<T>> for thread-safe shared ownership + mutation
// 7. Smart pointers implement Deref (auto-deref) and Drop (custom cleanup)
// 8. Reference cycles can leak memory - use Weak<T> to break cycles
// 9. Choose wisely: Box (heap), Rc (share), Arc (threads), RefCell (mutate)
// 10. Understanding smart pointers is key to building complex data structures

// ============================================================================
// COMMON MISTAKES
// ============================================================================
// ❌ Using Rc in multithreaded code (use Arc instead)
// ❌ Creating reference cycles without Weak<T> (memory leak!)
// ❌ Using RefCell when you don't need interior mutability
// ❌ Calling borrow_mut() while already borrowed (runtime panic!)
// ❌ Using Rc::clone(&x) when you just need &x (unnecessary ref count increment)
// ❌ Not understanding the difference between clone() and Rc::clone()
//    - clone() duplicates data (expensive)
//    - Rc::clone() increments ref count (cheap)
// ❌ Using Box when stack allocation would work fine
// ❌ Forgetting that RefCell is single-threaded only
// ❌ Not considering performance overhead of reference counting
// ❌ Using interior mutability as default (prefer immutability!)

// ============================================================================
// PERFORMANCE NOTES
// ============================================================================
// Box<T>:     One heap allocation, one pointer dereference
// Rc<T>:      Two heap allocations (data + counters), ref counting overhead
// Arc<T>:     Like Rc but with atomic operations (slower)
// RefCell<T>: Runtime borrow checking overhead (small)
//
// RULE OF THUMB:
// 1. Use stack allocation by default (fastest)
// 2. Use Box for heap allocation (still very fast)
// 3. Use Rc only when you truly need shared ownership
// 4. Use Arc only when you need thread safety
// 5. Use RefCell only when you can't satisfy borrow checker at compile time
// 6. Always prefer compile-time checking over runtime checking when possible
