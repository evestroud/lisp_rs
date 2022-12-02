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

fn apply(args: &Vec<Exp>, env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    if args.len() == 0 {
        return Ok(Atom::Nil);
    }
    let first = evaluate(&args[0], env)?;
    let rest = &args[1..];
    match first {
        Atom::Builtin(f) => {
            let rest = rest
                .iter()
                .map(|exp| evaluate(exp, env))
                .collect::<Result<Vec<Atom>, SchemeError>>()?;
            (f.func)(rest, env)
        }
        Atom::Lambda(mut lambda) => lambda.eval(
            rest.iter()
                .map(|exp| evaluate(exp, env))
                .collect::<Result<Vec<Atom>, SchemeError>>()?,
        ),
        Atom::SpecialForm(form) => match form {
            SpecialForm::Define => do_define_form(rest, env),
            SpecialForm::Let => do_let_form(rest, env),
            SpecialForm::Lambda => do_lambda_form(rest, env),
            SpecialForm::If => do_if_form(rest, env),
        },
        Atom::Nil => return Ok(Atom::Nil),
        _ => Err(SchemeError(format!("Expected a symbol, found {:?}", first))),
    }
}

fn do_if_form(args: &[Exp], env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(args, 3, 3)?;
    let condition = evaluate(&args[0], env)?;
    if let Atom::Boolean(false) = condition {
        return evaluate(&args[2], env);
    }
    evaluate(&args[1], env)
}

fn do_lambda_form(args: &[Exp], env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(args, 2, 0)?;
    let params = match &args[0] {
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
    };
    let body = args[1].clone();
    let env = create_closure(env.clone());
    Ok(Atom::Lambda(Box::new(Lambda { params, body, env })))
}

fn do_let_form(args: &[Exp], env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(args, 2, 0)?;
    let mut closure = create_closure(env.clone());

    match &args[0] {
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
        &args[1..]
            .iter()
            .map(|arg| arg.clone())
            .collect::<Vec<Exp>>(),
        &mut closure,
    )
}

fn do_define_form(args: &[Exp], env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(args, 2, 2)?;
    let second = &args[0];
    match second {
        Exp::SubExp(signature) => {
            if let Atom::Symbol(name) = evaluate(&as_quote(&signature[0]), env)? {
                let params = Exp::SubExp(signature[1..].to_vec());
                let lambda_form_args = [[params], [args[1].clone()]].concat();
                let lambda = do_lambda_form(lambda_form_args.as_slice(), env)?;
                env.borrow_mut().set(&name, &lambda);
                return Ok(Atom::Nil);
            } else {
                return Err(SchemeError(format!(
                    "Expected a symbol as the name, found {}",
                    signature[0]
                )));
            }
        }
        Exp::Literal(val) => {
            if let Atom::Symbol(symbol) = val {
                let third = evaluate(&args[1], env)?;
                env.borrow_mut().set(&symbol, &third);
                return Ok(Atom::Nil);
            }
        }
    }
    Err(SchemeError(format!(
        "Expected a symbol as the name, found {}",
        second
    )))
}

fn as_quote(exp: &Exp) -> Exp {
    Exp::Literal(Atom::Quote(Box::new(exp.clone())))
}
