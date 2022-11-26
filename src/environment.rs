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

    pub(crate) fn get(&self, name: &str) -> Option<&Atom> {
        self.table.get(name)
    }

    pub(crate) fn set(&mut self, name: &str, val: &Atom) {
        self.table.insert(name.to_string(), val.clone());
    }
}

fn add(args: Vec<Atom>, env: &mut Env) -> Result<Atom, EvalError> {
    let mut result = Rational::from(0.0);
    for item in args {
        match item {
            Atom::Number(val) => result = result.add(&val),
            _ => return Err(EvalError(format!("Expected a number, found {:?}", item))),
        };
    }
    Ok(Atom::Number(result))
}

fn sub(args: Vec<Atom>, env: &mut Env) -> Result<Atom, EvalError> {
    let mut result;
    if let Some(val) = args.get(0) {
        result = match val {
            Atom::Number(num) => num.clone(),
            _ => return Err(EvalError(format!("Expected a number, found {:?}", val))),
        }
    } else {
        return Err(EvalError(
            "- expects at least one argument, found none".to_string(),
        ));
    }

    if args.len() == 1 {
        return Ok(Atom::Number(result.mul(&Rational::from(-1.0))));
    }

    for item in args[1..].iter() {
        match item {
            Atom::Number(val) => result = result.sub(&val),
            _ => return Err(EvalError(format!("Expected a number, found {:?}", item))),
        };
    }
    Ok(Atom::Number(result))
}

fn mul(args: Vec<Atom>, env: &mut Env) -> Result<Atom, EvalError> {
    let mut result = Rational::from(1.0);
    for item in args {
        match item {
            Atom::Number(val) => result = result.mul(&val),
            _ => return Err(EvalError(format!("Expected a number, found {:?}", item))),
        }
    }
    Ok(Atom::Number(result))
}

fn div(args: Vec<Atom>, env: &mut Env) -> Result<Atom, EvalError> {
    if args.len() != 2 {
        return Err(EvalError(format!(
            "/ takes 2 arguments, found {}",
            args.len()
        )));
    }
    let mut args = args.iter();
    let num = args.next().unwrap();
    let den = args.next().unwrap();
    match (num, den) {
        (Atom::Number(n), Atom::Number(d)) => Ok(Atom::Number(n.div(&d))),
        _ => Err(EvalError(format!(
            "Expected two numbers, found {} and {}",
            num, den
        ))),
    }
}
