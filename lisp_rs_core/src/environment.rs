use crate::{builtins::generate_builtins, error::SchemeError, types::Cell};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, PartialEq)]
pub struct Frame {
    pub(crate) table: HashMap<String, Cell>,
    parent: Option<Rc<RefCell<Frame>>>,
}

pub type FrameRef = Rc<RefCell<Frame>>;

pub fn new_environment() -> FrameRef {
    Rc::new(RefCell::new(Frame {
        table: generate_builtins(),
        parent: None,
    }))
}

impl Frame {
    pub(crate) fn get(&self, name: &str) -> Result<Cell, SchemeError> {
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

    pub(crate) fn set(&mut self, name: &str, val: &Cell) {
        self.table.insert(name.to_string(), val.clone());
    }
}

impl std::fmt::Debug for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, parent: {:?}", self.table.keys(), self.parent)
    }
}

pub(crate) fn create_closure(parent: Rc<RefCell<Frame>>) -> Rc<RefCell<Frame>> {
    Rc::new(RefCell::new(Frame {
        table: HashMap::new(),
        parent: Some(parent),
    }))
}
