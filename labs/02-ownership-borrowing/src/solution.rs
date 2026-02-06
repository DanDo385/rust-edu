//! # Ownership and Borrowing - Complete Solution with EXHAUSTIVE Explanations
//!
//! ## What We're Building
//!
//! This module contains four functions that demonstrate Rust's ownership system—
//! the concept that makes Rust unique among programming languages. These functions
//! show you how Rust achieves memory safety without a garbage collector.
//!
//! ## Why Rust Is Perfect For This
//!
//! - **Memory safety without GC**: Ownership rules prevent use-after-free, double-free,
//!   and memory leaks at compile time. No runtime garbage collector needed!
//!
//! - **Zero-cost abstraction**: All ownership checks happen at compile time. The
//!   generated machine code is as fast as hand-written C, but memory-safe.
//!
//! - **Fearless concurrency**: The same ownership rules that prevent memory bugs
//!   also prevent data races in multithreaded code. Thread safety guaranteed!
//!
//! - **Compared to C/C++**: No manual memory management, no dangling pointers,
//!   no buffer overflows, no undefined behavior
//! - **Compared to Java/Go**: No garbage collector pauses, deterministic cleanup
//! - **Compared to Python/JavaScript**: Compile-time guarantees, no runtime surprises
//!
//! ## Key Rust Concepts You'll Learn
//!
//! - **Ownership**: Every value has exactly one owner at a time
//! - **Move semantics**: Non-Copy types transfer ownership on assignment
//! - **Borrowing**: Temporary access to data without ownership transfer
//! - **Immutable borrows (&T)**: Multiple readers allowed, no mutation
//! - **Mutable borrows (&mut T)**: Exclusive access, can mutate
//! - **Copy trait**: Types that can be duplicated automatically
//! - **Drop trait**: Automatic cleanup when owner goes out of scope

