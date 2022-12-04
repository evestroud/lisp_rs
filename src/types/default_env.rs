use crate::{
    lib::validate_num_args,
    types::{
        function::{Builtin, Function},
        Exp, Rational, SchemeError, Value,
    },
    Env,
};
use std::{cell::RefCell, collections::HashMap, rc::Rc};

pub(crate) fn builtins_map() -> HashMap<String, Exp> {
    HashMap::from([
        /*

        Basic math

            */
        (
            "+".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &add,
                name: "+".to_string(),
            }))),
        ),
        (
            "-".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &sub,
                name: "-".to_string(),
            }))),
        ),
        (
            "*".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &mul,
                name: "*".to_string(),
            }))),
        ),
        (
            "/".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &div,
                name: "/".to_string(),
            }))),
        ),
        /*

        Comparisons

            */
        (
            "eq?".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &eq,
                name: "eq?".to_string(),
            }))),
        ),
        (
            "<".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &lt,
                name: "<".to_string(),
            }))),
        ),
        (
            ">".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &gt,
                name: ">".to_string(),
            }))),
        ),
        (
            "<=".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &lte,
                name: "<=".to_string(),
            }))),
        ),
        (
            ">=".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &gte,
                name: ">=".to_string(),
            }))),
        ),
        /*

            Type checking

        */
        (
            "number?".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &number,
                name: "number?".to_string(),
            }))),
        ),
        (
            "symbol?".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &symbol,
                name: "symbol?".to_string(),
            }))),
        ),
        (
            "nil?".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &nil,
                name: "nil?".to_string(),
            }))),
        ),
        (
            "function?".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &function,
                name: "function?".to_string(),
            }))),
        ),
        (
            "special_form?".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &special_form,
                name: "special_form?".to_string(),
            }))),
        ),
        (
            "quote?".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &quote,
                name: "quote?".to_string(),
            }))),
        ),
        (
            "boolean?".to_string(),
            Exp::Atom(Value::Function(Function::Builtin(Builtin {
                func: &boolean,
                name: "boolean?".to_string(),
            }))),
        ),
    ])
}

/*
    Basic math
*/

pub(crate) fn add(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let mut result = Rational::from(0.0);

    for item in args.unwrap_list()? {
        if let Exp::Atom(Value::Number(num)) = item {
            result = result.add(&num);
        } else {
            return Err(SchemeError(format!("Expected a number, found {:?}", item)));
        }
    }

    Ok(Exp::Atom(Value::Number(result)))
}

pub(crate) fn sub(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let args = args.unwrap_list()?;
    validate_num_args(&args, 1, 0)?;
    let mut result;
    let first = args.get(0).unwrap();

    if let Exp::Atom(Value::Number(num)) = first {
        result = num.clone();
    } else {
        return Err(SchemeError(format!("Expected a number, found {:?}", first)));
    }

    if args.len() == 1 {
        return Ok(Exp::Atom(Value::Number(result.mul(&Rational::from(-1.0)))));
    }

    for item in args[1..].iter() {
        if let Exp::Atom(Value::Number(num)) = item {
            result = result.sub(&num.clone());
        } else {
            return Err(SchemeError(format!("Expected a number, found {:?}", first)));
        }
    }

    Ok(Exp::Atom(Value::Number(result)))
}

pub(crate) fn mul(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let mut result = Rational::from(1.0);

    for item in args.unwrap_list()? {
        if let Exp::Atom(Value::Number(num)) = item {
            result = result.mul(&num);
        } else {
            return Err(SchemeError(format!("Expected a number, found {:?}", item)));
        }
    }

    Ok(Exp::Atom(Value::Number(result)))
}

pub(crate) fn div(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let args = args.unwrap_list()?;
    validate_num_args(&args, 2, 2)?;
    let num = args.get(0).unwrap().unwrap_atom()?;
    let den = args.get(1).unwrap().unwrap_atom()?;
    if let (Value::Number(n), Value::Number(d)) = (num.clone(), den.clone()) {
        Ok(Exp::Atom(Value::Number(n.div(&d))))
    } else {
        Err(SchemeError(format!(
            "Expected two numbers, found {} and {}",
            num, den
        )))
    }
}

/*
   Comparison
*/

pub(crate) fn eq(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let args = args.unwrap_list()?;
    validate_num_args(&args, 2, 2)?;
    Ok(Exp::Atom(Value::Boolean(args[0] == args[1])))
}

