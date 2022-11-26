use std::{
    fmt::{Debug, Display},
    rc::Rc,
};

use crate::evaluator::EvalError;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Atom {
    Number(Rational),
    Symbol(String),
    Nil,
    Builtin(Rc<Builtin>),
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Atom::Number(n) => n.to_string(),
            Atom::Symbol(s) => s.to_string(),
            Atom::Nil => todo!(),
            Atom::Builtin(_) => "Builtin".to_string(),
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
    pub(crate) fn unwrap(&self) -> f32 {
        self.numerator / self.denominator
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
        write!(f, "{}", self.numerator / self.denominator)
    }
}

pub(crate) struct Builtin(pub(crate) &'static dyn Fn(Vec<Atom>) -> Result<Atom, EvalError>);

impl Debug for Builtin {
    // https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Builtin function")
    }
}

impl PartialEq for Builtin {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, other: &Self) -> bool {
        todo!()
    }
}
