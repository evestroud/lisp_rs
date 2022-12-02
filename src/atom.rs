use crate::{
    builtin::Builtin,
    environment::Env,
    evaluator::evaluate,
    lib::{validate_num_args, SchemeError},
    parser::Exp,
};
use std::{
    cell::RefCell,
    fmt::{Debug, Display},
    rc::Rc,
};

pub mod rational;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Atom {
    Number(rational::Rational),
    Symbol(String),
    Nil,
    Builtin(Rc<Builtin>),
    SpecialForm(SpecialForm),
    Quote(Box<Exp>),
    Lambda(Box<Lambda>),
    Boolean(bool),
}

impl Display for Atom {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let val = match self {
            Atom::Number(n) => n.to_string(),
            Atom::Symbol(s) => s.to_string(),
            Atom::Nil => "Nil".to_string(),
            Atom::Builtin(b) => b.to_string(),
            Atom::SpecialForm(f) => f.to_string(),
            Atom::Quote(exp) => format!("'{}", exp),
            Atom::Lambda(l) => l.to_string(),
            Atom::Boolean(b) => b.to_string(),
        };
        write!(f, "{}", val)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SpecialForm {
    Define,
    Let,
    Lambda,
    If,
}

impl Display for SpecialForm {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                SpecialForm::Define => "define".to_string(),
                SpecialForm::Let => "let".to_string(),
                SpecialForm::Lambda => "lambda".to_string(),
                SpecialForm::If => "if".to_string(),
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct Lambda {
    pub(crate) params: Vec<String>,
    pub(crate) body: Exp,
    pub(crate) env: Rc<RefCell<Env>>,
}

impl Lambda {
    pub(crate) fn eval(&mut self, args: Vec<Atom>) -> Result<Atom, SchemeError> {
        validate_num_args(&args, self.params.len(), self.params.len())?;
        for (name, val) in self.params.iter().zip(args) {
            self.env.borrow_mut().set(name, &val);
        }

        evaluate(&self.body, &mut self.env)
    }
}

impl Display for Lambda {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let string = format!("(lambda ({}) {})", self.params.join(" "), self.body);
        write!(f, "{}", string)
    }
}
