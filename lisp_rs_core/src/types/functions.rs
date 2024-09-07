use std::fmt::{Debug, Display};

use crate::{environment::FrameRef, error::SchemeError};

use super::Cell;

#[derive(Clone)]
pub struct Builtin(pub &'static dyn Fn(&Cell, &mut FrameRef) -> Result<Cell, SchemeError>);

impl Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Builtin function")
    }
}

impl Debug for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Builtin function")
    }
}

impl PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}
