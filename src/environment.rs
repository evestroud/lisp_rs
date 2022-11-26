use std::collections::HashMap;

use crate::{atom::Atom, builtin};

pub(crate) struct Env {
    pub(crate) table: HashMap<String, Atom>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub(crate) fn new() -> Self {
        Self {
            table: builtin::builtins_map(),
            parent: None,
        }
    }

    pub(crate) fn get(&self, name: &str) -> Option<&Atom> {
        self.table.get(name)
    }

    pub(crate) fn set(&mut self, name: &str, val: &Atom) {
        self.table.insert(name.to_string(), val.clone());
    }
}
