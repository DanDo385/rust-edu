// Integration tests for Lab 47: Operator Overloading
//
// Tests Rational and Complex number types with all operator implementations,
// mathematical properties, edge cases, and generic functions.

use operator_overloading::*;

// ============================================================================
// RATIONAL NUMBER CREATION TESTS
// ============================================================================

#[test]
fn test_rational_new_basic() {
    let r = Rational::new(1, 2);
    assert_eq!(r.numerator, 1);
    assert_eq!(r.denominator, 2);
}

#[test]
fn test_rational_simplification() {
    let r = Rational::new(2, 4);
    assert_eq!(r.numerator, 1);
    assert_eq!(r.denominator, 2);
}

#[test]
fn test_rational_simplification_large() {
    let r = Rational::new(12, 18);
    assert_eq!(r.numerator, 2);
    assert_eq!(r.denominator, 3);
}

#[test]
fn test_rational_negative_numerator() {
    let r = Rational::new(-1, 2);
    assert_eq!(r.numerator, -1);
    assert_eq!(r.denominator, 2);
}

#[test]
fn test_rational_negative_denominator() {
    let r = Rational::new(1, -2);
    // Denominator should be positive
    assert_eq!(r.numerator, -1);
    assert_eq!(r.denominator, 2);
}

#[test]
fn test_rational_both_negative() {
    let r = Rational::new(-3, -6);
    assert_eq!(r.numerator, 1);
    assert_eq!(r.denominator, 2);
}

#[test]
fn test_rational_zero_numerator() {
    let r = Rational::new(0, 5);
    assert_eq!(r.numerator, 0);
    assert_eq!(r.denominator, 1); // 0/5 simplifies to 0/1
}

#[test]
fn test_rational_whole_number() {
    let r = Rational::new(6, 3);
    assert_eq!(r.numerator, 2);
    assert_eq!(r.denominator, 1);
}

#[test]
#[should_panic(expected = "Denominator cannot be zero")]
fn test_rational_zero_denominator() {
    Rational::new(1, 0);
}

// ============================================================================
// RATIONAL ARITHMETIC TESTS
// ============================================================================

#[test]
fn test_rational_add() {
    let a = Rational::new(1, 2);
    let b = Rational::new(1, 3);
    let result = a + b;
    assert_eq!(result, Rational::new(5, 6));
}

#[test]
fn test_rational_add_same_denominator() {
    let a = Rational::new(1, 4);
    let b = Rational::new(1, 4);
    let result = a + b;
    assert_eq!(result, Rational::new(1, 2));
}

#[test]
fn test_rational_sub() {
    let a = Rational::new(1, 2);
    let b = Rational::new(1, 3);
    let result = a - b;
    assert_eq!(result, Rational::new(1, 6));
}

#[test]
fn test_rational_sub_to_zero() {
    let a = Rational::new(1, 2);
    let b = Rational::new(1, 2);
    let result = a - b;
    assert_eq!(result, Rational::new(0, 1));
}

#[test]
fn test_rational_mul() {
    let a = Rational::new(1, 2);
    let b = Rational::new(1, 3);
    let result = a * b;
    assert_eq!(result, Rational::new(1, 6));
}

#[test]
fn test_rational_mul_by_zero() {
    let a = Rational::new(1, 2);
    let b = Rational::new(0, 1);
    let result = a * b;
    assert_eq!(result, Rational::new(0, 1));
}

#[test]
fn test_rational_div() {
    let a = Rational::new(1, 2);
    let b = Rational::new(1, 3);
    let result = a / b;
    assert_eq!(result, Rational::new(3, 2));
}

#[test]
fn test_rational_neg() {
    let a = Rational::new(1, 2);
    let result = -a;
    assert_eq!(result, Rational::new(-1, 2));
}

#[test]
fn test_rational_neg_negative() {
    let a = Rational::new(-3, 4);
    let result = -a;
    assert_eq!(result, Rational::new(3, 4));
}

// ============================================================================
// RATIONAL MATHEMATICAL PROPERTIES
// ============================================================================

