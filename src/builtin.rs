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
        (
            ">".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &gt,
                name: ">".to_string(),
            })),
        ),
        (
            "<".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &lt,
                name: "<".to_string(),
            })),
        ),
        (
            ">=".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &gte,
                name: ">=".to_string(),
            })),
        ),
        (
            "<=".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &lte,
                name: "<=".to_string(),
            })),
        ),
        (
            "number?".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &number,
                name: "number?".to_string(),
            })),
        ),
        (
            "symbol?".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &symbol,
                name: "symbol?".to_string(),
            })),
        ),
        (
            "nil?".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &nil,
                name: "nil?".to_string(),
            })),
        ),
        (
            "builtin?".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &builtin,
                name: "builtin?".to_string(),
            })),
        ),
        (
            "special_form?".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &special_form,
                name: "special_form?".to_string(),
            })),
        ),
        (
            "quote?".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &quote,
                name: "quote?".to_string(),
            })),
        ),
        (
            "lambda?".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &lambda,
                name: "lambda?".to_string(),
            })),
        ),
        (
            "boolean?".to_string(),
            Atom::Builtin(Rc::from(Builtin {
                func: &boolean,
                name: "boolean?".to_string(),
            })),
        ),
    ])
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

pub(crate) fn eq(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 2, 2)?;
    Ok(Atom::Boolean(args[0] == args[1]))
}

pub(crate) fn gt(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 2, 2)?;
    if let (Atom::Number(x), Atom::Number(y)) = (args[0].clone(), args[1].clone()) {
        Ok(Atom::Boolean(x.eval() > y.eval()))
    } else {
        Err(SchemeError("> can only compare numbers".to_string()))
    }
}

pub(crate) fn lt(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 2, 2)?;
    if let (Atom::Number(x), Atom::Number(y)) = (args[0].clone(), args[1].clone()) {
        Ok(Atom::Boolean(x.eval() < y.eval()))
    } else {
        Err(SchemeError("> can only compare numbers".to_string()))
    }
}

pub(crate) fn gte(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 2, 2)?;
    if let (Atom::Number(x), Atom::Number(y)) = (args[0].clone(), args[1].clone()) {
        Ok(Atom::Boolean(x.eval() >= y.eval()))
    } else {
        Err(SchemeError("> can only compare numbers".to_string()))
    }
}

pub(crate) fn lte(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 2, 2)?;
    if let (Atom::Number(x), Atom::Number(y)) = (args[0].clone(), args[1].clone()) {
        Ok(Atom::Boolean(x.eval() <= y.eval()))
    } else {
        Err(SchemeError("> can only compare numbers".to_string()))
    }
}

/*
 *    Type Checking
 */

pub(crate) fn number(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 1, 1)?;
    Ok(Atom::Boolean(args[0].is_number()))
}

pub(crate) fn symbol(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 1, 1)?;
    Ok(Atom::Boolean(args[0].is_symbol()))
}

pub(crate) fn nil(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 1, 1)?;
    Ok(Atom::Boolean(args[0].is_nil()))
}

pub(crate) fn builtin(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 1, 1)?;
    Ok(Atom::Boolean(args[0].is_builtin()))
}

pub(crate) fn special_form(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 1, 1)?;
    Ok(Atom::Boolean(args[0].is_special_form()))
}

pub(crate) fn quote(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 1, 1)?;
    Ok(Atom::Boolean(args[0].is_quote()))
}

pub(crate) fn lambda(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 1, 1)?;
    Ok(Atom::Boolean(args[0].is_lambda()))
}

pub(crate) fn boolean(args: Vec<Atom>, _: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&args, 1, 1)?;
    Ok(Atom::Boolean(args[0].is_boolean()))
}
