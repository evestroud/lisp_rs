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
            Atom::Quote(exp) => {
                // ? not sure if this is correct
                if let Exp::Literal(val) = *exp.clone() {
                    return Ok(val.clone());
                }
                Ok(atom.clone())
            }
            _ => Ok(atom.clone()),
        },
    }
}

fn apply(list: &Vec<Exp>, env: &mut Env) -> Result<Atom, EvalError> {
    let mut list_iter = list.iter();
    let first = evaluate(
        list_iter.next().unwrap_or_else(|| &Exp::Literal(Atom::Nil)),
        env,
    )?;
    match first {
        Atom::Builtin(f) => {
            let rest_results = list_iter.map(|exp| evaluate(exp, env));
            let mut rest = Vec::<Atom>::new();
            for res in rest_results {
                rest.push(res?);
            }
            f.0(rest, env)
        }
        Atom::SpecialForm(form) => match form.as_str() {
            _ => Err(EvalError(format!("Invalid special form {}", form))),
        },
        Atom::Nil => return Ok(Atom::Nil),
        _ => Err(EvalError(format!("Expected a symbol, found {:?}", first))),
    }
}
