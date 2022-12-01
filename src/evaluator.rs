use std::{cell::RefCell, rc::Rc};

use crate::{
    atom::{Atom, Lambda, SpecialForm},
    environment::{create_closure, Env},
    lib::{validate_num_args, SchemeError},
    parser::Exp,
};

pub(crate) fn eval_all(input: &Vec<Exp>, env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    let mut result = Atom::Nil;
    for exp in input {
        result = evaluate(exp, env)?;
    }
    Ok(result)
}

pub(crate) fn evaluate(input: &Exp, env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    match input {
        Exp::SubExp(list) => apply(list, env),
        Exp::Literal(atom) => match atom {
            Atom::Symbol(symbol) => env.borrow().get(&symbol).map(|val| val.clone()),
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

fn apply(list: &Vec<Exp>, env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
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
            (f.func)(rest, env)
        }
        Atom::Lambda(mut lambda) => lambda.eval(
            list_iter
                .map(|exp| evaluate(exp, env))
                .collect::<Result<Vec<Atom>, SchemeError>>()?,
        ),
        Atom::SpecialForm(form) => match form {
            SpecialForm::Define => do_define_form(&list[1..], env),
            SpecialForm::Let => do_let_form(&list[1..], env),
            SpecialForm::Lambda => do_lambda_form(&list[1..], env),
        },
        Atom::Nil => return Ok(Atom::Nil),
        _ => Err(SchemeError(format!("Expected a symbol, found {:?}", first))),
    }
}

fn do_lambda_form(args: &[Exp], env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&Vec::from(args), 2, 0)?;
    let mut args_iter = args.iter();
    let params = match args_iter.next() {
        Some(param_args) => match param_args {
            Exp::SubExp(param_list) => param_list
                .iter()
                .map(|arg| {
                    let atom = evaluate(&as_quote(arg), env)?;
                    match atom {
                        Atom::Symbol(name) => Ok(name.to_string()),
                        _ => Err(SchemeError(format!(
                            "Parameter list expects symbols, found {}",
                            atom
                        ))),
                    }
                })
                .collect::<Result<Vec<String>, SchemeError>>()?,
            Exp::Literal(_) => todo!(),
        },
        None => todo!(),
    };
    let body = args_iter.next().unwrap().clone();
    let env = create_closure(env.clone());
    Ok(Atom::Lambda(Box::new(Lambda { params, body, env })))
}

fn do_let_form(args: &[Exp], env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&Vec::from(args), 2, 0)?;
    let mut args_iter = args.iter();
    let mut closure = create_closure(env.clone());

    match args_iter.next().unwrap() {
        Exp::SubExp(pairs) => {
            for pair in pairs {
                if let Exp::SubExp(pair_vec) = pair {
                    do_define_form(pair_vec, &mut closure)?;
                }
            }
        }
        Exp::Literal(_) => return Err(SchemeError("Expected a list".to_string())),
    }
    eval_all(
        &args_iter.map(|arg| arg.clone()).collect::<Vec<Exp>>(),
        &mut closure,
    )
}

fn do_define_form(args: &[Exp], env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(&Vec::from(args), 2, 2)?;
    let mut args_iter = args.iter();
    let second = evaluate(&as_quote(args_iter.next().unwrap()), env)?;
    let third = evaluate(args_iter.next().unwrap(), env)?;
    if let Atom::Symbol(symbol) = second {
        env.borrow_mut().set(&symbol, &third);
    } else {
        return Err(SchemeError(format!("Expected a symbol, found {}", second)));
    }
    Ok(Atom::Nil)
}

fn as_quote(exp: &Exp) -> Exp {
    Exp::Literal(Atom::Quote(Box::new(exp.clone())))
}
