//! # Structs and Methods - Complete Solution with EXHAUSTIVE Explanations

/// A rectangle with width and height.
///
/// ## Memory Layout
/// ```ignore
/// Stack:
/// ┌────────────────┐
/// │ Rectangle      │
/// │  width: u32    │  4 bytes
/// │  height: u32   │  4 bytes
/// └────────────────┘
/// Total: 8 bytes on stack, no heap allocation
/// ```ignore
#[derive(Debug, Clone, PartialEq)]
pub struct Rectangle {
    pub width: u32,
    pub height: u32,
}

impl Rectangle {
    /// Creates a new Rectangle if dimensions are valid (non-zero).
    ///
    /// ## Parameters
    /// - `width`: Width in pixels (must be > 0)
    /// - `height`: Height in pixels (must be > 0)
    ///
    /// ## Returns
    /// - `Some(Rectangle)` if both dimensions are positive
    /// - `None` if either dimension is zero
    ///
    /// ## Why Option<Rectangle>?
    /// - Zero width or height doesn't make sense for a rectangle
    /// - Option forces caller to handle invalid case
    /// - Better than panicking or returning dummy values
    ///
    /// ## Example
    /// ```ignore
    /// let rect = Rectangle::new(10, 20);
    /// assert!(rect.is_some());
    /// let invalid = Rectangle::new(0, 20);
    /// assert!(invalid.is_none());
    /// ```ignore
    pub fn new(width: u32, height: u32) -> Option<Rectangle> {
        // ====================================================================
        // VALIDATE DIMENSIONS
        // ====================================================================

        // Check if both dimensions are non-zero
        // `width > 0 && height > 0` = both must be positive
        //   - `width > 0` checks width is not zero
        //   - `&&` = logical AND (both must be true)
        //   - `height > 0` checks height is not zero
        //
        // Why validate?
        // - Rectangle with zero width or height doesn't make geometric sense
        // - Prevents bugs downstream (division by zero, etc.)
        // - Makes API clear about what's valid

        if width > 0 && height > 0 {
            // Valid dimensions—create the Rectangle
            //
            // `Some(Rectangle { width, height })` = wrap in Some
            //   - `Rectangle { width, height }` = struct instantiation
            //   - Field shorthand: `width` instead of `width: width`
            //   - Creates Rectangle on stack (just 8 bytes)
            //   - Wraps in Some to indicate success

            Some(Rectangle { width, height })
        } else {
            // Invalid dimensions—return None
            //
            // `None` = indicates validation failed
            //   - Caller must handle this case
            //   - No Rectangle created
            //   - Type-safe way to indicate failure

            None
        }

        // ====================================================================
        // ASSOCIATED FUNCTION VS METHOD
        // ====================================================================
        //
        // This is an "associated function" (not a method):
        // - No `self` parameter
        // - Called with `Rectangle::new(...)` (not `rect.new(...)`)
        // - Like a static method in other languages
        // - Often used for constructors
        //
        // Methods have `self`:
        // - `&self` = immutable borrow
        // - `&mut self` = mutable borrow
        // - `self` = takes ownership
        // - Called with dot notation: `rect.area()`
    }

    /// Calculates the area of the rectangle.
    ///
    /// ## Returns
    /// Area as u32 (width × height)
    ///
    /// ## Method Signature: `&self`
    /// - `&self` means immutable borrow of the Rectangle
    /// - Can read fields but not modify them
    /// - Caller keeps ownership
    /// - Can call this method many times
    ///
    /// ## Example
    /// ```ignore
    /// let rect = Rectangle::new(10, 20).unwrap();
    /// assert_eq!(rect.area(), 200);
    /// ```ignore
    pub fn area(&self) -> u32 {
        // `self.width * self.height` = multiply dimensions
        //   - `self` = reference to the Rectangle instance
        //   - `.width` = access the width field
        //   - `*` = multiplication operator
        //   - `.height` = access the height field
        //   - Result: u32 (width and height are u32)
        //
        // Why &self?
        // - We only need to read the fields
        // - Don't need to modify the Rectangle
        // - Don't need to take ownership
        // - Most efficient—just reading two u32 values

        self.width * self.height
    }

    /// Calculates the perimeter of the rectangle.
    ///
    /// ## Returns
    /// Perimeter as u32 (2 × (width + height))
    ///
    /// ## Example
    /// ```ignore
    /// let rect = Rectangle::new(10, 20).unwrap();
    /// assert_eq!(rect.perimeter(), 60);
    /// ```ignore
    pub fn perimeter(&self) -> u32 {
        // Perimeter formula: 2 × (width + height)
        //
        // `2 * (self.width + self.height)` = calculate perimeter
        //   - `self.width + self.height` = sum of dimensions
        //   - Parentheses ensure addition happens first
        //   - `2 *` = multiply by 2 (two widths, two heights)
        //
        // Order of operations:
        // 1. Add width and height
        // 2. Multiply by 2

        2 * (self.width + self.height)
    }

