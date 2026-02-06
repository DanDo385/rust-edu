// Project 44: Operator Overloading
//
// Demonstrates implementing operator traits for custom types.
// We'll build Rational (fraction) and Complex number types with full operator support.

use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

fn main() {
    println!("=== Operator Overloading ===\n");

    // Example 1: Rational numbers
    println!("1. Rational Number Arithmetic");
    rational_demo();

    println!("
{}
", "=".repeat(60));

    // Example 2: Complex numbers
    println!("2. Complex Number Arithmetic");
    complex_demo();

    println!("
{}
", "=".repeat(60));

    // Example 3: Operator implementations for references
    println!("3. Reference Operators");
    reference_demo();

    println!("
{}
", "=".repeat(60));

    // Example 4: Comparison operators
    println!("4. Comparison Operators");
    comparison_demo();

    println!("
{}
", "=".repeat(60));

    // Example 5: Compound assignment operators
    println!("5. Compound Assignment Operators");
    assignment_demo();

    println!("
{}
", "=".repeat(60));

    // Example 6: Generic functions with operator bounds
    println!("6. Generic Functions with Operator Bounds");
    generic_demo();

    println!("\n=== Operator Overloading Complete ===");
}

// ============================================================================
// RATIONAL NUMBERS (FRACTIONS)
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
struct Rational {
    numerator: i64,
    denominator: i64,
}

impl Rational {
    fn new(numerator: i64, denominator: i64) -> Self {
        if denominator == 0 {
            panic!("Denominator cannot be zero");
        }

        // Simplify the fraction
        let gcd = gcd(numerator.abs(), denominator.abs());
        let mut r = Rational {
            numerator: numerator / gcd,
            denominator: denominator / gcd,
        };

        // Ensure denominator is positive
        if r.denominator < 0 {
            r.numerator = -r.numerator;
            r.denominator = -r.denominator;
        }

        r
    }

    fn to_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

// Greatest common divisor (Euclidean algorithm)
fn gcd(mut a: i64, mut b: i64) -> i64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// Display formatting
impl fmt::Display for Rational {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.denominator == 1 {
            write!(f, "{}", self.numerator)
        } else {
            write!(f, "{}/{}", self.numerator, self.denominator)
        }
    }
}

// Addition: a/b + c/d = (ad + bc) / bd
impl Add for Rational {
    type Output = Rational;

    fn add(self, other: Rational) -> Rational {
        Rational::new(
            self.numerator * other.denominator + other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }
}

// Subtraction: a/b - c/d = (ad - bc) / bd
impl Sub for Rational {
    type Output = Rational;

    fn sub(self, other: Rational) -> Rational {
        Rational::new(
            self.numerator * other.denominator - other.numerator * self.denominator,
            self.denominator * other.denominator,
        )
    }
}

// Multiplication: (a/b) * (c/d) = (ac) / (bd)
impl Mul for Rational {
    type Output = Rational;

    fn mul(self, other: Rational) -> Rational {
        Rational::new(
            self.numerator * other.numerator,
            self.denominator * other.denominator,
        )
    }
}

// Division: (a/b) / (c/d) = (ad) / (bc)
impl Div for Rational {
    type Output = Rational;

    fn div(self, other: Rational) -> Rational {
        Rational::new(
            self.numerator * other.denominator,
            self.denominator * other.numerator,
        )
    }
}

// Negation: -(a/b) = -a/b
impl Neg for Rational {
    type Output = Rational;

    fn neg(self) -> Rational {
        Rational::new(-self.numerator, self.denominator)
    }
}

// ============================================================================
// COMPLEX NUMBERS
// ============================================================================

#[derive(Debug, Clone, Copy, PartialEq)]
struct Complex {
    real: f64,
    imag: f64,
}

impl Complex {
    fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }

    fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    fn conjugate(&self) -> Complex {
        Complex::new(self.real, -self.imag)
    }
}

// Display formatting
impl fmt::Display for Complex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        if self.imag >= 0.0 {
            write!(f, "{} + {}i", self.real, self.imag)
        } else {
            write!(f, "{} - {}i", self.real, -self.imag)
        }
    }
}

// Addition: (a + bi) + (c + di) = (a+c) + (b+d)i
impl Add for Complex {
    type Output = Complex;

    fn add(self, other: Complex) -> Complex {
        Complex::new(self.real + other.real, self.imag + other.imag)
    }
}

// Subtraction: (a + bi) - (c + di) = (a-c) + (b-d)i
impl Sub for Complex {
    type Output = Complex;

    fn sub(self, other: Complex) -> Complex {
        Complex::new(self.real - other.real, self.imag - other.imag)
    }
}

// Multiplication: (a + bi)(c + di) = (ac - bd) + (ad + bc)i
impl Mul for Complex {
    type Output = Complex;

    fn mul(self, other: Complex) -> Complex {
        Complex::new(
            self.real * other.real - self.imag * other.imag,
            self.real * other.imag + self.imag * other.real,
        )
    }
}

// Division: (a + bi) / (c + di) = [(ac + bd) + (bc - ad)i] / (c² + d²)
impl Div for Complex {
    type Output = Complex;

    fn div(self, other: Complex) -> Complex {
        let denominator = other.real * other.real + other.imag * other.imag;
        Complex::new(
            (self.real * other.real + self.imag * other.imag) / denominator,
            (self.imag * other.real - self.real * other.imag) / denominator,
        )
    }
}

// Negation: -(a + bi) = -a - bi
impl Neg for Complex {
    type Output = Complex;

    fn neg(self) -> Complex {
        Complex::new(-self.real, -self.imag)
    }
}

// ============================================================================
// REFERENCE IMPLEMENTATIONS
// ============================================================================

// Allow operations on references (avoids moving/copying large values)
impl Add for &Rational {
    type Output = Rational;

    fn add(self, other: &Rational) -> Rational {
        *self + *other
    }
}

impl Add for &Complex {
    type Output = Complex;

    fn add(self, other: &Complex) -> Complex {
        *self + *other
    }
}

// ============================================================================
// COMPOUND ASSIGNMENT OPERATORS
// ============================================================================

impl AddAssign for Rational {
    fn add_assign(&mut self, other: Rational) {
        *self = *self + other;
    }
}

impl SubAssign for Rational {
    fn sub_assign(&mut self, other: Rational) {
        *self = *self - other;
    }
}

impl MulAssign for Rational {
    fn mul_assign(&mut self, other: Rational) {
        *self = *self * other;
    }
}

impl DivAssign for Rational {
    fn div_assign(&mut self, other: Rational) {
        *self = *self / other;
    }
}

impl AddAssign for Complex {
    fn add_assign(&mut self, other: Complex) {
        *self = *self + other;
    }
}

impl SubAssign for Complex {
    fn sub_assign(&mut self, other: Complex) {
        *self = *self - other;
    }
}

impl MulAssign for Complex {
    fn mul_assign(&mut self, other: Complex) {
        *self = *self * other;
    }
}

impl DivAssign for Complex {
    fn div_assign(&mut self, other: Complex) {
        *self = *self / other;
    }
}

// ============================================================================
// COMPARISON IMPLEMENTATIONS
// ============================================================================

impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let left = self.numerator * other.denominator;
        let right = other.numerator * self.denominator;
        left.partial_cmp(&right)
    }
}

impl Ord for Rational {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        let left = self.numerator * other.denominator;
        let right = other.numerator * self.denominator;
        left.cmp(&right)
    }
}

