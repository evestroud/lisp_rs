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
