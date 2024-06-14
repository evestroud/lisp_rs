use crate::{environment::FrameRef, error::SchemeError, types::Cell};

pub fn evaluate(ast: Cell, env: &mut FrameRef) -> Result<Cell, SchemeError> {
    println!("EVAL: {}", ast);
    match ast {
        Cell::Nil => Ok(ast),
        Cell::Pair(car, cdr) => apply(evaluate(*car, env)?, *cdr, env),
        Cell::Number(_) => Ok(ast),
        Cell::Boolean(_) => Ok(ast),
        Cell::Symbol(s) => env.borrow().get(&s),
        Cell::Quote(q) => Ok(*q),
    }
}

fn apply(operator: Cell, operands: Cell, env: &mut FrameRef) -> Result<Cell, SchemeError> {
    // TODO Environments, Lambdas, Builtins
    println!("APPLY: operator: {}, operands: {}", operator, operands);
    Ok(Cell::Nil)
}