#[test]
fn test_rational_add_commutativity() {
    let a = Rational::new(2, 3);
    let b = Rational::new(4, 5);
    assert_eq!(a + b, b + a);
}

#[test]
fn test_rational_mul_commutativity() {
    let a = Rational::new(2, 3);
    let b = Rational::new(4, 5);
    assert_eq!(a * b, b * a);
}

#[test]
fn test_rational_add_identity() {
    let a = Rational::new(3, 7);
    let zero = Rational::new(0, 1);
    assert_eq!(a + zero, a);
}

#[test]
fn test_rational_mul_identity() {
    let a = Rational::new(3, 7);
    let one = Rational::new(1, 1);
    assert_eq!(a * one, a);
}

#[test]
fn test_rational_additive_inverse() {
    let a = Rational::new(3, 7);
    let result = a + (-a);
    assert_eq!(result, Rational::new(0, 1));
}

#[test]
fn test_rational_multiplicative_inverse() {
    let a = Rational::new(3, 7);
    let reciprocal = Rational::new(7, 3);
    let result = a * reciprocal;
    assert_eq!(result, Rational::new(1, 1));
}

// ============================================================================
// RATIONAL COMPARISON TESTS
// ============================================================================

#[test]
fn test_rational_equality() {
    assert_eq!(Rational::new(1, 2), Rational::new(2, 4));
}

#[test]
fn test_rational_inequality() {
    assert_ne!(Rational::new(1, 2), Rational::new(1, 3));
}

#[test]
fn test_rational_less_than() {
    assert!(Rational::new(1, 3) < Rational::new(1, 2));
}

#[test]
fn test_rational_greater_than() {
    assert!(Rational::new(2, 3) > Rational::new(1, 2));
}

#[test]
fn test_rational_less_equal() {
    assert!(Rational::new(1, 2) <= Rational::new(1, 2));
    assert!(Rational::new(1, 3) <= Rational::new(1, 2));
}

#[test]
fn test_rational_greater_equal() {
    assert!(Rational::new(1, 2) >= Rational::new(1, 2));
    assert!(Rational::new(2, 3) >= Rational::new(1, 2));
}

#[test]
fn test_rational_sorting() {
    let mut fractions = vec![
        Rational::new(3, 4),
        Rational::new(1, 2),
        Rational::new(1, 4),
        Rational::new(7, 8),
    ];
    fractions.sort();
    assert_eq!(fractions[0], Rational::new(1, 4));
    assert_eq!(fractions[1], Rational::new(1, 2));
    assert_eq!(fractions[2], Rational::new(3, 4));
    assert_eq!(fractions[3], Rational::new(7, 8));
}

// ============================================================================
// RATIONAL COMPOUND ASSIGNMENT TESTS
// ============================================================================

#[test]
fn test_rational_add_assign() {
    let mut a = Rational::new(1, 2);
    a += Rational::new(1, 3);
    assert_eq!(a, Rational::new(5, 6));
}

#[test]
fn test_rational_sub_assign() {
    let mut a = Rational::new(5, 6);
    a -= Rational::new(1, 6);
    assert_eq!(a, Rational::new(2, 3));
}

#[test]
fn test_rational_mul_assign() {
    let mut a = Rational::new(2, 3);
    a *= Rational::new(3, 4);
    assert_eq!(a, Rational::new(1, 2));
}

#[test]
fn test_rational_div_assign() {
    let mut a = Rational::new(1, 2);
    a /= Rational::new(2, 1);
    assert_eq!(a, Rational::new(1, 4));
}

// ============================================================================
// RATIONAL DISPLAY AND CONVERSION TESTS
// ============================================================================

#[test]
fn test_rational_display_fraction() {
    let r = Rational::new(3, 4);
    assert_eq!(format!("{}", r), "3/4");
}

#[test]
fn test_rational_display_whole_number() {
    let r = Rational::new(6, 3);
    assert_eq!(format!("{}", r), "2");
}

#[test]
fn test_rational_to_f64() {
    let r = Rational::new(1, 4);
    assert!((r.to_f64() - 0.25).abs() < f64::EPSILON);
}

