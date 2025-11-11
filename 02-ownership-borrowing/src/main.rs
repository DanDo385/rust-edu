//! # Ownership and Borrowing - Interactive Demo
//!
//! This binary demonstrates the functions from our library.
//! Run with: cargo run -p ownership-borrowing

use ownership_borrowing::solution::*;

fn main() {
    println!("=== Ownership and Borrowing Demo ===\n");

    // ============================================================================
    // DEMO 1: add_exclamation - Taking Ownership
    // ============================================================================

    println!("1. Taking Ownership (add_exclamation):");
    println!("   -------------------------------------");

    let original = String::from("Hello");
    println!("   Before: original = \"{}\"", original);

    // Call add_exclamation - this MOVES original
    let result = add_exclamation(original);

    // original is no longer valid here!
    // println!("   After: original = \"{}\"", original);  // ❌ Won't compile!

    println!("   After: result = \"{}\"", result);
    println!("   Note: 'original' was moved and is no longer valid!\n");

    // ============================================================================
    // DEMO 2: get_length - Immutable Borrowing
    // ============================================================================

    println!("2. Immutable Borrowing (get_length):");
    println!("   ----------------------------------");

    let text = String::from("Hello, Rust!");
    println!("   text = \"{}\"", text);

    // Borrow immutably with &
    let len = get_length(&text);

    println!("   length = {}", len);
    // text is still valid! We only borrowed it.
    println!("   text is still valid: \"{}\"", text);
    println!("   We can use it again: \"{}\"", text);
    println!();

    // ============================================================================
    // DEMO 3: make_uppercase - Mutable Borrowing
    // ============================================================================

    println!("3. Mutable Borrowing (make_uppercase):");
    println!("   ------------------------------------");

    let mut changeable = String::from("rust is awesome");
    println!("   Before: changeable = \"{}\"", changeable);

    // Borrow mutably with &mut
    make_uppercase(&mut changeable);

    // changeable was modified in place!
    println!("   After: changeable = \"{}\"", changeable);
    println!("   Same variable, modified in place!\n");

    // ============================================================================
    // DEMO 4: demonstrate_copy_vs_move
    // ============================================================================

    println!("4. Copy vs Move:");
    println!("   -------------");

    let explanation = demonstrate_copy_vs_move();
    println!("{}", explanation);

    // ============================================================================
    // EXTRA DEMONSTRATION: Borrowing Rules
    // ============================================================================

    println!("\n5. Borrowing Rules in Action:");
    println!("   ---------------------------");

    let mut data = String::from("data");

    // Multiple immutable borrows are OK
    let r1 = &data;
    let r2 = &data;
    println!("   Multiple immutable borrows: r1=\"{}\", r2=\"{}\"", r1, r2);
    // r1 and r2 go out of scope here

    // Now we can have a mutable borrow
    let r3 = &mut data;
    r3.push_str("!");
    println!("   Mutable borrow modified data: r3=\"{}\"", r3);
    // r3 goes out of scope here

    // Now data is accessible again
    println!("   Original variable accessible again: data=\"{}\"", data);

    println!("\n   This won't compile (uncomment to see error):");
    println!("   /*");
    println!("   let mut s = String::from(\"test\");");
    println!("   let r1 = &s;          // immutable borrow");
    println!("   let r2 = &mut s;      // ❌ ERROR: can't borrow as mutable");
    println!("   println!(\"{{}}\", r1); //          while immutable borrow exists");
    println!("   */");

    // ============================================================================
    // CLOSING MESSAGE
    // ============================================================================

    println!("\n=== Demo Complete! ===");
    println!("\nKey Takeaways:");
    println!("  1. Ownership: Each value has exactly one owner");
    println!("  2. Moving: Non-Copy types transfer ownership on assignment");
    println!("  3. Borrowing (&T): Multiple readers allowed, no modification");
    println!("  4. Mutable Borrowing (&mut T): One writer at a time, exclusive access");
    println!("  5. Copy vs Move: Small types copy, heap types move");
    println!("\nNow try:");
    println!("  1. Look at src/solution.rs for detailed explanations");
    println!("  2. Implement your own version in src/lib.rs");
    println!("  3. Run 'cargo test -p ownership-borrowing' to check your work");
}
