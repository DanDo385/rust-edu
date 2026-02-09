use std::ptr;
use std::sync::atomic::{AtomicPtr, Ordering};

/// Node held on the heap. `next` is a raw pointer to the next node.
pub struct Node<T> {
    val: T,
    next: *mut Node<T>,
}

/// A simple lock-free stack using atomic compare-and-swap.
pub struct LockFreeStack<T> {
    head: AtomicPtr<Node<T>>,
}

impl<T> LockFreeStack<T> {
    /// Creates an empty stack. `AtomicPtr::new` stores a null pointer on the stack.
    pub fn new() -> Self {
        Self {
            head: AtomicPtr::new(ptr::null_mut()),
        }
    }

    /// Pushes a new node by atomically swapping the head pointer.
    ///
    /// Ownership: `Box::new` allocates the node on the heap; `Box::into_raw` turns it
    /// into a raw pointer so we can manage it manually inside the loop.
    pub fn push(&self, val: T) {
        let new_node = Box::into_raw(Box::new(Node {
            val,
            next: ptr::null_mut(),
        }));

        loop {
            let head = self.head.load(Ordering::Acquire);
            // SAFETY: new_node is uniquely owned in this loop until successful CAS.
            unsafe {
                (*new_node).next = head;
            }

            if self
                .head
                .compare_exchange(head, new_node, Ordering::Release, Ordering::Relaxed)
                .is_ok()
            {
                break;
            }
        }
    }

    /// Pops a node by atomically moving the head pointer.
    pub fn pop(&self) -> Option<T> {
        loop {
            let head = self.head.load(Ordering::Acquire);
            if head.is_null() {
                return None;
            }

            // SAFETY: head was observed non-null; reading next is valid while node remains linked.
            let next = unsafe { (*head).next };

            if self
                .head
                .compare_exchange(head, next, Ordering::AcqRel, Ordering::Acquire)
                .is_ok()
            {
                // SAFETY: successful CAS gives exclusive ownership of old head.
                let boxed = unsafe { Box::from_raw(head) };
                return Some(boxed.val);
            }
        }
    }
}

impl<T> Drop for LockFreeStack<T> {
    /// Drain the stack during drop to avoid memory leaks.
    fn drop(&mut self) {
        while self.pop().is_some() {}
    }
}
