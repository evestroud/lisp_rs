use core::fmt;

use crate::{atom::Atom, parser::Exp};

#[derive(Debug, Clone)]
pub(crate) struct EvalError(String);

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) fn evaluate(input: &Exp) -> Result<Atom, EvalError> {
    match input {
        Exp::SubExp(list) => apply(list),
        Exp::Literal(atom) => match atom {
            Atom::Symbol(symbol) => todo!(), // symbol lookup
            _ => Ok(atom.clone()),
        },
    }
}

fn apply(list: &Vec<Exp>) -> Result<Atom, EvalError> {
    let first = evaluate(list.first().unwrap_or_else(|| &Exp::Literal(Atom::Nil)))?;
    let operation;
    match first {
        Atom::Symbol(symbol) => {
            operation = symbol;
        }
        Atom::Nil => return Ok(Atom::Nil),
        _ => return Err(EvalError(format!("Expected a symbol, found {:?}", first))),
    };
    Ok(Atom::Symbol(operation))
}
