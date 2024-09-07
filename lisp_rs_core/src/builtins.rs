use crate::{
    environment::FrameRef,
    error::SchemeError,
    types::{functions::Builtin, Cell},
};
use std::collections::HashMap;

pub fn generate_builtins() -> HashMap<String, Cell> {
    HashMap::from([
        ("nil".to_string(), Cell::Nil),
        ("+".to_string(), Cell::Builtin(Builtin(&add))),
        ("-".to_string(), Cell::Builtin(Builtin(&sub))),
        ("*".to_string(), Cell::Builtin(Builtin(&mul))),
        ("/".to_string(), Cell::Builtin(Builtin(&div))),
    ])
}

pub fn add(args: &Cell, _env: &mut FrameRef) -> Result<Cell, SchemeError> {
    let mut sum = 0;
    for a in args {
        if let Cell::Number(n) = a {
            sum += n;
        } else {
            return Err(SchemeError::new(format!("Expected number, found {}", a)));
        }
    }
    Ok(Cell::Number(sum))
}

pub fn sub(args: &Cell, _env: &mut FrameRef) -> Result<Cell, SchemeError> {
    validate_num_args(args, "sub", Arity::AtLeast(2))?;
    let mut difference = 0;
    for a in args {
        if let Cell::Number(n) = a {
            difference -= n;
        } else {
            return Err(SchemeError::new(format!("Expected number, found {}", a)));
        }
    }
    Ok(Cell::Number(difference))
}

pub fn mul(args: &Cell, _env: &mut FrameRef) -> Result<Cell, SchemeError> {
    let mut product = 1;
    for a in args {
        if let Cell::Number(n) = a {
            product *= n;
        } else {
            return Err(SchemeError::new(format!("Expected number, found {}", a)));
        }
    }
    Ok(Cell::Number(product))
}

pub fn div(args: &Cell, _env: &mut FrameRef) -> Result<Cell, SchemeError> {
    validate_num_args(args, "div", Arity::AtLeast(2))?;
    let mut quotient = 1;
    for a in args {
        if let Cell::Number(n) = a {
            quotient *= n;
        } else {
            return Err(SchemeError::new(format!("Expected number, found {}", a)));
        }
    }
    Ok(Cell::Number(quotient))
}

enum Arity {
    Zero,
    AtLeast(usize),
    AtMost(usize),
    Range(usize, usize),
}

fn validate_num_args(args: &Cell, name: &str, arity: Arity) -> Result<(), SchemeError> {
    match arity {
        Arity::Zero => {
            if !args.is_empty() {
                return Err(SchemeError::new(format!(
                    "Function {} expects no arguments, found {}.",
                    name,
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
