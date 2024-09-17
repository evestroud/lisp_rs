use crate::{environment::FrameRef, error::SchemeError, types::Cell};

pub fn evaluate(ast: &Cell, env: &mut FrameRef) -> Result<Cell, SchemeError> {
    println!("EVAL: {}", ast);
    match ast {
        Cell::Pair(car, cdr) => apply(car, cdr, env),
        Cell::Symbol(s) => env.borrow().get(s),
        Cell::Quote(q) => Ok((**q).clone()),
        Cell::Nil | Cell::Number(_) | Cell::Boolean(_) | Cell::Function(_) => Ok(ast.clone()),
    }
}

fn apply(operator: &Cell, operands: &Cell, env: &mut FrameRef) -> Result<Cell, SchemeError> {
    println!("APPLY: operator: {}, operands: {}", operator, operands);
    if let Cell::Function(mut f) = evaluate(operator, env)? {
        let resolved_operands: Result<_, _> =
            operands.iter().map(|item| evaluate(item, env)).collect();
        f.call(&resolved_operands?)
    } else {
        Err(SchemeError::new("APPLY: Expected a function".to_string()))
    }
}