#[test]
fn test_rational_to_f64_repeating() {
    let r = Rational::new(1, 3);
    assert!((r.to_f64() - 0.333333333).abs() < 0.0001);
}

// ============================================================================
// RATIONAL REFERENCE OPERATOR TESTS
// ============================================================================

#[test]
fn test_rational_reference_add() {
    let a = Rational::new(1, 2);
    let b = Rational::new(1, 3);
    let c = &a + &b;
    // a and b are still usable
    assert_eq!(c, Rational::new(5, 6));
    assert_eq!(a, Rational::new(1, 2));
    assert_eq!(b, Rational::new(1, 3));
}

// ============================================================================
// COMPLEX NUMBER CREATION TESTS
// ============================================================================

#[test]
fn test_complex_new() {
    let c = Complex::new(3.0, 4.0);
    assert_eq!(c.real, 3.0);
    assert_eq!(c.imag, 4.0);
}

#[test]
fn test_complex_purely_real() {
    let c = Complex::new(5.0, 0.0);
    assert_eq!(c.real, 5.0);
    assert_eq!(c.imag, 0.0);
}

#[test]
fn test_complex_purely_imaginary() {
    let c = Complex::new(0.0, 3.0);
    assert_eq!(c.real, 0.0);
    assert_eq!(c.imag, 3.0);
}

// ============================================================================
// COMPLEX ARITHMETIC TESTS
// ============================================================================

#[test]
fn test_complex_add() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let result = a + b;
    assert_eq!(result, Complex::new(4.0, 6.0));
}

#[test]
fn test_complex_sub() {
    let a = Complex::new(5.0, 7.0);
    let b = Complex::new(3.0, 4.0);
    let result = a - b;
    assert_eq!(result, Complex::new(2.0, 3.0));
}

#[test]
fn test_complex_mul() {
    // (1 + 2i)(3 + 4i) = (3-8) + (4+6)i = -5 + 10i
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let result = a * b;
    assert!((result.real - (-5.0)).abs() < f64::EPSILON);
    assert!((result.imag - 10.0).abs() < f64::EPSILON);
}

#[test]
fn test_complex_div() {
    // (1 + 2i) / (3 + 4i) = (3+8)/(9+16) + (6-4)/(9+16)i = 11/25 + 2/25i
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let result = a / b;
    assert!((result.real - 0.44).abs() < 0.001);
    assert!((result.imag - 0.08).abs() < 0.001);
}

#[test]
fn test_complex_neg() {
    let a = Complex::new(3.0, -4.0);
    let result = -a;
    assert_eq!(result, Complex::new(-3.0, 4.0));
}

// ============================================================================
// COMPLEX MAGNITUDE AND CONJUGATE TESTS
// ============================================================================

#[test]
fn test_complex_magnitude_3_4() {
    let c = Complex::new(3.0, 4.0);
    assert!((c.magnitude() - 5.0).abs() < f64::EPSILON);
}

#[test]
fn test_complex_magnitude_zero() {
    let c = Complex::new(0.0, 0.0);
    assert!((c.magnitude() - 0.0).abs() < f64::EPSILON);
}

#[test]
fn test_complex_magnitude_purely_real() {
    let c = Complex::new(5.0, 0.0);
    assert!((c.magnitude() - 5.0).abs() < f64::EPSILON);
}

#[test]
fn test_complex_conjugate() {
    let c = Complex::new(3.0, 4.0);
    let conj = c.conjugate();
    assert_eq!(conj, Complex::new(3.0, -4.0));
}

#[test]
fn test_complex_conjugate_negative_imag() {
    let c = Complex::new(1.0, -2.0);
    let conj = c.conjugate();
    assert_eq!(conj, Complex::new(1.0, 2.0));
}

#[test]
fn test_complex_multiply_by_conjugate() {
    // z * conj(z) = |z|^2 (real number)
    let z = Complex::new(3.0, 4.0);
    let result = z * z.conjugate();
    assert!((result.real - 25.0).abs() < f64::EPSILON);
    assert!(result.imag.abs() < f64::EPSILON);
}

// ============================================================================
// COMPLEX COMPOUND ASSIGNMENT TESTS
// ============================================================================