/// Takes ownership of a String, adds an exclamation mark, and returns it.
///
/// ## What This Function Does
///
/// This function demonstrates TAKING OWNERSHIP of data. The caller gives up
/// ownership of their String, we modify it, then we give ownership back.
/// This is different from most languages where parameters are copied or referenced.
///
/// ## Rust Concepts Demonstrated
///
/// - **Taking ownership**: `s: String` (not `&String`) means we take ownership
/// - **Ownership transfer**: Caller can't use their variable after calling this
/// - **Modifying owned data**: Since we own it, we can modify it freely
/// - **Returning ownership**: We give the (modified) String back to caller
/// - **Move semantics**: No copying involved, just ownership transfer
///
/// ## Parameters
///
/// - `s: String` - Let's break this down completely:
///   - `s` = parameter name
///   - `:` = "has type"
///   - `String` = owned string (NO &, so we take ownership)
///
///   What does taking ownership mean?
///   - The caller's variable becomes INVALID after calling this function
///   - We are now the owner of that String's heap data
///   - We're responsible for it until we return it or drop it
///   - This is a MOVE, not a copy—no data duplication happens
///
/// ## Returns
///
/// The same String (modified) with ownership transferred back to caller
///
/// ## Example
/// ```ignore
/// let my_string = String::from("Hello");
/// let result = add_exclamation(my_string);
/// // my_string is NO LONGER VALID here! It was moved.
/// // println!("{}", my_string);  // ❌ Won't compile: value used after move
/// println!("{}", result);  // ✅ "Hello!"
/// ```ignore
///
/// ## Memory Layout
/// ```ignore
/// Before call:
/// Caller's stack:              Heap:
/// ┌──────────────────┐
/// │ my_string:String │────▶  ┌─────────┐
/// │  - ptr: 0x1000   │       │ "Hello" │
/// │  - len: 5        │       │ (5 bytes)│
/// │  - cap: 5        │       └─────────┘
/// └──────────────────┘
///
/// During call (ownership moved):
/// Caller's stack:              Our stack:              Heap:
/// ┌──────────────────┐        ┌──────────────┐
/// │ my_string: ❌    │        │ s: String    │────▶  ┌─────────┐
/// │ (invalid!)       │        │ - ptr: 0x1000│       │ "Hello" │
/// └──────────────────┘        │ - len: 5     │       │         │
///                             │ - cap: 5     │       └─────────┘
///                             └──────────────┘
///
/// After adding '!':
/// Our stack:              Heap:
/// ┌──────────────┐
/// │ s: String    │────▶  ┌──────────┐
/// │ - ptr: 0x1000│       │ "Hello!" │
/// │ - len: 6     │       │ (6 bytes)│
/// │ - cap: 10    │       └──────────┘
/// └──────────────┘        (may reallocate if needed)
///
/// After return (ownership moved back):
/// Caller's stack:              Heap:
/// ┌──────────────────┐
/// │ result: String   │────▶  ┌──────────┐
/// │  - ptr: 0x1000   │       │ "Hello!" │
/// │  - len: 6        │       │ (6 bytes)│
/// │  - cap: 10       │       └──────────┘
/// └──────────────────┘
/// ```ignore
pub fn add_exclamation(mut s: String) -> String {
    // ========================================================================
    // WHY `mut s` INSTEAD OF JUST `s`?
    // ========================================================================

    // When we take ownership of `s`, we can do anything with it, including
    // modifying it. But to modify, we need to mark it as mutable with `mut`.
    //
    // `mut` = mutable keyword
    //   - Allows us to modify the variable
    //   - Without it, `s` would be immutable (can't call .push() on it)
    //   - This is separate from ownership—we own it either way
    //
    // Alternative: We could take `s: String` (without mut) if we only read it,
    // but since we want to modify, we need `mut s: String`

    // ========================================================================
    // STEP 1: ADD THE EXCLAMATION MARK
    // ========================================================================

    // `s.push('!')` = append a single character to the String
    //   - `s` = our owned String
    //   - `.` = access a method on s
    //   - `push` = method that adds a char to the end
    //   - `('!')` = the character to add (char literal in single quotes)
    //
    // What happens in memory?
    // - String checks if there's capacity for one more char
    // - If yes: writes '!' at the end, increments length
    // - If no: allocates new larger buffer, copies old data, adds '!'
    // - Rust handles all of this automatically—no buffer overflows possible!
    //
    // Why can we call .push()?
    // - We OWN the String (took ownership from caller)
    // - Methods that modify data require ownership or mutable borrow
    // - Since we own it, we can do anything with it
    //
    // Cost: O(1) amortized time (usually just increments length)
    //       O(1) or O(n) space if reallocation needed

    s.push('!');

    // Alternative approaches:
    //
    // 1. Using push_str (for a string slice instead of char):
    //    s.push_str("!");
    //
    // 2. Using format! (creates new String, less efficient):
    //    return format!("{}!", s);
    //
    // 3. Using concatenation (also creates new String):
    //    return s + "!";
    //
    // The .push() approach is most efficient because it modifies in place!

    // ========================================================================
    // STEP 2: RETURN THE MODIFIED STRING
    // ========================================================================

    // `s` = just the variable name, no semicolon
    //   - This is an expression (evaluates to the String)
    //   - Its value becomes the return value
    //   - Ownership of s transfers to the caller
    //
    // What happens to ownership?
    // - We owned s (took it from caller)
    // - We modified it
    // - Now we're giving ownership back to caller
    // - Caller gets the modified String
    // - When our function ends, our stack frame is destroyed
    // - But the heap data (the actual "Hello!" bytes) is NOT freed
    // - The caller now owns that heap data
    //
    // This is called MOVE SEMANTICS:
    // - Ownership moved FROM caller TO us (parameter)
    // - Ownership moved FROM us TO caller (return value)
    // - No copying of the string data itself!
    // - Just transferring responsibility

    s

    // ============================================================================
    // OWNERSHIP FLOW DIAGRAM
    // ============================================================================
    //
    // 1. Caller creates String (caller owns it):
    //    let my_string = String::from("Hello");
    //    Owner: caller
    //
    // 2. Caller calls function:
    //    let result = add_exclamation(my_string);
    //    Ownership MOVES from caller to function parameter `s`
    //    Owner: function (parameter s)
    //    Caller's `my_string` is now INVALID
    //
    // 3. Function modifies:
    //    s.push('!');
    //    Owner: still function (parameter s)
    //
    // 4. Function returns:
    //    return s;
    //    Ownership MOVES from function to caller's `result`
    //    Owner: caller (result variable)
    //
    // 5. Function ends:
    //    Function's stack frame is destroyed
    //    But heap data is NOT freed (ownership moved out)
    //
    // 6. Later, when `result` goes out of scope:
    //    String's Drop trait runs
    //    Heap memory is finally freed
    //
    // TOTAL ALLOCATIONS: 1 (the original String::from)
    // TOTAL FREES: 1 (when result is dropped)
    // TOTAL COPIES: 0 (just ownership transfers!)
    //
    // This is why Rust is fast and safe!

    // ============================================================================
    // WHAT IF WE DIDN'T RETURN IT?
    // ============================================================================
    //
    // If we wrote:
    // ```rust
    // pub fn add_exclamation(mut s: String) {
    //     s.push('!');
    //     // No return statement
    // }
    // ```
    //
    // Then:
    // 1. We take ownership of the String
    // 2. We modify it
    // 3. Function ends
    // 4. s goes out of scope
    // 5. s's Drop trait runs
    // 6. The heap memory is FREED
    // 7. Caller has lost their String forever!
    //
    // This is actually valid Rust! It's called "consuming" a value.
    // The function takes ownership and drops it.
    // Use case: cleanup functions, or when you're done with data
    //
    // But for our case, we want to give it back, so we return it!

    // ============================================================================
    // RUST VS OTHER LANGUAGES
    // ============================================================================
    //
    // C++ (manual memory management):
    // ```cpp
    // std::string add_exclamation(std::string s) {
    //     s += "!";
    //     return s;  // Copy! (unless move semantics explicitly invoked)
    // }
    // ```
    // Issues:
    // - Might copy the string (expensive!)
    // - Move semantics exist but require std::move
    // - Easy to create dangling pointers in complex code
    // - No compile-time guarantees about memory safety
    //
    // Python (garbage collected):
    // ```python
    // def add_exclamation(s: str) -> str:
    //     return s + "!"  // Creates new string (strings immutable in Python)
    // }
    // ```
    // Issues:
    // - Strings are immutable, must create new one
    // - Garbage collector overhead
    // - No compile-time type checking (type hints not enforced)
    //
    // JavaScript (garbage collected):
    // ```javascript
    // function addExclamation(s) {
    //     return s + "!";  // Creates new string
    // }
    // ```
    // Issues:
    // - No type safety
    // - Garbage collector
    // - Could pass non-string, would coerce or error at runtime
    //
    // Go (garbage collected):
    // ```go
    // func addExclamation(s string) string {
    //     return s + "!"  // Creates new string (strings immutable in Go)
    // }
    // ```
    // Issues:
    // - Strings immutable, must allocate new one
    // - Garbage collector
    // - Less efficient than Rust's in-place modification
    //
    // Rust advantages:
    // - Modifies in place (efficient!)
    // - No garbage collector (fast, predictable)
    // - Memory safety guaranteed at compile time
    // - Ownership makes it clear who's responsible for cleanup
    // - Can't accidentally keep references to freed memory
}

