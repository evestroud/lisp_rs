use crate::types::SchemeError;
use std::fmt::Debug;
use std::{cell::RefCell, fmt::Display, rc::Rc};

use crate::environment::Env;

use super::Exp;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Function {
    Builtin(Builtin),
    // Lambda: Lambda
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        todo!()
    }
}

#[derive(Clone)]
pub(crate) struct Builtin {
    pub(crate) func: &'static dyn Fn(Exp, &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError>,
    pub(crate) name: String,
}

impl Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Builtin '{}", self.name)
    }
}

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
        self.name == other.name
    }
}
