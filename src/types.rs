use self::{function::Function, rational::Rational};
use crate::lib::SchemeError;
use std::fmt::{self, Display};

pub mod function;
pub mod rational;

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Exp {
    List(Vec<Exp>),
    Atom(Value),
}

impl Exp {
    pub(crate) fn unwrap_atom(&self) -> Result<Value, SchemeError> {
        match self {
            Exp::List(_) => Err(SchemeError(format!("Expected an atom, found {}", self))),
            Exp::Atom(atom) => Ok(atom.clone()),
        }
    }
}

impl Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Exp::List(list) => list.iter().map(|item| item.to_string()).collect(),
                Exp::Atom(atom) => atom.to_string(),
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Value {
    // Nil,
    Boolean(bool),
    Number(Rational),
    Symbol(String),
    // Builtin(Rc<Builtin>),
    SpecialForm(SpecialForm),
    Quote(Box<Exp>),
    Function(Function), // Lambda(Box<Lambda>),
}

impl Display for Value {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Value::Boolean(b) => b.to_string(),
                Value::Number(n) => n.to_string(),
                Value::Symbol(s) => s.to_string(),
                Value::SpecialForm(sf) => sf.to_string(),
                Value::Quote(q) => q.to_string(),
                Value::Function(f) => f.to_string(),
            }
        )
    }
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum SpecialForm {
    Define,
    Let,
    Lambda,
    If,
    And,
    Or,
    Eval,
    Apply,
}

impl From<&str> for SpecialForm {
    fn from(s: &str) -> Self {
        match s.to_ascii_lowercase().as_str() {
            "define" => Self::Define,
            "let" => Self::Let,
            "lambda" => Self::Lambda,
            "if" => Self::If,
            "and" => Self::And,
            "or" => Self::Or,
            "eval" => Self::Eval,
            "apply" => Self::Apply,
            _ => panic!("SpecialForm::from called on incorrect string"),
        }
    }
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
                SpecialForm::And => "and".to_string(),
                SpecialForm::Or => "or".to_string(),
                SpecialForm::Eval => "eval".to_string(),
                SpecialForm::Apply => "apply".to_string(),
            }
        )
    }
}