/// Borrows a String immutably and returns its length.
///
/// ## What This Function Does
///
/// This function demonstrates BORROWING data. We get temporary read access
/// to a String without taking ownership. The caller keeps their String and
/// can continue using it after we return.
///
/// ## Rust Concepts Demonstrated
///
/// - **Immutable borrowing**: `&String` means we borrow but don't own
/// - **Read-only access**: Can't modify the String, only read it
/// - **Multiple borrows**: Can have many immutable borrows simultaneously
/// - **Original data stays valid**: Caller's String is still valid after call
/// - **No ownership transfer**: We never own the data
///
/// ## Parameters
///
/// - `s: &String` - Let's break this down:
///   - `s` = parameter name
///   - `:` = "has type"
///   - `&` = borrow operator (we're borrowing, not owning)
///   - `String` = we're borrowing a String
///
///   What does borrowing mean?
///   - We get a reference (pointer) to the String
///   - We can READ the data
///   - We CANNOT modify it (immutable borrow)
///   - We CANNOT take ownership
///   - The original owner keeps ownership
///   - When we're done, the borrow ends, owner can continue using it
///
/// ## Returns
///
/// The length of the string as usize (we computed it, we own this number)
///
/// ## Example
/// ```ignore
/// let my_string = String::from("Hello");
/// let len = get_length(&my_string);  // &my_string = borrow
/// println!("{}", len);         // 5
/// println!("{}", my_string);   // ✅ Still valid! We only borrowed it.
/// ```ignore
///
/// ## Memory Layout
/// ```ignore
/// Caller's stack:              Our stack:              Heap:
/// ┌──────────────────┐        ┌──────────────┐
/// │ my_string:String │────▶   │              │        ┌─────────┐
/// │  - ptr: 0x1000   │   │    │ s: &String   │───▶───┤         │
/// │  - len: 5        │   │    │ (0x2000)     │        │ "Hello" │
/// │  - cap: 5        │   │    └──────────────┘        │ (5 bytes)│
/// └──────────────────┘   │                            └─────────┘
///       ▲                │
///       │                │
///       └────────────────┘
///      (s points to my_string, which points to heap)
///
/// Key points:
/// - my_string OWNS the heap data (at 0x1000)
/// - s is a REFERENCE to my_string (points to 0x2000 on caller's stack)
/// - s does NOT own anything
/// - When our function returns, s disappears (just a pointer)
/// - my_string is still valid and still owns the heap data
/// - No data was copied, no ownership transferred!
/// ```ignore
pub fn get_length(s: &String) -> usize {
    // ========================================================================
    // UNDERSTANDING BORROWING
    // ========================================================================

    // `s` is a reference to a String. Let's understand what that means:
    //
    // - `s` is NOT the String itself
    // - `s` is a POINTER to the String (stored on caller's stack)
    // - On a 64-bit system, `s` is just 8 bytes (a memory address)
    // - We can READ through the pointer to access the String's data
    // - We CANNOT modify the String (immutable borrow)
    // - We CANNOT transfer ownership (we don't have it!)
    //
    // Why does Rust do this?
    // - Efficiency: Passing a pointer is cheap (8 bytes vs potentially large String)
    // - Safety: Caller keeps ownership, we can't accidentally free their data
    // - Concurrency: Multiple immutable borrows allowed (thread-safe reading)

    // ========================================================================
    // STEP 1: GET THE LENGTH
    // ========================================================================

    // `s.len()` = call the len() method on the String
    //   - `s` = our borrowed reference (&String)
    //   - `.` = access a method (Rust automatically dereferences for methods)
    //   - `len()` = method that returns the length as usize
    //
    // How does .len() work?
    // - String stores its length in a field (no need to count characters)
    // - Reading the length is O(1) - instant!
    // - Returns usize (unsigned integer, size of a pointer)
    // - This is a cheap operation, just reading a number
    //
    // Why can we call methods on a reference?
    // - Rust automatically dereferences for method calls
    // - s.len() is syntactic sugar for (*s).len()
    // - But you don't need to write the * - Rust does it for you!
    //
    // Can we modify the String?
    // - NO! `s` is an immutable borrow (&String, not &mut String)
    // - Methods like .push() wouldn't compile
    // - Compiler error: "cannot borrow as mutable"

    s.len()

    // ============================================================================
    // ALTERNATIVE SYNTAX
    // ============================================================================
    //
    // These are all equivalent:
    //
    // 1. What we wrote (most idiomatic):
    //    s.len()
    //
    // 2. Explicit dereference:
    //    (*s).len()
    //
    // 3. Using String's function syntax:
    //    String::len(s)
    //
    // 4. With temporary variable:
    //    let length = s.len();
    //    length
    //
    // The first form (s.len()) is standard Rust style!

    // ============================================================================
    // BORROWING RULES - THE HEART OF RUST
    // ============================================================================
    //
    // Rust's borrowing rules (enforced at compile time):
    //
    // 1. You can have EITHER:
    //    - Any number of immutable borrows (&T), OR
    //    - Exactly ONE mutable borrow (&mut T)
    //    BUT NOT BOTH at the same time!
    //
    // 2. Borrows must not outlive the data they borrow
    //
    // Why these rules?
    // - Prevents data races (multiple writers or read-during-write)
    // - Prevents iterator invalidation (modifying collection while iterating)
    // - Prevents use-after-free (borrow must not outlive owner)
    // - All checked at compile time—no runtime cost!
    //
    // Example of the rules:
    // ```rust
    // let mut s = String::from("hello");
    //
    // let r1 = &s;      // ✅ immutable borrow
    // let r2 = &s;      // ✅ another immutable borrow (multiple readers OK!)
    // println!("{} {}", r1, r2);  // Using the borrows
    // // r1 and r2 go out of scope here (last use)
    //
    // let r3 = &mut s;  // ✅ mutable borrow (previous borrows ended)
    // r3.push_str("!");
    // println!("{}", r3);
    // ```
    //
    // This won't compile:
    // ```rust
    // let mut s = String::from("hello");
    // let r1 = &s;      // immutable borrow
    // let r2 = &mut s;  // ❌ ERROR: can't have &mut while & exists
    // println!("{}", r1);
    // ```

    // ============================================================================
    // LIFETIME ANALYSIS (Simplified)
    // ============================================================================
    //
    // Every reference has a lifetime—the scope for which it's valid.
    //
    // In our function:
    // ```rust
    // pub fn get_length(s: &String) -> usize {
    //     s.len()
    // }
    // ```
    //
    // Full signature with explicit lifetime:
    // ```rust
    // pub fn get_length<'a>(s: &'a String) -> usize {
    //     s.len()
    // }
    // ```
    //
    // What this means:
    // - `'a` is a lifetime parameter (like a generic type)
    // - `&'a String` means "a reference valid for lifetime 'a"
    // - Return type is usize (no reference, we own the number)
    // - Since we don't return a reference, lifetime is simple
    // - Rust can infer this, so we don't need to write it
    //
    // The borrow checker ensures:
    // - `s` is valid for entire function call
    // - We can't return a reference to local data
    // - We can't outlive the data we're borrowing

    // ============================================================================
    // WHAT IF PARAMETER WAS `s: String` INSTEAD OF `s: &String`?
    // ============================================================================
    //
    // If we took ownership:
    // ```rust
    // pub fn get_length(s: String) -> usize {
    //     s.len()
    // }  // s is dropped here—heap memory freed!
    // ```
    //
    // Then:
    // 1. Caller would lose their String (ownership moved to us)
    // 2. We'd get the length
    // 3. Function ends
    // 4. String is dropped, heap memory freed
    // 5. Caller can no longer use their String!
    //
    // This would work but would be wasteful:
    // - Caller loses their String just so we can read its length
    // - They'd have to clone before calling if they wanted to keep it
    // - Cloning is expensive (allocates + copies heap data)
    //
    // Borrowing is the right choice here:
    // - We only need to READ, not OWN
    // - Caller keeps their String
    // - No allocations, no copies, just passing a pointer
    // - This is Rust's zero-cost abstraction philosophy!

    // ============================================================================
    // RUST VS OTHER LANGUAGES
    // ============================================================================
    //
    // C++ (reference):
    // ```cpp
    // size_t get_length(const std::string& s) {
    //     return s.length();
    // }
    // ```
    // Similar to Rust! But:
    // - No compile-time guarantee reference is valid
    // - Easy to return dangling references in other contexts
    // - No protection against data races in multithreading
    //
    // Python (everything is references):
    // ```python
    // def get_length(s: str) -> int:
    //     return len(s)
    // ```
    // Issues:
    // - Everything is a reference (can't opt into ownership)
    // - No compile-time checking
    // - Garbage collector overhead
    //
    // Go (slices are references):
    // ```go
    // func getLength(s string) int {
    //     return len(s)
    // }
    // ```
    // Issues:
    // - Garbage collector
    // - No distinction between owned and borrowed (everything borrowed)
    //
    // Rust advantages:
    // - Explicit about ownership vs borrowing
    // - Compile-time guarantees about validity
    // - No garbage collector needed
    // - Zero runtime cost
    // - Thread-safe by default (immutable borrows are safe to share)
}

