// Lab 30: Lock-Free Structure
//
// Implementation of a lock-free stack using atomic operations and Compare-And-Swap (CAS).
// Demonstrates low-level concurrent programming, memory ordering, and the challenges
// of lock-free data structures.
//
// Key Concepts:
// - AtomicPtr for lock-free pointer manipulation
// - Compare-And-Swap (CAS) loops for atomic updates
// - Memory ordering (Acquire/Release) for cross-thread visibility
// - Manual Send/Sync implementations for raw-pointer types
// - RAII cleanup via Drop for heap-allocated nodes
//
// Note: This is educational. Use crossbeam in production for proper
// memory reclamation and ABA problem mitigation.

use std::ptr;
use std::sync::atomic::{AtomicPtr, AtomicUsize, Ordering};

// ============================================================================
// NODE STRUCTURE (INTERNAL)
// ============================================================================

struct Node<T> {
    data: T,
    next: *mut Node<T>,
}

// ============================================================================
// LOCK-FREE STACK
// ============================================================================

/// A lock-free stack implemented using atomic Compare-And-Swap operations.
///
/// This stack is thread-safe without using locks. Instead, it uses a CAS loop:
/// 1. Read the current head pointer
/// 2. Prepare the new state
/// 3. Atomically swap if head hasn't changed (CAS)
/// 4. If CAS fails (another thread modified head), retry
///
/// # Memory Layout
/// ```text
/// LockFreeStack
///   ├── head: AtomicPtr<Node<T>>  ──> Node { data: 3, next: ─┐ }
///   └── len: AtomicUsize                   Node { data: 2, next: ─┐ }
///                                          Node { data: 1, next: null }
/// ```
///
/// # Thread Safety
/// - `Send`: The stack can be sent between threads
/// - `Sync`: The stack can be shared between threads via references
/// - All operations use atomic instructions (no locks needed)
pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
    len: AtomicUsize,
}

impl<T> LockFreeStack<T> {
    /// Creates a new empty lock-free stack.
    pub fn new() -> Self {
        LockFreeStack {
            head: AtomicPtr::new(ptr::null_mut()),
            len: AtomicUsize::new(0),
        }
    }

    /// Pushes a value onto the top of the stack.
    ///
    /// Uses a CAS loop:
    /// 1. Allocate a new node on the heap via `Box::into_raw`
    /// 2. Read the current head (Acquire ordering for visibility)
    /// 3. Set new node's `next` to current head
    /// 4. CAS: if head is still what we read, update it to new node
    /// 5. If CAS fails (another thread changed head), retry from step 2
    ///
    /// This is lock-free: no thread can block another. Under contention,
    /// threads retry the CAS but never sleep or wait on a lock.
    pub fn push(&self, data: T) {
        // Allocate node on the heap; Box::into_raw gives us a raw pointer
        let new_node = Box::into_raw(Box::new(Node {
            data,
            next: ptr::null_mut(),
        }));

        loop {
            // Acquire: see all writes that happened before the last Release store
            let old_head = self.head.load(Ordering::Acquire);

            // Point new node's next to current head
            // SAFETY: new_node is valid; we just allocated it
            unsafe {
                (*new_node).next = old_head;
            }

            // CAS: atomically update head from old_head to new_node
            match self.head.compare_exchange(
                old_head,
                new_node,
                Ordering::Release,  // Success: publish the new node to other threads
                Ordering::Acquire,  // Failure: re-read with synchronization
            ) {
                Ok(_) => {
                    self.len.fetch_add(1, Ordering::Relaxed);
                    break;
                }
                Err(_) => {
                    // Another thread changed head; retry
                    continue;
                }
            }
        }
    }

