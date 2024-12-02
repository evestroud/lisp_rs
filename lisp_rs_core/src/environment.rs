use crate::{
    builtins,
    error::SchemeError,
    types::{
        functions::{Builtin, Function},
        Cell,
    },
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Clone, PartialEq)]
pub struct Frame {
    pub table: HashMap<String, Cell>,
    parent: Option<Rc<RefCell<Frame>>>,
}

pub type FrameRef = Rc<RefCell<Frame>>;

pub fn new_environment() -> FrameRef {
    let mut env = Rc::new(RefCell::new(Frame {
        table: HashMap::new(),
        parent: None,
    }));
    builtins::builtins::load_builtins(&mut env);
    env
}

impl Frame {
    pub fn get(&self, name: &str) -> Result<Cell, SchemeError> {
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

    pub fn set(&mut self, name: &str, val: &Cell) {
        self.table.insert(name.to_string(), val.clone());
    }

    pub fn load_builtins(&mut self, builtins: Vec<Builtin>) {
        builtins.into_iter().for_each(|builtin| {
            if self.table.contains_key(builtin.name) {
                panic!("error loading builtins: duplicate entry {}", builtin.name)
            }
            self.table.insert(
                builtin.name.to_string(),
                Cell::Function(Function::Builtin(builtin)),
            );
        })
    }
}

impl std::fmt::Debug for Frame {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{:?}, parent: {:?}", self.table.keys(), self.parent)
    }
}

pub fn create_closure(parent: Rc<RefCell<Frame>>) -> Rc<RefCell<Frame>> {
    Rc::new(RefCell::new(Frame {
        table: HashMap::new(),
        parent: Some(parent),
    }))
}