// ============================================================================
// DEMO FUNCTIONS
// ============================================================================

fn rational_demo() {
    let a = Rational::new(1, 2); // 1/2
    let b = Rational::new(1, 3); // 1/3
    let c = Rational::new(2, 4); // 2/4 = 1/2 (simplified)

    println!("  a = {}", a);
    println!("  b = {}", b);
    println!("  c = {} (simplified from 2/4)", c);

    println!("\n  Arithmetic:");
    println!("    {} + {} = {}", a, b, a + b); // 1/2 + 1/3 = 5/6
    println!("    {} - {} = {}", a, b, a - b); // 1/2 - 1/3 = 1/6
    println!("    {} * {} = {}", a, b, a * b); // 1/2 * 1/3 = 1/6
    println!("    {} / {} = {}", a, b, a / b); // 1/2 / 1/3 = 3/2

    println!("\n  Negation:");
    println!("    -{} = {}", a, -a);

    println!("\n  Floating point conversion:");
    println!("    {} = {}", a, a.to_f64());
    println!("    {} = {}", b, b.to_f64());

    println!("\n  Equality:");
    println!("    {} == {} ? {}", a, c, a == c); // 1/2 == 1/2
    println!("    {} == {} ? {}", a, b, a == b); // 1/2 != 1/3
}

