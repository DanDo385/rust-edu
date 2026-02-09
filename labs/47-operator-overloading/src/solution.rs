// Lab 47: Operator Overloading
//
// This module implements Rational (fraction) and Complex number types with
// full operator support. Demonstrates implementing operator traits for
// custom types in Rust.
//
// Key concepts:
// - std::ops traits (Add, Sub, Mul, Div, Neg)
// - Compound assignment traits (AddAssign, SubAssign, etc.)
// - Reference operator implementations
// - Comparison traits (PartialOrd, Ord)
// - Display formatting
// - Generic functions with operator bounds

use std::fmt;
use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

// ============================================================================
// RATIONAL NUMBERS (FRACTIONS)
// ============================================================================

/// A rational number (fraction) with exact arithmetic.
///
/// Rational numbers are automatically simplified upon creation.
/// The denominator is always positive.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    pub numerator: i64,
    pub denominator: i64,
}

impl Rational {
    /// Create a new Rational number, simplified to lowest terms.
    ///
    /// # Panics
    /// Panics if `denominator` is zero.
    pub fn new(numerator: i64, denominator: i64) -> Self {
        if denominator == 0 {
            panic!("Denominator cannot be zero");
        }

        let g = gcd(numerator.abs(), denominator.abs());
        let mut r = Rational {
            numerator: numerator / g,
            denominator: denominator / g,
        };

        // Ensure denominator is positive
        if r.denominator < 0 {
            r.numerator = -r.numerator;
            r.denominator = -r.denominator;
        }

        r
    }

    /// Convert to f64 (floating-point approximation).
    pub fn to_f64(&self) -> f64 {
        self.numerator as f64 / self.denominator as f64
    }
}

/// Greatest common divisor (Euclidean algorithm).
pub fn gcd(mut a: i64, mut b: i64) -> i64 {
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

// Reference implementations
impl Add for &Rational {
    type Output = Rational;

    fn add(self, other: &Rational) -> Rational {
        *self + *other
    }
}

// Compound assignment operators
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

// Comparison
impl PartialOrd for Rational {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
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
// COMPLEX NUMBERS
// ============================================================================

/// A complex number with f64 real and imaginary parts.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    /// Create a new complex number.
    pub fn new(real: f64, imag: f64) -> Self {
        Complex { real, imag }
    }

    /// Compute the magnitude (absolute value) of the complex number.
    ///
    /// |a + bi| = sqrt(a^2 + b^2)
    pub fn magnitude(&self) -> f64 {
        (self.real * self.real + self.imag * self.imag).sqrt()
    }

    /// Compute the complex conjugate.
    ///
    /// conjugate(a + bi) = a - bi
    pub fn conjugate(&self) -> Complex {
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

// Division: (a + bi) / (c + di) = [(ac + bd) + (bc - ad)i] / (c^2 + d^2)
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

// Reference implementations
impl Add for &Complex {
    type Output = Complex;

    fn add(self, other: &Complex) -> Complex {
        *self + *other
    }
}

// Compound assignment operators
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
// WHAT RUST DOES UNDER THE HOOD
// ============================================================================
// 1. OPERATOR DESUGARING
//    - `a + b` is syntactic sugar for `a.add(b)`
//    - The compiler rewrites operators to method calls
//
// 2. TRAIT RESOLUTION
//    - The compiler looks for an Add trait implementation
//    - Trait bounds are checked at compile-time
//
// 3. INLINING
//    - Simple operator methods are inlined (no function call overhead)
//    - Same performance as hand-written code
//
// 4. MOVE SEMANTICS
//    - `a + b` consumes both (they're moved)
//    - `&a + &b` borrows them (doesn't move)
//    - Implement both for flexibility