pub(crate) fn gt(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let args = args.unwrap_list()?;
    validate_num_args(&args, 2, 2)?;
    if let (Exp::Atom(Value::Number(x)), Exp::Atom(Value::Number(y))) =
        (args[0].clone(), args[1].clone())
    {
        Ok(Exp::Atom(Value::Boolean(x.eval() > y.eval())))
    } else {
        Err(SchemeError("> can only compare numbers".to_string()))
    }
}

pub(crate) fn lt(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let args = args.unwrap_list()?;
    validate_num_args(&args, 2, 2)?;
    if let (Exp::Atom(Value::Number(x)), Exp::Atom(Value::Number(y))) =
        (args[0].clone(), args[1].clone())
    {
        Ok(Exp::Atom(Value::Boolean(x.eval() < y.eval())))
    } else {
        Err(SchemeError("> can only compare numbers".to_string()))
    }
}

pub(crate) fn gte(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let args = args.unwrap_list()?;
    validate_num_args(&args, 2, 2)?;
    if let (Exp::Atom(Value::Number(x)), Exp::Atom(Value::Number(y))) =
        (args[0].clone(), args[1].clone())
    {
        Ok(Exp::Atom(Value::Boolean(x.eval() >= y.eval())))
    } else {
        Err(SchemeError("> can only compare numbers".to_string()))
    }
}

pub(crate) fn lte(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let args = args.unwrap_list()?;
    validate_num_args(&args, 2, 2)?;
    if let (Exp::Atom(Value::Number(x)), Exp::Atom(Value::Number(y))) =
        (args[0].clone(), args[1].clone())
    {
        Ok(Exp::Atom(Value::Boolean(x.eval() <= y.eval())))
    } else {
        Err(SchemeError("> can only compare numbers".to_string()))
    }
}

/*
 *    Type Checking
 */

pub(crate) fn number(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let args = args.unwrap_list()?;
    validate_num_args(&args, 1, 1)?;
    if let Value::Number(_) = args.get(0).unwrap().unwrap_atom()? {
        Ok(Exp::Atom(Value::Boolean(true)))
    } else {
        Ok(Exp::Atom(Value::Boolean(false)))
    }
}

pub(crate) fn symbol(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let args = args.unwrap_list()?;
    validate_num_args(&args, 1, 1)?;
    if let Value::Symbol(_) = args.get(0).unwrap().unwrap_atom()? {
        Ok(Exp::Atom(Value::Boolean(true)))
    } else {
        Ok(Exp::Atom(Value::Boolean(false)))
    }
}

pub(crate) fn nil(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let args = args.unwrap_list()?;
    validate_num_args(&args, 1, 1)?;
    if args.get(0).unwrap().unwrap_list()?.len() == 0 {
        Ok(Exp::Atom(Value::Boolean(true)))
    } else {
        Ok(Exp::Atom(Value::Boolean(false)))
    }
}

pub(crate) fn function(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let args = args.unwrap_list()?;
    validate_num_args(&args, 1, 1)?;
    if let Value::Function(_) = args.get(0).unwrap().unwrap_atom()? {
        Ok(Exp::Atom(Value::Boolean(true)))
    } else {
        Ok(Exp::Atom(Value::Boolean(false)))
    }
}

pub(crate) fn special_form(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let args = args.unwrap_list()?;
    validate_num_args(&args, 1, 1)?;
    if let Value::SpecialForm(_) = args.get(0).unwrap().unwrap_atom()? {
        Ok(Exp::Atom(Value::Boolean(true)))
    } else {
        Ok(Exp::Atom(Value::Boolean(false)))
    }
}

pub(crate) fn quote(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let args = args.unwrap_list()?;
    validate_num_args(&args, 1, 1)?;
    if let Value::Quote(_) = args.get(0).unwrap().unwrap_atom()? {
        Ok(Exp::Atom(Value::Boolean(true)))
    } else {
        Ok(Exp::Atom(Value::Boolean(false)))
    }
}

pub(crate) fn boolean(args: &Exp, _: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let args = args.unwrap_list()?;
    validate_num_args(&args, 1, 1)?;
    if let Value::Boolean(_) = args.get(0).unwrap().unwrap_atom()? {
        Ok(Exp::Atom(Value::Boolean(true)))
    } else {
        Ok(Exp::Atom(Value::Boolean(false)))
    }
}
