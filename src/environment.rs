use std::{collections::HashMap, rc::Rc};

use crate::{atom::Atom, builtin, evaluator::EvalError};

pub(crate) struct Env {
    pub(crate) table: HashMap<String, Atom>,
    parent: Option<Rc<Env>>,
}

impl Env {
    pub(crate) fn new() -> Self {
        Self {
            table: builtin::builtins_map(),
            parent: None,
        }
    }

    pub(crate) fn get(&self, name: &str) -> Result<&Atom, EvalError> {
        if let Some(val) = self.table.get(name) {
            return Ok(val);
        }
        if let Some(parent) = &self.parent {
            return parent.get(name);
        }
        Err(EvalError(format!("Name {} not found", name)))
    }

    pub(crate) fn set(&mut self, name: &str, val: &Atom) {
        self.table.insert(name.to_string(), val.clone());
    }

    pub(crate) fn create_closure(self) -> Self {
        Self {
            table: HashMap::new(),
            parent: Some(Rc::new(self)),
        }
    }
}
