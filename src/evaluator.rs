use crate::{
    environment::{create_closure, Env},
    error::SchemeError,
    types::function::{Function, Lambda},
    types::{Exp, SpecialForm, Value},
};
use std::{cell::RefCell, rc::Rc};

pub(crate) fn eval_all(input: &Exp, env: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let mut result = Exp::List(Vec::new());
    if let Exp::List(statements) = input {
        for exp in statements {
            result = evaluate(exp, env)?;
        }
        Ok(result)
    } else {
        Err(SchemeError::new(
            "eval-all called on a non-list".to_string(),
        ))
    }
}

pub(crate) fn evaluate(input: &Exp, env: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    match input {
        Exp::List(list) => {
            validate_num_args("evalute", &list, 1, usize::MAX)?;
            let operator = evaluate(list.get(0).unwrap(), env)?;
            if let Exp::Atom(Value::SpecialForm(form)) = operator {
                match form {
                    SpecialForm::Define => do_define_form(&Exp::from(&list[1..]), env),
                    SpecialForm::Let => do_let_form(&Exp::from(&list[1..]), env),
                    SpecialForm::Lambda => do_lambda_form(&Exp::from(&list[1..]), env),
                    SpecialForm::If => do_if_form(&Exp::from(&list[1..]), env),
                    SpecialForm::And => do_and_form(&Exp::from(&list[1..]), env),
                    SpecialForm::Or => do_or_form(&Exp::from(&list[1..]), env),
                    SpecialForm::Eval => {
                        validate_num_args("eval", &list[1..], 1, 1)?;
                        evaluate(&evaluate(list.get(1).unwrap(), env)?, env)
                    }
                    SpecialForm::Apply => {
                        let apply_list = &list[1..];
                        validate_num_args("apply", &apply_list, 2, 2)?;
                        let operator = evaluate(apply_list.get(0).unwrap(), env)?;
                        let args = evaluate(apply_list.get(1).unwrap(), env)?;
                        apply(&operator, &args, env)
                    }
                }
            } else {
                apply(&operator, &Exp::List(list[1..].to_vec()), env)
            }
        }
        Exp::Atom(atom) => match atom {
            Value::Symbol(symbol) => Ok(env.borrow().get(symbol)?),
            Value::Quote(quoted) => Ok(*quoted.clone()),
            _ => Ok(Exp::Atom(atom.clone())),
        },
    }
}

pub(crate) fn eval_args(input: &Exp, env: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let mut results = Vec::new();
    for exp in input.unwrap_list()? {
        results.push(evaluate(&exp, env)?);
    }
    Ok(Exp::List(results))
}

pub(crate) fn apply(
    operator: &Exp,
    args: &Exp,
    env: &mut Rc<RefCell<Env>>,
) -> Result<Exp, SchemeError> {
    if let Value::Function(mut function) = operator.unwrap_atom()? {
        let args = eval_args(args, env)?;
        function.call(&args, env)
    } else {
        Err(SchemeError::new(format!(
            "Expected a function, found {}",
            operator
        )))
    }
}

fn do_define_form(args: &Exp, env: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    validate_num_args("define", args, 1, usize::MAX)?;
    if let Exp::List(args) = args {
        let second = &args[0];
        match second {
            Exp::List(signature) => {
                validate_num_args("define signature", signature, 1, usize::MAX)?;
                if let Value::Symbol(name) = signature.get(0).unwrap().unwrap_atom()? {
                    let params = Exp::List(signature[1..].to_vec());
                    let lambda_form_args = vec![&[params][..], &args[1..]].concat();
                    let lambda = do_lambda_form(lambda_form_args.as_slice(), env)?;
                    env.borrow_mut().set(&name, &lambda);
                    return Ok(Exp::new_list());
                } else {
                    return Err(SchemeError::new(format!(
                        "Expected a symbol as the name, found {}",
                        signature[0]
                    )));
                }
            }
            Exp::Atom(val) => {
                validate_num_args("define value", args, 2, 2)?;
                if let Value::Symbol(symbol) = val {
                    let third = evaluate(&args[1], env)?;
                    env.borrow_mut().set(&symbol, &third);
                    return Ok(Exp::new_list());
                }
            }
        }
        Err(SchemeError::new(format!(
            "Expected a symbol as the name, found {}",
            second
        )))
    } else {
        Err(SchemeError::new(format!(
            "define expects a list, found {}",
            args
        )))
    }
}

