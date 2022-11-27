use crate::atom::{Atom, Rational};
use crate::environment::Env;
use crate::evaluator::EvalError;
use std::collections::HashMap;
use std::fmt::Debug;
use std::rc::Rc;

pub(crate) struct Builtin(
    pub(crate) &'static dyn Fn(Vec<Atom>, &mut Env) -> Result<Atom, EvalError>,
);

impl Debug for Builtin {
    // https://stackoverflow.com/questions/38088067/equivalent-of-func-or-function-in-rust
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Builtin function")
    }
}

impl PartialEq for Builtin {
    fn ne(&self, other: &Self) -> bool {
        !self.eq(other)
    }

    fn eq(&self, _: &Self) -> bool {
        todo!()
    }
}

pub(crate) fn builtins_map() -> HashMap<String, Atom> {
    HashMap::from([
        ("+".to_string(), Atom::Builtin(Rc::from(Builtin(&add)))),
        ("-".to_string(), Atom::Builtin(Rc::from(Builtin(&sub)))),
        ("*".to_string(), Atom::Builtin(Rc::from(Builtin(&mul)))),
        ("/".to_string(), Atom::Builtin(Rc::from(Builtin(&div)))),
    ])
}

fn validate_num_args(args: &Vec<Atom>, min: usize, max: usize) -> Result<(), EvalError> {
    match args.len() >= min {
        true => Ok(()),
        false => Err(EvalError(format!(
            "Expected at least {} args, found {}",
            min,
            args.len(),
        ))),
    }?;
    if max > min {
        match args.len() <= max {
            true => Ok(()),
            false => Err(EvalError(format!(
                "Procedure takes a maximum of {} args, found {}",
                max,
                args.len(),
            ))),
        }?;
    }
    Ok(())
}

pub(crate) fn add(args: Vec<Atom>, _: &mut Env) -> Result<Atom, EvalError> {
    let mut result = Rational::from(0.0);
    for item in args {
        match item {
            Atom::Number(val) => result = result.add(&val),
            _ => return Err(EvalError(format!("Expected a number, found {:?}", item))),
        };
    }
    Ok(Atom::Number(result))
}

pub(crate) fn sub(args: Vec<Atom>, _: &mut Env) -> Result<Atom, EvalError> {
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

pub(crate) fn mul(args: Vec<Atom>, _: &mut Env) -> Result<Atom, EvalError> {
    let mut result = Rational::from(1.0);
    for item in args {
        match item {
            Atom::Number(val) => result = result.mul(&val),
            _ => return Err(EvalError(format!("Expected a number, found {:?}", item))),
        }
    }
    Ok(Atom::Number(result))
}

pub(crate) fn div(args: Vec<Atom>, _: &mut Env) -> Result<Atom, EvalError> {
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
