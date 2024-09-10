use crate::{environment::FrameRef, error::SchemeError, types::Cell};

pub fn evaluate(ast: &Cell, env: &mut FrameRef) -> Result<Cell, SchemeError> {
    println!("EVAL: {}", ast);
    match ast {
        Cell::Pair(car, cdr) => apply(&evaluate(car, env)?, cdr, env),
        Cell::Symbol(s) => env.borrow().get(s),
        Cell::Quote(q) => Ok((**q).clone()),
        Cell::Nil | Cell::Number(_) | Cell::Boolean(_) | Cell::Builtin(_) | Cell::Lambda(_) => {
            Ok(ast.clone())
        }
    }
}

fn apply(operator: &Cell, operands: &Cell, env: &mut FrameRef) -> Result<Cell, SchemeError> {
    // TODO Lambdas
    println!("APPLY: operator: {}, operands: {}", operator, operands);
    match evaluate(operator, env)? {
        Cell::Builtin(f) => Ok(f.call(operands)?),
        Cell::Lambda(mut l) => Ok(l.call(operands)?),
        _ => Err(SchemeError::new("APPLY: Expected a function".to_string())),
    }
}