fn do_let_form(args: &Exp, env: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    validate_num_args("let", args, 2, usize::MAX)?;
    let mut closure = create_closure(env.clone());

    match &args[0] {
        Exp::List(pairs) => {
            for pair in pairs {
                if let Exp::List(pair_vec) = pair {
                    validate_num_args("let pair", pair_vec, 2, usize::MAX)?;
                    if let Value::Symbol(name) = pair_vec.get(0).unwrap().unwrap_atom()? {
                        let value = evaluate(pair_vec.get(1).unwrap(), &mut closure)?;
                        closure.borrow_mut().set(&name, &value);
                    }
                }
            }
        }
        Exp::Atom(_) => {
            return Err(SchemeError::new(
                "Let expects a list of definitions".to_string(),
            ))
        }
    }
    eval_all(
        &args[1..]
            .iter()
            .map(|arg| arg.clone())
            .collect::<Vec<Exp>>(),
        &mut closure,
    )
}

fn do_lambda_form(args: &Exp, env: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    validate_num_args("lambda", args, 2, usize::MAX)?;
    let params = match &args[0] {
        Exp::List(param_list) => param_list
            .iter()
            .map(|arg| {
                let atom = arg.unwrap_atom()?;
                match atom {
                    Value::Symbol(name) => Ok(name.to_string()),
                    _ => Err(SchemeError::new(format!(
                        "Parameter list expects symbols, found {}",
                        atom
                    ))),
                }
            })
            .collect::<Result<Vec<String>, SchemeError>>()?,
        Exp::Atom(_) => todo!(),
    };
    let body = args[1..].to_vec();
    let env = create_closure(env.clone());
    Ok(Exp::Atom(Value::Function(Function::Lambda(Lambda {
        params,
        body,
        env,
    }))))
}

fn do_if_form(args: &Exp, env: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    validate_num_args("if", args, 3, 3)?;
    let condition = evaluate(&args[0], env)?.unwrap_atom()?;
    if let Value::Boolean(false) = condition {
        return evaluate(&args[2], env);
    }
    evaluate(&args[1], env)
}

fn do_and_form(args: &Exp, env: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let mut val = Value::Boolean(true);
    for a in args {
        val = evaluate(a, env)?.unwrap_atom()?;
        if val == Value::Boolean(false) {
            break;
        }
    }
    Ok(Exp::Atom(val))
}

fn do_or_form(args: &Exp, env: &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError> {
    let mut val = Value::Boolean(false);
    for a in args {
        val = evaluate(a, env)?.unwrap_atom()?;
        if val != Value::Boolean(false) {
            break;
        }
    }
    Ok(Exp::Atom(val))
}

pub(crate) fn validate_num_args<T>(
    name: &str,
    args: &[T],
    min: usize,
    max: usize,
) -> Result<(), SchemeError> {
    match args.len() >= min {
        true => Ok(()),
        false => Err(SchemeError::new(format!(
            "{} expects at least {} args, found {}",
            name,
            min,
            args.len(),
        ))),
    }?;
    if max < usize::MAX {
        match args.len() <= max {
            true => Ok(()),
            false => Err(SchemeError::new(format!(
                "{} takes a maximum of {} args, found {}",
                name,
                max,
                args.len(),
            ))),
        }?;
    }
    Ok(())
}

// pub(crate) fn validate_type<T>(arg: Exp) -> Result<(), SchemeError> {
//     match T {}
// }
