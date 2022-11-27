use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::{builtin::Builtin, parser::Exp};

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Atom {
    Number(Rational),
    Symbol(String),
    Nil,
    Builtin(Rc<Builtin>),
    SpecialForm(String),
    Quote(Box<Exp>),
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Atom::Number(n) => n.to_string(),
            Atom::Symbol(s) => s.to_string(),
            Atom::Nil => "Nil".to_string(),
            Atom::Builtin(_) => "Builtin".to_string(), // TODO
            Atom::SpecialForm(f) => f.to_string(),
            Atom::Quote(exp) => format!("'{}", exp),
        };
        write!(f, "{}", val)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Rational {
    pub(crate) numerator: f32,
    pub(crate) denominator: f32,
}

impl Rational {
    pub(crate) fn eval(&self) -> f32 {
        self.numerator / self.denominator
    }

    fn new(n: f32, d: f32) -> Self {
        let factor = Self::gcd(n, d);
        Self {
            numerator: n / factor,
            denominator: d / factor,
        }
    }

    fn gcd(a: f32, b: f32) -> f32 {
        if a == 0.0 {
            return b;
        }
        Self::gcd(b % a, a)
    }

    pub(crate) fn add(&self, other: &Self) -> Self {
        let (mut n1, d1) = (self.numerator, self.denominator);
        let (mut n2, d2) = (other.numerator, other.denominator);
        let cd = d1 * d2;
        n1 = n1 * d2;
        n2 = n2 * d1;
        Self::new(n1 + n2, cd)
    }

    pub(crate) fn sub(&self, other: &Self) -> Self {
        let (mut n1, d1) = (self.numerator, self.denominator);
        let (mut n2, d2) = (other.numerator, other.denominator);
        let cd = d1 * d2;
        n1 = n1 * d2;
        n2 = n2 * d1;
        Self::new(n1 - n2, cd)
    }

    pub(crate) fn mul(&self, other: &Self) -> Self {
        let (n1, d1) = (self.numerator, self.denominator);
        let (n2, d2) = (other.numerator, other.denominator);
        Self::new(n1 * n2, d1 * d2)
    }

    pub(crate) fn div(&self, other: &Self) -> Self {
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