/// Borrows a String mutably and converts it to uppercase in place.
///
/// ## What This Function Does
///
/// This function demonstrates MUTABLE BORROWING. We get temporary write access
/// to a String without taking ownership. We can modify it, and the caller
/// sees those modifications.
///
/// ## Rust Concepts Demonstrated
///
/// - **Mutable borrowing**: `&mut String` means exclusive write access
/// - **In-place modification**: We modify the original data, no copy made
/// - **Exclusive access**: Only ONE mutable borrow allowed at a time
/// - **No concurrent borrows**: Can't have any other borrows (mut or immut) while &mut exists
/// - **Original data modified**: Caller sees the changes we made
///
/// ## Parameters
///
/// - `s: &mut String` - Let's break this down:
///   - `s` = parameter name
///   - `:` = "has type"
///   - `&mut` = mutable borrow operator (exclusive write access)
///   - `String` = we're borrowing a String mutably
///
///   What does mutable borrowing mean?
///   - We get a reference (pointer) to the String
///   - We can READ the data
///   - We can MODIFY the data
///   - We CANNOT take ownership
///   - We have EXCLUSIVE access (no other borrows allowed simultaneously)
///   - Changes we make are visible to the owner
///
/// ## Returns
///
/// Nothing (unit type `()`) - we modify in place
///
/// ## Example
/// ```ignore
/// let mut my_string = String::from("hello");
/// make_uppercase(&mut my_string);  // &mut = mutable borrow
/// println!("{}", my_string);       // ✅ "HELLO" - modified!
/// ```ignore
///
/// ## Memory Layout
/// ```ignore
/// Before call:
/// Caller's stack:              Heap:
/// ┌──────────────────┐
/// │ my_string:String │────▶  ┌─────────┐
/// │  - ptr: 0x1000   │       │ "hello" │
/// │  - len: 5        │       │ (5 bytes)│
/// │  - cap: 5        │       └─────────┘
/// └──────────────────┘
///
/// During call:
/// Caller's stack:              Our stack:              Heap:
/// ┌──────────────────┐        ┌──────────────┐
/// │ my_string:String │◀───────│ s: &mut      │        ┌─────────┐
/// │  - ptr: 0x1000   │        │    String    │───────▶│ "hello" │
/// │  - len: 5        │        │ (0x2000)     │        │         │
/// │  - cap: 5        │        └──────────────┘        └─────────┘
/// └──────────────────┘
///   (CANNOT be used while we have &mut!)
///
/// After modification:
/// Caller's stack:              Our stack:              Heap:
/// ┌──────────────────┐        ┌──────────────┐
/// │ my_string:String │◀───────│ s: &mut      │        ┌─────────┐
/// │  - ptr: 0x1000   │        │    String    │───────▶│ "HELLO" │
/// │  - len: 5        │        │              │        │ (5 bytes)│
/// │  - cap: 5        │        └──────────────┘        └─────────┘
/// └──────────────────┘
///
/// After return:
/// Caller's stack:              Heap:
/// ┌──────────────────┐
/// │ my_string:String │────▶  ┌─────────┐
/// │  - ptr: 0x1000   │       │ "HELLO" │
/// │  - len: 5        │       │ (modified!)
/// │  - cap: 5        │       └─────────┘
/// └──────────────────┘
///   (can use again—borrow ended)
/// ```ignore
pub fn make_uppercase(s: &mut String) {
    // ========================================================================
    // UNDERSTANDING MUTABLE BORROWING
    // ========================================================================

    // `s` is a mutable reference to a String. What does that mean?
    //
    // - `s` is a pointer to a String (on caller's stack)
    // - We can READ the String's data
    // - We can MODIFY the String's data
    // - We have EXCLUSIVE access (no other borrows allowed)
    // - We DON'T own the String (can't drop or move it)
    //
    // Why &mut instead of taking ownership?
    // - Caller wants to keep their String
    // - We just need to modify it temporarily
    // - No need to move ownership back and forth
    // - More efficient (no moves, just a pointer)
    //
    // Why only ONE mutable borrow at a time?
    // - Prevents data races (two threads writing simultaneously)
    // - Prevents iterator invalidation (modifying while iterating)
    // - Compiler enforces this at compile time!

    // ========================================================================
    // STEP 1: GET THE UPPERCASE VERSION
    // ========================================================================

    // `s.to_uppercase()` = create an uppercase version of the String
    //   - `s` = our mutable reference
    //   - `.to_uppercase()` = method that returns a NEW String (uppercase)
    //   - This doesn't modify s, it creates a new String
    //   - Returns a String (owned, on heap)
    //
    // Why to_uppercase() instead of a method that modifies in place?
    // - Rust's String doesn't have a built-in uppercase_in_place() method
    // - to_uppercase() handles Unicode properly (e.g., German ß → SS)
    // - Some characters expand when uppercased!
    // - So we need to create a new String with the result
    //
    // What about Unicode?
    // - to_uppercase() properly handles multi-byte UTF-8 characters
    // - Example: "straße" → "STRASSE" (ß becomes SS, length changes!)
    // - This is why we need to create a new String

    let uppercase = s.to_uppercase();

    // ========================================================================
    // STEP 2: REPLACE THE ORIGINAL STRING'S CONTENTS
    // ========================================================================

    // Now we need to replace s's contents with uppercase
    // We have a few options:

    // OPTION 1: Dereference and assign (what we'll use)
    // `*s = uppercase;`
    //   - `*s` = dereference the mutable reference (get to the actual String)
    //   - `=` = assignment operator
    //   - `uppercase` = the new String we created
    //
    // What happens here?
    // 1. The old String in s is dropped (heap memory freed)
    // 2. The new String (uppercase) is moved into s's location
    // 3. Ownership of uppercase transfers to the caller's variable
    //
    // This is safe because:
    // - We have exclusive access (&mut)
    // - No one else can be using the old data
    // - The caller's variable is updated in place

    *s = uppercase;

    // ALTERNATIVE OPTION 2: Clear and push
    // ```rust
    // let uppercase = s.to_uppercase();
    // s.clear();
    // s.push_str(&uppercase);
    // ```
    // This would:
    // 1. Create uppercase String
    // 2. Clear s (length becomes 0, capacity stays)
    // 3. Push uppercase's characters into s
    //
    // Less efficient: creates String, then copies characters again!

    // ALTERNATIVE OPTION 3: Using mem::replace
    // ```rust
    // use std::mem;
    // let old = mem::replace(s, s.to_uppercase());
    // // old contains the old value, s contains uppercase
    // drop(old);  // Explicitly drop the old value
    // ```
    //
    // More explicit but same result!

    // ALTERNATIVE OPTION 4: Manual iteration (most complex)
    // ```rust
    // *s = s.chars().map(|c| c.to_uppercase().next().unwrap()).collect();
    // ```
    // This shows how to uppercase character by character
    // More complex and not necessarily more efficient!

    // ========================================================================
    // NO RETURN VALUE
    // ========================================================================

    // Notice: This function doesn't return anything!
    // - Function signature: `fn make_uppercase(s: &mut String)`
    // - No `-> Type` means it returns `()` (unit type, like void)
    //
    // Why no return value?
    // - We modified the String IN PLACE
    // - The caller's variable is updated
    // - No need to return anything
    // - This is idiomatic Rust for mutation functions
    //
    // The caller sees the changes:
    // ```rust
    // let mut text = String::from("hello");
    // make_uppercase(&mut text);
    // // text is now "HELLO"!
    // ```

    // ============================================================================
    // THE DERFERENCE OPERATOR (`*`)
    // ============================================================================
    //
    // In `*s = uppercase;`, the `*` is the dereference operator.
    //
    // What does it do?
    // - `s` is a reference (&mut String) - a pointer
    // - `*s` follows the pointer to get the actual String
    // - Then we can assign to it
    //
    // When do you need `*`?
    // - When assigning to a reference: `*s = value;`
    // - When reading through a reference to a Copy type: `let x = *int_ref;`
    //
    // When do you NOT need `*`?
    // - Method calls: `s.push('!')` - Rust auto-dereferences
    // - Field access: `s.len` - Rust auto-dereferences
    // - Reading non-Copy types through reference - use the reference directly
    //
    // Why does Rust auto-dereference?
    // - Convenience! Writing `(*s).method()` every time would be tedious
    // - Rust knows when it needs to dereference for methods/fields
    // - But for assignment, you need to be explicit with `*`

    // ============================================================================
    // EXCLUSIVITY OF MUTABLE BORROWS
    // ============================================================================
    //
    // While we have `&mut s`, the caller CANNOT:
    // - Read the String: `println!("{}", my_string);` would error
    // - Modify the String: can't call methods on it
    // - Create another borrow: `let r = &my_string;` would error
    // - Create another mutable borrow: `let r = &mut my_string;` would error
    //
    // This prevents:
    // - Data races (two threads accessing simultaneously)
    // - Reading inconsistent state (reading while being modified)
    // - Iterator invalidation (modifying while iterating)
    //
    // When we return, the borrow ends:
    // - Caller can use their String again
    // - They see the modifications we made
    // - They can create new borrows
    //
    // Example that won't compile:
    // ```rust
    // let mut s = String::from("hello");
    // let r1 = &mut s;           // mutable borrow
    // let r2 = &s;               // ❌ ERROR: can't borrow as immutable
    // r1.push('!');              //          because already borrowed as mutable
    // ```
    //
    // This WILL compile (borrows don't overlap):
    // ```rust
    // let mut s = String::from("hello");
    // let r1 = &mut s;
    // r1.push('!');
    // // r1 last used here, borrow ends
    // let r2 = &s;               // ✅ OK: previous borrow ended
    // println!("{}", r2);
    // ```

    // ============================================================================
    // RUST VS OTHER LANGUAGES
    // ============================================================================
    //
    // C++ (reference):
    // ```cpp
    // void make_uppercase(std::string& s) {
    //     std::transform(s.begin(), s.end(), s.begin(), ::toupper);
    // }
    // ```
    // Similar concept, but:
    // - No compile-time guarantee of exclusive access
    // - Easy to have data races in multithreaded code
    // - toupper() doesn't handle Unicode properly
    //
    // Python (strings are immutable!):
    // ```python
    // def make_uppercase(s: list) -> None:
    //     # Can't modify string in place! Strings are immutable.
    //     # If s were a list: s[:] = [x.upper() for x in s]
    //     pass
    // ```
    // Issues:
    // - Strings are immutable in Python
    // - Must create new string and return it
    // - No way to modify caller's string in place
    // - If using mutable list, no compile-time safety
    //
    // JavaScript (strings are immutable):
    // ```javascript
    // function makeUppercase(s) {
    //     // Can't modify! Strings are immutable.
    //     // Would need to return new string
    //     return s.toUpperCase();
    // }
    // ```
    // Issues:
    // - Strings are immutable
    // - Must create new string
    // - If using objects/arrays, no safety guarantees
    //
    // Go (strings are immutable):
    // ```go
    // func makeUppercase(s *string) {
    //     *s = strings.ToUpper(*s)
    // }
    // ```
    // Similar to Rust! But:
    // - Strings are immutable, must allocate new one
    // - No compile-time guarantee of exclusive access
    // - Could have data races in concurrent code
    //
    // Rust advantages:
    // - Can modify String in place (they're mutable if mut)
    // - Compile-time guarantee of exclusive access
    // - Impossible to have data races (no other borrows while &mut exists)
    // - No garbage collector needed
    // - Zero runtime cost for safety checks
    // - Proper Unicode handling
}

