use core::fmt;

use crate::{atom::Atom, environment::Env, parser::Exp};

#[derive(Debug, Clone)]
pub(crate) struct EvalError(pub(crate) String);

impl fmt::Display for EvalError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

pub(crate) fn evaluate(input: &Exp, env: &mut Env) -> Result<Atom, EvalError> {
    match input {
        Exp::SubExp(list) => apply(list, env),
        Exp::Literal(atom) => match atom {
            Atom::Symbol(symbol) => {
                if let Some(val) = env.get(&symbol) {
                    Ok(val.clone())
                } else {
                    Err(EvalError(format!("Symbol {} not found", symbol)))
                }
            }
            _ => Ok(atom.clone()),
        },
    }
}

fn apply(list: &Vec<Exp>, env: &mut Env) -> Result<Atom, EvalError> {
    let first = evaluate(
        list.first().unwrap_or_else(|| &Exp::Literal(Atom::Nil)),
        env,
    )?;
    let operation;
    match first {
        Atom::Builtin(f) => operation = f,
        Atom::Nil => return Ok(Atom::Nil),
        _ => return Err(EvalError(format!("Expected a symbol, found {:?}", first))),
    };
    let rest_results = list[1..].iter().map(|exp| evaluate(exp, env));
    let mut rest = Vec::<Atom>::new();
    for res in rest_results {
        rest.push(res?);
    }
    operation.0(rest, env)
}
