use crate::atom::rational::Rational;
use crate::atom::{self, Atom};
use crate::environment::Env;
use crate::lib::validate_num_args;
use crate::lib::SchemeError;
use std::cell::RefCell;
use std::collections::HashMap;
use std::fmt::{Debug, Display};
use std::rc::Rc;

pub(crate) struct Builtin {
    pub(crate) func: &'static dyn Fn(Vec<Atom>, &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError>,
    name: String,
}

impl Display for Builtin {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "Builtin '{}", self.name)
    }
}

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

    fn eq(&self, other: &Self) -> bool {
        self.name == other.name
    }
}

pub(crate) fn builtins_map() -> HashMap<String, Atom> {
    HashMap::from([
        (
            "+".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &add,
                name: "+".to_string(),
            })),
        ),
        (
            "-".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &sub,
                name: "-".to_string(),
            })),
        ),
        (
            "*".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &mul,
                name: "*".to_string(),
            })),
        ),
        (
            "/".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &div,
                name: "/".to_string(),
            })),
        ),
        (
            "eq?".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &eq,
                name: "eq?".to_string(),
            })),
        ),
    ])
}

pub(crate) fn eq(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 2, 2)?;
    Ok(Atom::Boolean(args[0] == args[1]))
}

pub(crate) fn add(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    let mut result = Rational::from(0.0);
    for item in args {
        match item {
            Atom::Number(val) => result = result.add(&val),
            _ => return Err(SchemeError(format!("Expected a number, found {:?}", item))),
        };
    }
    Ok(Atom::Number(result))
}

pub(crate) fn sub(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 1, 0)?;
    let mut result;
    let first = args.get(0).unwrap();

    if let Atom::Number(num) = first {
        result = num.clone()
    } else {
        return Err(SchemeError(format!("Expected a number, found {:?}", first)));
    }

    if args.len() == 1 {
        return Ok(Atom::Number(result.mul(&Rational::from(-1.0))));
    }

    for item in args[1..].iter() {
        match item {
            Atom::Number(val) => result = result.sub(&val),
            _ => return Err(SchemeError(format!("Expected a number, found {:?}", item))),
        };
    }
    Ok(Atom::Number(result))
}

pub(crate) fn mul(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    let mut result = Rational::from(1.0);
    for item in args {
        match item {
            Atom::Number(val) => result = result.mul(&val),
            _ => return Err(SchemeError(format!("Expected a number, found {:?}", item))),
        }
    }
    Ok(Atom::Number(result))
}

pub(crate) fn div(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 2, 2)?;
    let mut args = args.iter();
    let num = args.next().unwrap();
    let den = args.next().unwrap();
    match (num, den) {
        (Atom::Number(n), Atom::Number(d)) => Ok(Atom::Number(n.div(&d))),
        _ => Err(SchemeError(format!(
            "Expected two numbers, found {} and {}",
            num, den
        ))),
    }
}
