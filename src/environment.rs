use std::{collections::HashMap, rc::Rc};

use crate::{
    atom::{Atom, Builtin},
    evaluator::EvalError,
};

pub(crate) struct Env {
    pub(crate) table: HashMap<String, Atom>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub(crate) fn new() -> Self {
        Self {
            table: HashMap::from([
                ("+".to_string(), Atom::Builtin(Rc::from(Builtin(&add)))),
                ("-".to_string(), Atom::Builtin(Rc::from(Builtin(&sub)))),
            ]),
            parent: None,
        }
    }

    pub(crate) fn lookup(&self, name: &str) -> Option<&Atom> {
        self.table.get(name)
    }
}

fn add(args: Vec<Atom>) -> Result<Atom, EvalError> {
    let mut result;
    match args.get(0) {
        Some(val) => {
            result = match val {
                Atom::Int(num) => *num,
                Atom::Float(_) => todo!(),
                _ => return Err(EvalError(format!("Expected a number, found {:?}", val))),
            }
        }
        None => {
            return Err(EvalError(
                "+ expects at least one argument, found none".to_string(),
            ))
        }
    }

    for item in args[1..].iter() {
        match item {
            Atom::Int(val) => result += val,
            _ => todo!(),
        }
    }
    Ok(Atom::Int(result))
}

fn sub(args: Vec<Atom>) -> Result<Atom, EvalError> {
    let mut result;
    match args.get(0) {
        Some(val) => {
            result = match val {
                Atom::Int(num) => *num,
                Atom::Float(_) => todo!(),
                _ => return Err(EvalError(format!("Expected a number, found {:?}", val))),
            }
        }
        None => {
            return Err(EvalError(
                "- expects at least one argument, found none".to_string(),
            ))
        }
    }

    if args.len() == 1 {
        return Ok(Atom::Int(-result));
    }

    for item in args[1..].iter() {
        match item {
            Atom::Int(val) => result -= val,
            _ => todo!(),
        }
    }
    Ok(Atom::Int(result))
}
