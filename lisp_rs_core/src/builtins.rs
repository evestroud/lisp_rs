use crate::{
    error::SchemeError,
    types::{
        functions::{Builtin, Function},
        Cell,
    },
};
use std::collections::HashMap;

pub fn generate_builtins() -> HashMap<String, Cell> {
    HashMap::from([
        ("nil".to_string(), Cell::Nil),
        (
            "+".to_string(),
            Cell::Function(Function::Builtin(Builtin(&add))),
        ),
        (
            "-".to_string(),
            Cell::Function(Function::Builtin(Builtin(&sub))),
        ),
        (
            "*".to_string(),
            Cell::Function(Function::Builtin(Builtin(&mul))),
        ),
        (
            "/".to_string(),
            Cell::Function(Function::Builtin(Builtin(&div))),
        ),
    ])
}

pub fn add(args: &Cell) -> Result<Cell, SchemeError> {
    args.iter()
        .try_fold(Cell::Number(0), |result, item| match (result, item) {
            (Cell::Number(x), Cell::Number(y)) => Ok(Cell::Number(x + y)),
            _ => Err(SchemeError::new(format!("Expected number, found {}", item))),
        })
}

pub fn sub(args: &Cell) -> Result<Cell, SchemeError> {
    validate_num_args(args, "sub", Arity::AtLeast(1))?;
    args.iter().try_fold(
        // Simple inversion for unary application
        if args.len() == 1 {
            Cell::Number(0)
        } else {
            Cell::Nil
        },
        |result, item| match (result, item) {
            (Cell::Nil, Cell::Number(n)) => Ok(Cell::Number(*n)),
            (Cell::Number(x), Cell::Number(y)) => Ok(Cell::Number(x - y)),
            _ => Err(SchemeError::new(format!("Expected number, found {}", item))),
        },
    )
}

pub fn mul(args: &Cell) -> Result<Cell, SchemeError> {
    args.iter()
        .try_fold(Cell::Number(1), |result, item| match (result, item) {
            (Cell::Number(x), Cell::Number(y)) => Ok(Cell::Number(x * y)),
            _ => Err(SchemeError::new(format!("Expected number, found {}", item))),
        })
}

pub fn div(args: &Cell) -> Result<Cell, SchemeError> {
    validate_num_args(args, "div", Arity::AtLeast(1))?;
    args.iter().try_fold(
        // Simple inversion for unary application
        if args.len() == 1 {
            Cell::Number(1)
        } else {
            Cell::Nil
        },
        |result, item| match (result, item) {
            (Cell::Nil, Cell::Number(n)) => Ok(Cell::Number(*n)),
            (Cell::Number(x), Cell::Number(y)) => Ok(Cell::Number(x / y)),
            _ => Err(SchemeError::new(format!("Expected number, found {}", item))),
        },
    )
}

enum Arity {
    Exact(usize),
    AtLeast(usize),
    AtMost(usize),
    Range(usize, usize),
}

fn validate_num_args(args: &Cell, name: &str, arity: Arity) -> Result<(), SchemeError> {
    match arity {
        Arity::Exact(n) => {
            if args.len() != n {
                return Err(SchemeError::new(format!(
                    "Function {} expects at exactly {} arguments, found {}.",
                    name,
                    n,
                    args.len()
                )));
            }
        }
        Arity::AtLeast(n) => {
            if args.len() < n {
                return Err(SchemeError::new(format!(
                    "Function {} expects at least {} arguments, found {}.",
                    name,
                    n,
                    args.len()
                )));
            }
        }
        Arity::AtMost(n) => {
            if args.len() > n {
                return Err(SchemeError::new(format!(
                    "Function {} expects at most {} arguments, found {}.",
                    name,
                    n,
                    args.len()
                )));
            }
        }
        Arity::Range(min, max) => {
            if args.len() < min || args.len() > max {
                return Err(SchemeError::new(format!(
                    "Function {} expects at least {} and at most {} arguments, found {}.",
                    name,
                    min,
                    max,
                    args.len()
                )));
            }
        }
    }
    Ok(())
}
