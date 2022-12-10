use crate::{
    environment::Env,
    eval_all,
    evaluator::validate_num_args,
    types::{Exp, SchemeError},
};
use std::fmt::Debug;
use std::{cell::RefCell, fmt::Display, rc::Rc};

/*
    Function - allows calling lambdas and hardcoded functions
*/

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Function {
    Builtin(Builtin),
    Lambda(Lambda),
}

impl Function {
    pub(crate) fn call(
        &mut self,
        args: &Exp,
        env: &mut Rc<RefCell<Env>>,
    ) -> Result<Exp, SchemeError> {
        match self {
            Function::Builtin(b) => (b.func)(args, env),
            Function::Lambda(l) => l.eval(args),
        }
    }
}

impl Display for Function {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Function::Builtin(f) => f.name.clone(),
                Function::Lambda(l) => l.to_string(),
            }
        )
    }
}

/*
    Builtin - Type definition
*/

#[derive(Clone)]
pub(crate) struct Builtin {
    pub(crate) func: &'static dyn Fn(&Exp, &mut Rc<RefCell<Env>>) -> Result<Exp, SchemeError>,
    pub(crate) name: String,
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

/*
    Lambda
*/

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Lambda {
    pub(crate) params: Vec<String>,
    pub(crate) body: Vec<Exp>,
    pub(crate) env: Rc<RefCell<Env>>,
}

impl Lambda {
    pub(crate) fn eval(&mut self, args: &Exp) -> Result<Exp, SchemeError> {
        let args = args.unwrap_list()?;
        validate_num_args(&args, self.params.len(), self.params.len())?;
        for (name, val) in self.params.iter().zip(args) {
            self.env.borrow_mut().set(name, &val);
        }

        eval_all(&self.body, &mut self.env)
    }
}

impl Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = format!(
            "(lambda ({}) {})",
            self.params.join(" "),
            self.body
                .iter()
                .map(|exp| exp.to_string())
                .reduce(|p, c| p + " " + &c)
                .unwrap_or("".to_string())
        );
        write!(f, "{}", string)
    }
}
