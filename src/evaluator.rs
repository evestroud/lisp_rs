use std::{cell::RefCell, rc::Rc};

use crate::{
    atom::{Atom, Lambda, SchemeExp, SpecialForm},
    environment::{create_closure, Env},
    lib::{validate_num_args, SchemeError},
};

pub(crate) fn eval_all(
    input: &[SchemeExp],
    env: &mut Rc<RefCell<Env>>,
) -> Result<Atom, SchemeError> {
    let mut result = Atom::Nil;
    for exp in input {
        result = evaluate(exp, env)?;
    }
    Ok(result)
}

pub(crate) fn evaluate(input: &SchemeExp, env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    match input {
        SchemeExp::List(list) => {
            let operator = evaluate(&list[0], env)?;
            let args = list[1..]
                .iter()
                .map(|exp| evaluate(exp, env).clone())
                .collect::<Result<Vec<Atom>, _>>()?;
            if let Atom::SpecialForm(ref form) = operator {
                match form {
                    SpecialForm::Define => do_define_form(&list[1..], env),
                    SpecialForm::Let => do_let_form(&list[1..], env),
                    SpecialForm::Lambda => do_lambda_form(&list[1..], env),
                    SpecialForm::If => do_if_form(&list[1..], env),
                    SpecialForm::And => do_and_form(&list[1..], env),
                    SpecialForm::Or => do_or_form(&list[1..], env),
                    SpecialForm::Eval => eval_all(&&list[1..].to_vec(), env),
                    SpecialForm::Apply => {
                        if let Some(operator) = args.get(0) {
                            apply(operator.clone(), &args[1..], env)
                        } else {
                            return Err(SchemeError("Expected an argument".to_string()));
                        }
                    }
                }
            } else {
                apply(operator, &args, env)
            }
        }
        SchemeExp::Atom(atom) => match atom {
            Atom::Symbol(symbol) => env.borrow().get(&symbol).map(|val| val.clone()),
            Atom::Quote(exp) => {
                // ? not sure if this is correct
                if let SchemeExp::Atom(val) = *exp.clone() {
                    return Ok(val.clone());
                }
                Ok(atom.clone())
            }
            _ => Ok(atom.clone()),
        },
    }
}

fn apply(operator: Atom, args: &[Atom], env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    if args.len() == 0 {
        return Err(SchemeError("Expected an expression".to_string()));
    }
    match operator {
        Atom::Builtin(f) => (f.func)(args.to_vec(), env),
        Atom::Lambda(mut lambda) => lambda.eval(args),

        Atom::Nil => return Ok(Atom::Nil),
        _ => Err(SchemeError(format!(
            "Expected a function, found {:?}",
            operator
        ))),
    }
}

fn do_and_form(args: &[SchemeExp], env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    let mut val = Atom::Boolean(true);
    for a in args {
        val = evaluate(a, env)?;
        if val == Atom::Boolean(false) {
            break;
        }
    }
    Ok(val)
}

fn do_or_form(args: &[SchemeExp], env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    let mut val = Atom::Boolean(false);
    for a in args {
        val = evaluate(a, env)?;
        if val != Atom::Boolean(false) {
            break;
        }
    }
    Ok(val)
}

fn do_if_form(args: &[SchemeExp], env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(args, 3, 3)?;
    let condition = evaluate(&args[0], env)?;
    if let Atom::Boolean(false) = condition {
        return evaluate(&args[2], env);
    }
    evaluate(&args[1], env)
}

fn do_lambda_form(args: &[SchemeExp], env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(args, 2, 0)?;
    let params = match &args[0] {
        SchemeExp::List(param_list) => param_list
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
        SchemeExp::Atom(_) => todo!(),
    };
    let body = args[1..].to_vec();
    let env = create_closure(env.clone());
    Ok(Atom::Lambda(Box::new(Lambda { params, body, env })))
}

fn do_let_form(args: &[SchemeExp], env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(args, 2, 0)?;
    let mut closure = create_closure(env.clone());

    match &args[0] {
        SchemeExp::List(pairs) => {
            for pair in pairs {
                if let SchemeExp::List(pair_vec) = pair {
                    do_define_form(pair_vec, &mut closure)?;
                }
            }
        }
        SchemeExp::Atom(_) => return Err(SchemeError("Expected a list".to_string())),
    }
    eval_all(
        &args[1..]
            .iter()
            .map(|arg| arg.clone())
            .collect::<Vec<SchemeExp>>(),
        &mut closure,
    )
}

fn do_define_form(args: &[SchemeExp], env: &mut Rc<RefCell<Env>>) -> Result<Atom, SchemeError> {
    validate_num_args(args, 2, 2)?;
    let second = &args[0];
    match second {
        SchemeExp::List(signature) => {
            if let Atom::Symbol(name) = evaluate(&as_quote(&signature[0]), env)? {
                let params = SchemeExp::List(signature[1..].to_vec());
                let lambda_form_args = vec![&[params][..], &args[1..]].concat();
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
        SchemeExp::Atom(val) => {
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

fn as_quote(exp: &SchemeExp) -> SchemeExp {
    SchemeExp::Atom(Atom::Quote(Box::new(exp.clone())))
}