fn complex_demo() {
    let a = Complex::new(1.0, 2.0); // 1 + 2i
    let b = Complex::new(3.0, 4.0); // 3 + 4i

    println!("  a = {}", a);
    println!("  b = {}", b);

    println!("\n  Arithmetic:");
    println!("    {} + {} = {}", a, b, a + b);
    println!("    {} - {} = {}", a, b, a - b);
    println!("    {} * {} = {}", a, b, a * b);
    println!("    {} / {} = {}", a, b, a / b);

    println!("\n  Negation:");
    println!("    -{} = {}", a, -a);

    println!("\n  Magnitude and conjugate:");
    println!("    |{}| = {:.3}", a, a.magnitude());
    println!("    conjugate({}) = {}", a, a.conjugate());

    println!("\n  Complex multiplication example:");
    println!("    (1 + 2i) * (3 + 4i) = {} (expected: -5 + 10i)", a * b);
}

fn reference_demo() {
    let a = Complex::new(1.0, 1.0);
    let b = Complex::new(2.0, 2.0);

    println!("  a = {}", a);
    println!("  b = {}", b);

    // Using references (doesn't move a and b)
    let c = &a + &b;
    println!("\n  Using references:");
    println!("    &a + &b = {}", c);

    // Can still use a and b
    println!("    a is still available: {}", a);
    println!("    b is still available: {}", b);

    // Without references (would move a and b)
    let d = a + b;
    println!("\n  Without references (values moved):");
    println!("    a + b = {}", d);
    // println!("    a = {}", a);  // Error: a was moved
}

fn comparison_demo() {
    let a = Rational::new(1, 2); // 1/2
    let b = Rational::new(2, 3); // 2/3
    let c = Rational::new(1, 2); // 1/2

    println!("  a = {}", a);
    println!("  b = {}", b);
    println!("  c = {}", c);

    println!("\n  Equality:");
    println!("    {} == {} ? {}", a, c, a == c);
    println!("    {} != {} ? {}", a, b, a != b);

    println!("\n  Ordering:");
    println!("    {} < {} ? {}", a, b, a < b); // 1/2 < 2/3
    println!("    {} > {} ? {}", a, b, a > b);
    println!("    {} <= {} ? {}", a, c, a <= c);
    println!("    {} >= {} ? {}", a, c, a >= c);
}

fn assignment_demo() {
    let mut a = Rational::new(1, 2);
    let b = Rational::new(1, 3);

    println!("  Initial: a = {}, b = {}", a, b);

    a += b;
    println!("  After a += b: a = {}", a);

    a *= Rational::new(2, 1);
    println!("  After a *= 2: a = {}", a);

    a -= Rational::new(1, 6);
    println!("  After a -= 1/6: a = {}", a);

    a /= Rational::new(2, 1);
    println!("  After a /= 2: a = {}", a);

    println!("\n  Complex numbers:");
    let mut z = Complex::new(1.0, 0.0);
    println!("  Initial: z = {}", z);

    z += Complex::new(0.0, 1.0);
    println!("  After z += i: z = {}", z);

    z *= Complex::new(0.0, 1.0);
    println!("  After z *= i: z = {} (rotation by 90°)", z);
}

fn generic_demo() {
    // Generic function that works with any type implementing Add
    fn add_three<T>(a: T, b: T, c: T) -> T
    where
        T: Add<Output = T>,
    {
        a + b + c
    }

    // Works with Rational
    let r1 = Rational::new(1, 2);
    let r2 = Rational::new(1, 3);
    let r3 = Rational::new(1, 6);
    let sum_r = add_three(r1, r2, r3);
    println!("  add_three({}, {}, {}) = {}", r1, r2, r3, sum_r);

    // Works with Complex
    let c1 = Complex::new(1.0, 0.0);
    let c2 = Complex::new(0.0, 1.0);
    let c3 = Complex::new(1.0, 1.0);
    let sum_c = add_three(c1, c2, c3);
    println!("  add_three({}, {}, {}) = {}", c1, c2, c3, sum_c);

    // Generic multiply function
    fn square<T>(x: T) -> T
    where
        T: Mul<Output = T> + Copy,
    {
        x * x
    }

    println!("\n  Squaring:");
    let r = Rational::new(2, 3);
    println!("    ({})² = {}", r, square(r));

    let z = Complex::new(1.0, 1.0);
    println!("    ({})² = {}", z, square(z));
}

