use std::fmt::{Debug, Display};

use crate::{
    environment::{create_closure, FrameRef},
    error::SchemeError,
    evaluator::evaluate,
};

use super::Cell;

#[derive(Clone, Debug, PartialEq)]
pub enum Function {
    Builtin(Builtin),
    Lambda(Lambda),
}

impl Function {
    pub fn call(&mut self, args: &Cell) -> Result<Cell, SchemeError> {
        match self {
            Function::Builtin(b) => b.call(args),
            Function::Lambda(l) => l.call(args),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Function::Builtin(b) => b.to_string(),
                Function::Lambda(l) => l.to_string(),
            }
        )
    }
}

#[derive(Clone)]
pub struct Builtin(pub &'static dyn Fn(&Cell) -> Result<Cell, SchemeError>);

impl Builtin {
    pub fn call(&self, args: &Cell) -> Result<Cell, SchemeError> {
        (self.0)(args)
    }
}

impl Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Builtin function")
    }
}

impl Debug for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Builtin function")
    }
}

impl PartialEq for Builtin {
    fn eq(&self, other: &Self) -> bool {
        std::ptr::eq(self.0, other.0)
    }
}

#[derive(PartialEq, Debug, Clone)]
pub struct Lambda {
    parameters: Vec<String>,
    body: Box<Cell>,
    env: FrameRef,
}

impl Lambda {
    pub fn new(parameters: Vec<String>, body: Box<Cell>, env: FrameRef) -> Self {
        Self {
            parameters,
            body,
            env: create_closure(env),
        }
    }

    pub fn call(&mut self, arguments: &Cell) -> Result<Cell, SchemeError> {
        if self.parameters.len() != arguments.len() {
            return Err(SchemeError::new(format!(
                "LAMBDA: Expected {} arguments, found {}",
                self.parameters.len(),
                arguments.len()
            )));
        }

        for (param, arg) in std::iter::zip(self.parameters.iter(), arguments.iter()) {
            self.env.borrow_mut().set(param, arg);
        }
        evaluate(&self.body, &mut self.env)
    }
}

impl Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "(lambda ({:?}))", self.parameters)
    }
}
