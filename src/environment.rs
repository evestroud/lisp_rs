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
            table: HashMap::from([("+".to_string(), Atom::Builtin(Rc::from(Builtin(&add))))]),
            parent: None,
        }
    }

    pub(crate) fn lookup(&self, name: &str) -> Option<&Atom> {
        self.table.get(name)
    }
}

fn add(args: Vec<Atom>) -> Result<Atom, EvalError> {
    let mut result = 0;
    for item in args {
        match item {
            Atom::Int(val) => result += val,
            _ => todo!(),
        }
    }
    Atom::Int(result)
}
