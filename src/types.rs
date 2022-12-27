use self::{function::Function, rational::Rational};
use crate::error::SchemeError;
use std::fmt::{self, Display};

pub(crate) mod default_env;
pub(crate) mod function;
pub(crate) mod rational;

/*
    Expressions
*/

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Exp {
    List(Vec<Exp>),
    ImpList(Vec<Exp>),
    Atom(Value),
}

impl Exp {
    pub(crate) fn unwrap_atom(&self) -> Result<Value, SchemeError> {
        match self {
            Exp::Atom(atom) => Ok(atom.clone()),
            _ => Err(SchemeError::new(format!(
                "Expected an atom, found {}",
                self
            ))),
        }
    }

    pub(crate) fn unwrap_list(&self) -> Result<Vec<Exp>, SchemeError> {
        match self {
            Exp::List(list) => Ok(list.clone()),
            _ => Err(SchemeError::new(format!("Expected a list, found {}", self))),
        }
    }

    pub(crate) fn new_list() -> Self {
        Self::List(Vec::new())
    }

    pub(crate) fn imp_from(list: &[Exp]) -> Self {
        Self::ImpList(list.to_vec())
    }
}

impl Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let string = match self {
            Exp::List(list) => {
                String::from("(")
                    + list
                        .iter()
                        .map(|item| item.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                        .as_str()
                    + ")"
            }
            Exp::Atom(atom) => atom.to_string(),
            Exp::ImpList(list) => {
                let mut list_str: Vec<String> = list
                    .iter()
                    .map(|item| item.to_string())
                    .collect::<Vec<String>>();
                list_str.insert(list_str.len() - 1, ".".to_string());
                String::from("(") + list_str.join(" ").as_str() + ")"
            }
        };
        write!(f, "{}", string)
    }
}

impl From<&[Exp]> for Exp {
    fn from(list: &[Exp]) -> Self {
        Self::List(list.to_vec())
    }
}

/*
    Atomic values
*/

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Value {
    Boolean(bool),
    Number(Rational),
    Symbol(String),
    SpecialForm(SpecialForm),
    Quote(Box<Exp>),
    Function(Function),
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

/*
    Special forms - just a path-choosing enum, no behavior or data
*/

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