#[test]
fn test_complex_add_assign() {
    let mut z = Complex::new(1.0, 2.0);
    z += Complex::new(3.0, 4.0);
    assert_eq!(z, Complex::new(4.0, 6.0));
}

#[test]
fn test_complex_sub_assign() {
    let mut z = Complex::new(5.0, 7.0);
    z -= Complex::new(3.0, 4.0);
    assert_eq!(z, Complex::new(2.0, 3.0));
}

#[test]
fn test_complex_mul_assign() {
    let mut z = Complex::new(1.0, 0.0);
    z *= Complex::new(0.0, 1.0); // multiply by i
    // 1 * i = i (rotation by 90 degrees)
    assert!(z.real.abs() < f64::EPSILON);
    assert!((z.imag - 1.0).abs() < f64::EPSILON);
}

#[test]
fn test_complex_div_assign() {
    let mut z = Complex::new(4.0, 6.0);
    z /= Complex::new(2.0, 0.0);
    assert_eq!(z, Complex::new(2.0, 3.0));
}

// ============================================================================
// COMPLEX DISPLAY TESTS
// ============================================================================

#[test]
fn test_complex_display_positive_imag() {
    let c = Complex::new(3.0, 4.0);
    assert_eq!(format!("{}", c), "3 + 4i");
}

#[test]
fn test_complex_display_negative_imag() {
    let c = Complex::new(3.0, -4.0);
    assert_eq!(format!("{}", c), "3 - 4i");
}

// ============================================================================
// COMPLEX REFERENCE OPERATOR TESTS
// ============================================================================

#[test]
fn test_complex_reference_add() {
    let a = Complex::new(1.0, 1.0);
    let b = Complex::new(2.0, 2.0);
    let c = &a + &b;
    // a and b are still usable
    assert_eq!(c, Complex::new(3.0, 3.0));
    assert_eq!(a, Complex::new(1.0, 1.0));
}

// ============================================================================
// GCD TESTS
// ============================================================================

#[test]
fn test_gcd_basic() {
    assert_eq!(gcd(12, 8), 4);
}

#[test]
fn test_gcd_coprime() {
    assert_eq!(gcd(7, 13), 1);
}

#[test]
fn test_gcd_same() {
    assert_eq!(gcd(5, 5), 5);
}

#[test]
fn test_gcd_one_is_zero() {
    assert_eq!(gcd(0, 5), 5);
    assert_eq!(gcd(5, 0), 5);
}

#[test]
fn test_gcd_large() {
    assert_eq!(gcd(100, 75), 25);
}

// ============================================================================
// COMPLEX MATHEMATICAL PROPERTIES
// ============================================================================

#[test]
fn test_complex_i_squared_is_negative_one() {
    // i^2 = -1
    let i = Complex::new(0.0, 1.0);
    let result = i * i;
    assert!((result.real - (-1.0)).abs() < f64::EPSILON);
    assert!(result.imag.abs() < f64::EPSILON);
}

#[test]
fn test_complex_add_commutativity() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let ab = a + b;
    let ba = b + a;
    assert!((ab.real - ba.real).abs() < f64::EPSILON);
    assert!((ab.imag - ba.imag).abs() < f64::EPSILON);
}

#[test]
fn test_complex_mul_commutativity() {
    let a = Complex::new(1.0, 2.0);
    let b = Complex::new(3.0, 4.0);
    let ab = a * b;
    let ba = b * a;
    assert!((ab.real - ba.real).abs() < f64::EPSILON);
    assert!((ab.imag - ba.imag).abs() < f64::EPSILON);
}

#[test]
fn test_complex_additive_identity() {
    let a = Complex::new(3.0, 4.0);
    let zero = Complex::new(0.0, 0.0);
    let result = a + zero;
    assert_eq!(result, a);
}

#[test]
fn test_complex_multiplicative_identity() {
    let a = Complex::new(3.0, 4.0);
    let one = Complex::new(1.0, 0.0);
    let result = a * one;
    assert!((result.real - a.real).abs() < f64::EPSILON);
    assert!((result.imag - a.imag).abs() < f64::EPSILON);
}