    /// Checks if another rectangle can fit inside this one.
    ///
    /// ## Parameters
    /// - `other`: Another rectangle to test
    ///
    /// ## Returns
    /// `true` if other fits inside self (both dimensions must fit)
    ///
    /// ## Example
    /// ```ignore
    /// let big = Rectangle::new(100, 100).unwrap();
    /// let small = Rectangle::new(50, 50).unwrap();
    /// assert!(big.can_fit(&small));
    /// assert!(!small.can_fit(&big));
    /// ```ignore
    pub fn can_fit(&self, other: &Rectangle) -> bool {
        // Check if other rectangle fits inside this one
        //
        // `self.width >= other.width && self.height >= other.height`
        //   - `self.width >= other.width` = our width fits their width
        //   - `&&` = logical AND (both must be true)
        //   - `self.height >= other.height` = our height fits their height
        //   - BOTH dimensions must be >= for rectangle to fit
        //
        // Why `>=` not `>`?
        // - A rectangle can fit inside one of equal size (just touches edges)
        // - If you want strict containment, use `>` instead
        //
        // Note: This doesn't account for rotation
        // - If rotation allowed, would need to check both orientations
        // - This assumes same orientation

        self.width >= other.width && self.height >= other.height
    }

    /// Creates a new rectangle scaled by a factor.
    ///
    /// ## Parameters
    /// - `factor`: Scaling factor (2 = double size, 3 = triple, etc.)
    ///
    /// ## Returns
    /// New Rectangle with scaled dimensions
    ///
    /// ## Method Signature: `&self`
    /// - Borrows self immutably
    /// - Returns new Rectangle (doesn't modify original)
    /// - Original Rectangle unchanged
    ///
    /// ## Example
    /// ```ignore
    /// let rect = Rectangle::new(10, 20).unwrap();
    /// let doubled = rect.scale(2);
    /// assert_eq!(doubled.width, 20);
    /// assert_eq!(doubled.height, 40);
    /// assert_eq!(rect.width, 10); // Original unchanged
    /// ```ignore
    pub fn scale(&self, factor: u32) -> Rectangle {
        // Create new Rectangle with scaled dimensions
        //
        // `Rectangle { width: self.width * factor, height: self.height * factor }`
        //   - Create new Rectangle instance
        //   - `width: self.width * factor` = scale width
        //   - `height: self.height * factor` = scale height
        //   - Returns new Rectangle (doesn't modify self)
        //
        // Why not &mut self?
        // - Mathematical operations usually don't mutate
        // - Caller might want original Rectangle unchanged
        // - Can chain: `rect.scale(2).scale(3)` (6x scaling)
        // - More functional style (immutable operations)
        //
        // Alternative design (mutating):
        // ```rust
        // pub fn scale_mut(&mut self, factor: u32) {
        //     self.width *= factor;
        //     self.height *= factor;
        // }
        // ```
        // This would modify in place—different use case!

        Rectangle {
            width: self.width * factor,
            height: self.height * factor,
        }
    }
}

// ============================================================================
// UNDERSTANDING STRUCTS
// ============================================================================
//
// What is a struct?
// - Custom data type that groups related data
// - Like a class in other languages (but no inheritance)
// - Can have fields (data) and methods (behavior)
// - Stored on stack if all fields are stack types
//
// Anatomy of our Rectangle struct:
// ```rust
// #[derive(Debug, Clone, PartialEq)]  // Derived traits
// pub struct Rectangle {               // Struct definition
//     pub width: u32,                  // Public field
//     pub height: u32,                 // Public field
// }
// ```
//
// Derived traits:
// - `Debug`: Enables {:?} formatting for debugging
// - `Clone`: Enables .clone() for explicit copying
// - `PartialEq`: Enables == and != operators
//
// Why pub fields?
// - In this simple case, allowing direct access is fine
// - In production, might use private fields with getters:
//   ```rust
//   struct Rectangle {
//       width: u32,  // Private
//       height: u32, // Private
//   }
//   impl Rectangle {
//       pub fn width(&self) -> u32 { self.width }
//       pub fn set_width(&mut self, w: u32) { self.width = w; }
//   }
//   ```

// ============================================================================
// UNDERSTANDING impl BLOCKS
// ============================================================================
//
// `impl Rectangle { ... }` = implementation block
// - Associates functions/methods with a type
// - Can have multiple impl blocks for same type
// - Methods have `self` parameter
// - Associated functions don't have `self` (like ::new)
//
// Three forms of self:
//
// 1. `&self` - Immutable borrow (most common):
//    - Can read fields
//    - Can't modify fields
//    - Can't take ownership
//    - Examples: getters, calculations
//
// 2. `&mut self` - Mutable borrow:
//    - Can read fields
//    - Can modify fields
//    - Can't take ownership
//    - Examples: setters, in-place operations
//
// 3. `self` - Takes ownership (rare):
//    - Can do anything
//    - Consumes the value
//    - Can't use after calling this method
//    - Examples: into_string(), unwrap()
//
// Associated functions (no self):
//    - Called with Type::function()
//    - Often used for constructors (::new, ::default)
//    - Can't access instance fields
