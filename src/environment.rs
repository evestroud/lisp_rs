use std::collections::HashMap;

use crate::atom::{Atom, Builtin};

pub(crate) struct Env {
    pub(crate) table: HashMap<String, Atom>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub(crate) fn new() -> Self {
        Self {
            table: HashMap::from([("+".to_string(), Atom::Builtin(Builtin(&add)))]),
            parent: None,
        }
    }

    pub(crate) fn lookup(&self, name: &str) -> Option<&Atom> {
        self.table.get(name)
    }
}

fn add(args: Vec<Atom>) -> Atom {
    let mut result = 0;
    for item in args {
        match item {
            Atom::Int(val) => result += val,
            _ => todo!(),
        }
    }
    Atom::Int(result)
}