// ============================================================================
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. OPERATOR DESUGARING
//    - The expression `a + b` is syntactic sugar for `a.add(b)`
//    - The compiler rewrites operators to method calls
//    - This happens during parsing, before type checking
//
// 2. TRAIT RESOLUTION
//    - The compiler looks for an `Add` trait implementation
//    - It checks type compatibility (Self + Rhs = Output)
//    - Trait bounds are checked at compile-time
//
// 3. INLINING
//    - Simple operator methods are inlined (no function call overhead)
//    - #[inline] attribute is often applied automatically
//    - Result: same performance as hand-written code
//
// 4. MOVE SEMANTICS
//    - `a + b` consumes both `a` and `b` (they're moved)
//    - `&a + &b` borrows them (doesn't move)
//    - Implement both for flexibility
//
// 5. ZERO-COST ABSTRACTION
//    - Custom operators compile to the same assembly as primitives
//    - No runtime overhead for operator overloading
//    - This is a key Rust design principle

// ============================================================================
// KEY TAKEAWAYS
// ============================================================================
// 1. Operators are implemented via traits (Add, Sub, Mul, etc.)
// 2. All operator overloading is explicit (no implicit conversions)
// 3. Implement operators for both owned and borrowed types
// 4. Use compound assignment traits (+=, -=, etc.) for efficiency
// 5. Respect mathematical properties (commutativity, associativity)
// 6. Custom operators have zero runtime cost (inlined)
// 7. Type safety prevents accidental type mixing
// 8. Generic functions can use operator bounds
// 9. Display trait makes custom types printable
// 10. Rational numbers provide exact arithmetic (no rounding errors)

// ============================================================================
// MATHEMATICAL PROPERTIES DEMONSTRATED
// ============================================================================
// COMMUTATIVITY (a + b == b + a):
//   - Addition: ✓
//   - Multiplication: ✓
//   - Subtraction: ✗
//   - Division: ✗
//
// ASSOCIATIVITY ((a + b) + c == a + (b + c)):
//   - Addition: ✓
//   - Multiplication: ✓
//
// IDENTITY ELEMENT:
//   - Addition: 0 (a + 0 == a)
//   - Multiplication: 1 (a * 1 == a)
//
// INVERSE ELEMENT:
//   - Negation: -a (a + (-a) == 0)
//   - Reciprocal: 1/a (a * (1/a) == 1)

// ============================================================================
// COMMON BEGINNER MISTAKES
// ============================================================================
// ❌ Only implementing operators for owned types (not &T)
// ❌ Forgetting AddAssign, SubAssign, etc.
// ❌ Not handling edge cases (division by zero, overflow)
// ❌ Violating mathematical properties (non-commutative addition)
// ❌ Not simplifying rational numbers (2/4 should be 1/2)
// ❌ Comparing floating-point complex numbers with == (precision issues)
// ❌ Moving values when references would work
// ❌ Panicking in operators (use Result instead for fallible ops)

// ============================================================================
// PERFORMANCE TIPS
// ============================================================================
// 1. Implement Copy for small types (< 16 bytes)
// 2. Use &T for large types to avoid copying
// 3. Inline small methods with #[inline]
// 4. Avoid allocations in hot paths
// 5. Use compound assignment (+=) instead of a = a + b
// 6. Simplify fractions eagerly (prevents overflow)
// 7. Use SIMD for vector operations (requires unstable features)

// ============================================================================
// REAL-WORLD USE CASES
// ============================================================================
// - Computer graphics: Vectors, matrices, quaternions
// - Game development: Physics calculations
// - Robotics: 3D transformations
// - Signal processing: Complex numbers (FFT)
// - Financial calculations: Exact rational arithmetic
// - Computer algebra: Symbolic math
// - Machine learning: Matrix operations
// - Scientific computing: Linear algebra

// ============================================================================
// ADVANCED TOPICS
// ============================================================================
// - Implementing Index and IndexMut for custom containers
// - Bitwise operators for custom bit sets
// - Deref and DerefMut for smart pointers
// - Generic operators with associated types
// - Operator overloading in const contexts
// - SIMD operations for vectorized math
