use std::{self, fmt::Display};

use crate::error::SchemeError;

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Rational {
    pub(crate) numerator: f32,
    pub(crate) denominator: f32,
}

impl Rational {
    pub(crate) fn eval(&self) -> f32 {
        self.numerator / self.denominator
    }

    pub(crate) fn new(n: f32, d: f32) -> Result<Self, SchemeError> {
        if n == 0.0 {
            return Ok(Self {
                numerator: 0.0,
                denominator: 1.0,
            });
        }
        if d == 0.0 {
            return Err(SchemeError::new("Divide by zero".to_string()));
        }

        let factor = Self::gcd(n, d);
        Ok(Self {
            numerator: n / factor,
            denominator: d / factor,
        })
    }

    pub(crate) fn gcd(mut a: f32, mut b: f32) -> f32 {
        loop {
            let c = b % a;
            if c == 0.0 {
                return a;
            }
            b = a;
            a = c;
        }
    }

    pub(crate) fn add(&self, other: &Self) -> Result<Self, SchemeError> {
        let (mut n1, d1) = (self.numerator, self.denominator);
        let (mut n2, d2) = (other.numerator, other.denominator);
        let cd = d1 * d2;
        n1 = n1 * d2;
        n2 = n2 * d1;
        Self::new(n1 + n2, cd)
    }

    pub(crate) fn sub(&self, other: &Self) -> Result<Self, SchemeError> {
        let (mut n1, d1) = (self.numerator, self.denominator);
        let (mut n2, d2) = (other.numerator, other.denominator);
        let cd = d1 * d2;
        n1 = n1 * d2;
        n2 = n2 * d1;
        Self::new(n1 - n2, cd)
    }

    pub(crate) fn mul(&self, other: &Self) -> Result<Self, SchemeError> {
        let (n1, d1) = (self.numerator, self.denominator);
        let (n2, d2) = (other.numerator, other.denominator);
        Self::new(n1 * n2, d1 * d2)
    }

    pub(crate) fn div(&self, other: &Self) -> Result<Self, SchemeError> {
        let (n1, d1) = (self.numerator, self.denominator);
        let (n2, d2) = (other.numerator, other.denominator);
        Self::new(n1 * d2, n2 * d1)
    }
}

impl From<f32> for Rational {
    fn from(val: f32) -> Self {
        Self {
            numerator: val,
            denominator: 1.0,
        }
    }
}

impl Display for Rational {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.eval())
    }
}