    /// Pops the top value from the stack, returning `None` if empty.
    ///
    /// Uses a CAS loop:
    /// 1. Read the current head (Acquire ordering)
    /// 2. If head is null, return None (empty stack)
    /// 3. Read head's `next` pointer
    /// 4. CAS: if head is still what we read, update it to `next`
    /// 5. If CAS fails, retry from step 1
    /// 6. On success, reconstruct the Box and extract data
    ///
    /// # Safety Note
    /// After a successful CAS, we own the old head node exclusively because
    /// no other thread can reach it (it's been removed from the linked list).
    pub fn pop(&self) -> Option<T> {
        loop {
            let old_head = self.head.load(Ordering::Acquire);

            if old_head.is_null() {
                return None;
            }

            // SAFETY: old_head is not null (checked above).
            // In a single-pop scenario this is safe because CAS ensures
            // only one thread successfully removes this node.
            let next = unsafe { (*old_head).next };

            match self.head.compare_exchange(
                old_head,
                next,
                Ordering::Release,
                Ordering::Acquire,
            ) {
                Ok(_) => {
                    self.len.fetch_sub(1, Ordering::Relaxed);

                    // SAFETY: We won the CAS, so we exclusively own old_head.
                    // Reconstruct the Box to take ownership and extract data.
                    let node = unsafe { Box::from_raw(old_head) };
                    return Some(node.data);
                }
                Err(_) => {
                    // Another thread changed head; retry
                    continue;
                }
            }
        }
    }

    /// Returns the approximate length of the stack.
    ///
    /// This value is racy: another thread may push or pop between
    /// reading `len` and acting on the result. Use for diagnostics only.
    pub fn len(&self) -> usize {
        self.len.load(Ordering::Relaxed)
    }

    /// Returns true if the stack appears to be empty.
    ///
    /// Like `len()`, this is an approximation in concurrent contexts.
    pub fn is_empty(&self) -> bool {
        self.head.load(Ordering::Acquire).is_null()
    }
}

impl<T> Default for LockFreeStack<T> {
    fn default() -> Self {
        Self::new()
    }
}

// ============================================================================
// DROP IMPLEMENTATION
// ============================================================================
// Walk the linked list and free every node.
// Safe because Drop takes &mut self (exclusive access guaranteed).

impl<T> Drop for LockFreeStack<T> {
    fn drop(&mut self) {
        let mut current = self.head.load(Ordering::Relaxed);

        while !current.is_null() {
            // SAFETY: We have exclusive access (&mut self).
            // Each node is valid and was heap-allocated via Box::into_raw.
            unsafe {
                let node = Box::from_raw(current);
                current = node.next;
                // node is dropped here, freeing the heap allocation
            }
        }
    }
}

// ============================================================================
// SEND + SYNC
// ============================================================================
// Raw pointers are !Send and !Sync by default. We implement these manually
// because our atomic operations guarantee thread safety.
//
// Requirements:
// - T: Send because values are moved between threads (push on one, pop on another)
// - The stack itself is safe to share (&self methods use atomics, not locks)

// SAFETY: LockFreeStack can be sent between threads.
// All internal state is either atomic or behind raw pointers that are
// only accessed through atomic CAS operations.
unsafe impl<T: Send> Send for LockFreeStack<T> {}

// SAFETY: LockFreeStack can be shared between threads via &LockFreeStack.
// push() and pop() use CAS loops on AtomicPtr, ensuring only one thread
// succeeds in modifying the head at a time.
unsafe impl<T: Send> Sync for LockFreeStack<T> {}

// ============================================================================
// OWNERSHIP & MEMORY MODEL
// ============================================================================
//
// PUSH:
//   1. Box::new(Node { data, next: null }) -- allocate on heap
//   2. Box::into_raw(box) -- convert to raw pointer, Box forgets ownership
//   3. AtomicPtr stores the raw pointer
//   4. Ownership is now "in the stack" (the linked list owns the nodes)
//
// POP:
//   1. CAS removes node from linked list
//   2. Box::from_raw(ptr) -- reconstruct Box, reclaiming ownership
//   3. Extract data from Box<Node>
//   4. Box is dropped, freeing the Node's heap memory
//
// DROP:
//   1. Walk the list from head to null
//   2. Box::from_raw each node
//   3. Box drop frees each node
//   4. All heap memory reclaimed
//
// CAS (Compare-And-Swap):
//   Hardware instruction (CMPXCHG on x86, LL/SC on ARM).
//   Atomically: if *ptr == expected { *ptr = new; return Ok } else { return Err }
//   This is the fundamental building block of lock-free programming.
//
// MEMORY ORDERING:
//   Acquire (loads): "I want to see everything that happened before the
//                     matching Release store"
//   Release (stores): "Everything I wrote before this store must be visible
//                      to threads that do an Acquire load"
//   Relaxed: Only atomicity guaranteed, no ordering with other operations.
//            Used for the len counter because exact ordering doesn't matter.
