use std::{cell::RefCell, rc::Rc};

use crate::{environment::Env, error::SchemeError, types::Cell};

pub fn evaluate(ast: Cell, env: &mut Rc<RefCell<Env>>) -> Result<Cell, SchemeError> {
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

fn apply(operator: Cell, operands: Cell, env: &mut Rc<RefCell<Env>>) -> Result<Cell, SchemeError> {
    // TODO Environments, Lambdas, Builtins
    println!("APPLY: operator: {}, operands: {}", operator, operands);
    Ok(Cell::Nil)
}
