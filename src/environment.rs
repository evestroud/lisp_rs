use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::{atom::Atom, builtin::builtins_map, lib::SchemeError};

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Env {
    pub(crate) table: HashMap<String, Atom>,
    parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub(crate) fn new() -> Self {
        Self {
            table: builtins_map(),
            parent: None,
        }
    }

    pub(crate) fn get(&self, name: &str) -> Result<Atom, SchemeError> {
        if let Some(val) = self.table.get(name) {
            return Ok(val.clone());
        }
        if let Some(parent) = &self.parent {
            if let Ok(val) = parent.borrow().get(name) {
                return Ok(val.clone());
            }
        }
        Err(SchemeError(format!("Name {} not found", name)))
    }

    pub(crate) fn set(&mut self, name: &str, val: &Atom) {
        self.table.insert(name.to_string(), val.clone());
    }
}

pub(crate) fn create_closure<'a>(parent: Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
    Rc::new(RefCell::new(Env {
        table: HashMap::new(),
        parent: Some(parent),
    }))
}
