use crate::{
    error::SchemeError,
    types::{default_env::builtins_map, Exp},
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, PartialEq)]
pub(crate) struct Env {
    pub(crate) table: HashMap<String, Exp>,
    parent: Option<Rc<RefCell<Env>>>,
}

impl Env {
    pub(crate) fn new() -> Self {
        Self {
            table: builtins_map(),
            parent: None,
        }
    }

    pub(crate) fn get(&self, name: &str) -> Result<Exp, SchemeError> {
        if let Some(val) = self.table.get(name) {
            return Ok(val.clone());
        }
        if let Some(parent) = &self.parent {
            if let Ok(val) = parent.borrow().get(name) {
                return Ok(val.clone());
            }
        }
        Err(SchemeError::new(format!("Name {} not found", name)))
    }

    pub(crate) fn set(&mut self, name: &str, val: &Exp) {
        self.table.insert(name.to_string(), val.clone());
    }
}

impl std::fmt::Debug for Env {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, parent: {:?}", self.table.keys(), self.parent)
    }
}

pub(crate) fn create_closure<'a>(parent: Rc<RefCell<Env>>) -> Rc<RefCell<Env>> {
    Rc::new(RefCell::new(Env {
        table: HashMap::new(),
        parent: Some(parent),
    }))
}
