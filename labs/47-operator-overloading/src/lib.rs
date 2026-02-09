//! # Operator Overloading - Student API

use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct Rational {
    pub numerator: i64,
    pub denominator: i64,
}

impl Rational {
    pub fn new(_numerator: i64, _denominator: i64) -> Self {
        todo!("Construct and normalize a Rational")
    }

    pub fn to_f64(&self) -> f64 {
        todo!("Convert Rational to f64")
    }
}

pub fn gcd(_a: i64, _b: i64) -> i64 {
    todo!("Compute greatest common divisor")
}

impl fmt::Display for Rational {
    fn fmt(&self, _f: &mut fmt::Formatter) -> fmt::Result {
        todo!("Format Rational for display")
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Complex {
    pub real: f64,
    pub imag: f64,
}

impl Complex {
    pub fn new(_real: f64, _imag: f64) -> Self {
        todo!("Construct a Complex number")
    }

    pub fn magnitude(&self) -> f64 {
        todo!("Compute complex magnitude")
    }

    pub fn conjugate(&self) -> Complex {
        todo!("Compute complex conjugate")
    }
}

#[doc(hidden)]
pub mod solution;