/// Demonstrates the difference between Copy types and Move types.
///
/// ## What This Function Does
///
/// This function creates a clear demonstration of Rust's Copy and Move semantics.
/// It shows how primitive types (i32) are copied automatically, while heap-allocated
/// types (String) are moved by default to prevent expensive hidden copies.
///
/// ## Rust Concepts Demonstrated
///
/// - **Copy trait**: Types that can be duplicated by copying bits
/// - **Move semantics**: Types that transfer ownership instead of copying
/// - **clone() method**: Explicit deep copy for types that don't implement Copy
/// - **Ownership transfer**: What happens to variables after moves
/// - **Stack vs Heap**: Where different types are stored
///
/// ## Returns
///
/// A String explaining Copy vs Move with concrete examples
///
/// ## Example
/// ```ignore
/// let explanation = demonstrate_copy_vs_move();
/// println!("{}", explanation);
/// // Output shows i32 being copied and String being moved
/// ```ignore
pub fn demonstrate_copy_vs_move() -> String {
    // ========================================================================
    // PART 1: DEMONSTRATE COPY BEHAVIOR (i32)
    // ========================================================================

    // Let's show how Copy types work
    //
    // `let x = 5;` = create an i32 on the stack
    //   - i32 is a primitive type
    //   - Stored entirely on the stack (4 bytes)
    //   - Implements the Copy trait
    //   - "Copy" means it can be duplicated by copying bits
    let x = 5;

    // `let y = x;` = copy x to y
    //   - Since i32 implements Copy, this COPIES the value
    //   - x is still valid after this line!
    //   - y has its own copy of the value 5
    //   - Both x and y are independent
    //   - Copying 4 bytes is cheap (single CPU instruction)
    let y = x;

    // Both x and y are valid here!
    // We can use both because i32 was COPIED, not moved

    // ========================================================================
    // PART 2: DEMONSTRATE MOVE BEHAVIOR (String)
    // ========================================================================

    // Let's show how Move types work
    //
    // `let s1 = String::from("hello");`
    //   - Creates a String on the heap
    //   - String does NOT implement Copy
    //   - String is heap-allocated (expensive to copy)
    //   - Contains: pointer + length + capacity (24 bytes on stack)
    //   - Points to: actual character data on heap
    let s1 = String::from("hello");

    // `let s2 = s1;` = move s1 to s2
    //   - Since String doesn't implement Copy, this MOVES
    //   - Ownership of the heap data transfers from s1 to s2
    //   - s1 is no longer valid after this line!
    //   - s2 now owns the heap data
    //   - No data was copied—just ownership transfer
    //   - Prevents expensive hidden copies of heap data
    let s2 = s1;

    // s1 is INVALID here! Trying to use it would be a compile error:
    // println!("{}", s1);  // ❌ ERROR: value borrowed here after move

    // s2 is valid and owns the String
    // This prevents bugs and makes performance predictable!

    // ========================================================================
    // PART 3: DEMONSTRATE EXPLICIT CLONING
    // ========================================================================

    // What if you DO want to copy a String?
    // Use .clone() to make it explicit!
    //
    // `let s3 = String::from("world");`
    //   - Create another String
    let s3 = String::from("world");

    // `let s4 = s3.clone();`
    //   - `.clone()` makes an explicit deep copy
    //   - Allocates new heap memory
    //   - Copies all the character data
    //   - s3 and s4 are now independent
    //   - Both are valid and own their own heap data
    //   - This is expensive but explicit—you can see it in code!
    let s4 = s3.clone();

    // Both s3 and s4 are valid here!
    // They each own their own heap data
    // No surprise memory allocations—you explicitly called .clone()

    // ========================================================================
    // PART 4: BUILD THE EXPLANATION
    // ========================================================================

    // Now we'll build a String that explains what happened
    // Using format! to create a detailed explanation

    format!(
        // The format string (similar to printf in C)
        // Each `{{}}` is a literal brace
        // Each `{}` is a placeholder for a value
        "\
Copy vs Move Demonstration:

1. COPY TYPES (i32):
   - let x = 5;
   - let y = x;
   - Both x and y are valid! x = {}, y = {}
   - i32 is Copy: assignment duplicates the value
   - Cheap: only 4 bytes copied
   - No heap allocation

2. MOVE TYPES (String):
   - let s1 = String::from(\"hello\");
   - let s2 = s1;
   - s1 is now INVALID! Only s2 is valid: \"{}\"
   - String is Move: ownership transfers, no copy
   - Prevents expensive hidden copies
   - Prevents double-free bugs

3. EXPLICIT CLONING (String):
   - let s3 = String::from(\"world\");
   - let s4 = s3.clone();
   - Both s3 and s4 are valid! s3 = \"{}\", s4 = \"{}\"
   - .clone() makes explicit deep copy
   - Allocates new heap memory
   - You can see the cost in code

WHY THIS MATTERS:
- Copy: For small, stack-only types (i32, f64, bool, char, tuples of Copy types)
- Move: For types with heap data (String, Vec, Box, most structs)
- Explicit cloning: When you need a copy, you can see it (not hidden)
- Performance: No surprise expensive operations
- Safety: No double-free or use-after-free bugs
",
        x,  // First {} - value of x
        y,  // Second {} - value of y
        s2, // Third {} - value of s2
        s3, // Fourth {} - value of s3
        s4, // Fifth {} - value of s4
    )

    // ============================================================================
    // COPY TRAIT - TECHNICAL DETAILS
    // ============================================================================
    //
    // What types implement Copy?
    // - All primitive integer types: i8, i16, i32, i64, i128, isize, u8, u16, u32, u64, u128, usize
    // - Floating point types: f32, f64
    // - Boolean: bool
    // - Character: char
    // - Tuples of Copy types: (i32, i32), (bool, f64), etc.
    // - Arrays of Copy types: [i32; 10], [bool; 5], etc.
    //
    // Copy trait requirements:
    // - Type must not implement Drop
    // - All fields must implement Copy
    // - Type must be fixed-size on stack
    // - Copying must be cheap (just memcpy)
    //
    // Why doesn't String implement Copy?
    // - String owns heap data
    // - Copying would require allocating new heap memory
    // - Expensive operation (proportional to string length)
    // - Rust wants expensive operations to be explicit (.clone())
    //
    // Custom types and Copy:
    // ```rust
    // #[derive(Copy, Clone)]  // Can derive Copy if all fields are Copy
    // struct Point {
    //     x: i32,
    //     y: i32,
    // }
    // ```

    // ============================================================================
    // MOVE SEMANTICS - TECHNICAL DETAILS
    // ============================================================================
    //
    // What happens during a move?
    // 1. Ownership transfers from source to destination
    // 2. Source variable becomes invalid (compiler tracks this)
    // 3. NO data copying happens (just pointer/metadata copy)
    // 4. When destination goes out of scope, data is dropped
    // 5. Source won't try to drop (it doesn't own anymore)
    //
    // Move is the DEFAULT for non-Copy types:
    // ```rust
    // let s1 = String::from("hello");
    // let s2 = s1;  // Move (default)
    // let s3 = s1;  // ❌ ERROR: s1 was moved
    // ```
    //
    // Prevents double-free bug:
    // If String implemented Copy, this could happen:
    // 1. s1 owns heap data at 0x1000
    // 2. s2 copies s1, now also points to 0x1000
    // 3. s1 goes out of scope, frees 0x1000
    // 4. s2 goes out of scope, tries to free 0x1000 again
    // 5. DOUBLE FREE! Undefined behavior!
    //
    // Rust's solution:
    // - String doesn't implement Copy
    // - Assignment moves ownership
    // - Only s2 owns the data
    // - Only s2 will free it
    // - No double-free possible!

    // ============================================================================
    // WHEN TO USE EACH
    // ============================================================================
    //
    // Use Copy types:
    // - Small, fixed-size data (fits in a few CPU registers)
    // - Data that lives entirely on stack
    // - No heap allocation
    // - No complex cleanup needed
    // - Examples: coordinates, counters, flags, IDs
    //
    // Use Move types:
    // - Variable-size data (String, Vec)
    // - Heap-allocated data
    // - Expensive to copy
    // - Resources that need cleanup (files, network connections)
    // - Examples: strings, collections, smart pointers
    //
    // Use clone() when:
    // - You need a deep copy of move type
    // - You're willing to pay the performance cost
    // - You need independent copies
    // - Example: caching, backups, duplicating data structures

    // ============================================================================
    // RUST VS OTHER LANGUAGES
    // ============================================================================
    //
    // C++ (complex copy semantics):
    // ```cpp
    // std::string s1 = "hello";
    // std::string s2 = s1;  // COPIES! Allocates new heap memory
    // // Both valid, but hidden allocation happened
    // ```
    // Issues:
    // - Assignment copies by default (expensive!)
    // - Must use std::move for move semantics
    // - Easy to accidentally copy expensive objects
    // - No compiler help to track ownership
    //
    // Python (everything is references):
    // ```python
    // x = [1, 2, 3]
    // y = x  // Both refer to same list!
    // y.append(4)
    // print(x)  // [1, 2, 3, 4] - x was modified too!
    // ```
    // Issues:
    // - Assignment doesn't copy, just creates another reference
    // - Surprising behavior (modifying y modifies x)
    // - Need to explicitly use copy.deepcopy()
    // - Garbage collector required
    //
    // JavaScript (similar to Python):
    // ```javascript
    // let obj1 = {value: 10};
    // let obj2 = obj1;  // Both reference same object
    // obj2.value = 20;
    // console.log(obj1.value);  // 20 - surprise!
    // ```
    //
    // Go (slices/maps are references, others copy):
    // ```go
    // x := 5
    // y := x  // Copy
    //
    // s1 := []int{1, 2, 3}
    // s2 := s1  // Reference! Both point to same array
    // ```
    // Issues:
    // - Inconsistent: some types copy, some don't
    // - Hard to know what's expensive
    //
    // Rust advantages:
    // - Explicit: Copy types copy, others move
    // - Performance: No hidden expensive operations
    // - Safety: Ownership prevents double-free
    // - Clarity: .clone() makes copies visible in code
    // - Consistency: Rules are the same everywhere
}
