use std::{collections::HashMap, rc::Rc};

use crate::{
    atom::{Atom, Builtin, Rational},
    evaluator::EvalError,
};

pub(crate) struct Env {
    pub(crate) table: HashMap<String, Atom>,
    parent: Option<Box<Env>>,
}

impl Env {
    pub(crate) fn new() -> Self {
        Self {
            table: HashMap::from([
                ("+".to_string(), Atom::Builtin(Rc::from(Builtin(&add)))),
                ("-".to_string(), Atom::Builtin(Rc::from(Builtin(&sub)))),
                ("*".to_string(), Atom::Builtin(Rc::from(Builtin(&mul)))),
                ("/".to_string(), Atom::Builtin(Rc::from(Builtin(&div)))),
            ]),
            parent: None,
        }
    }

    pub(crate) fn lookup(&self, name: &str) -> Option<&Atom> {
        self.table.get(name)
    }
}

fn add(args: Vec<Atom>) -> Result<Atom, EvalError> {
    let mut result = 0.0;
    for item in args {
        match item {
            Atom::Number(val) => result += val.eval(),
            _ => return Err(EvalError(format!("Expected a number, found {:?}", item))),
        }
    }
    Ok(Atom::Number(Rational::from(result)))
}

fn sub(args: Vec<Atom>) -> Result<Atom, EvalError> {
    let mut result;
    match args.get(0) {
        Some(val) => {
            result = match val {
                Atom::Number(num) => num.eval(),
                _ => return Err(EvalError(format!("Expected a number, found {:?}", val))),
            }
        }
        None => {
            return Err(EvalError(
                "- expects at least one argument, found none".to_string(),
            ))
        }
    }

    if args.len() == 1 {
        return Ok(Atom::Number(Rational::from(-result)));
    }

    for item in args[1..].iter() {
        match item {
            Atom::Number(val) => result -= val.eval(),
            _ => return Err(EvalError(format!("Expected a number, found {:?}", item))),
        }
    }
    Ok(Atom::Number(Rational::from(result)))
}

fn mul(args: Vec<Atom>) -> Result<Atom, EvalError> {
    let mut result = 1.0;
    for item in args {
        match item {
            Atom::Number(val) => result *= val.eval(),
            _ => return Err(EvalError(format!("Expected a number, found {:?}", item))),
        }
    }
    Ok(Atom::Number(Rational::from(result)))
}

fn div(args: Vec<Atom>) -> Result<Atom, EvalError> {
    if args.len() != 2 {
        return Err(EvalError(format!(
            "/ takes 2 arguments, found {}",
            args.len()
        )));
    }
    let mut args = args.iter();
    let num = match args.next().unwrap() {
        Atom::Number(val) => val.eval() as f32,
        _ => todo!(),
    };
    let den = match args.next().unwrap() {
        Atom::Number(val) => val.eval() as f32,
        _ => todo!(),
    };
    Ok(Atom::Number(Rational::from(num / den)))
}
