use std::fmt::Display;
use std::{collections::VecDeque, fmt};

use crate::atom::{Atom, SpecialForm};
use crate::lib::SchemeError;
use crate::tokenizer::{Literal, Token};

#[derive(Debug, Clone, PartialEq)]
pub(crate) enum Exp {
    SubExp(Vec<Exp>),
    Literal(Atom),
}

impl Display for Exp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let this = match self {
            Exp::SubExp(exp) => {
                // recursively parse subexpressions
                format!(
                    "({})",
                    exp.iter()
                        .map(|e| e.to_string())
                        .collect::<Vec<String>>()
                        .join(" ")
                )
            }
            Exp::Literal(atom) => atom.to_string(),
        };
        write!(f, "{}", this)
    }
}

pub(crate) fn parse_all(tokens: &mut VecDeque<Token>) -> Result<Vec<Exp>, SchemeError> {
    let mut result = Vec::new();
    while tokens.len() != 0 {
        result.push(parse(tokens)?);
    }
    Ok(result)
}

pub(crate) fn parse(tokens: &mut VecDeque<Token>) -> Result<Exp, SchemeError> {
    if tokens.len() == 0 {
        return Err(SchemeError("Unexpected EOF while parsing".to_string()));
    }
    let t = &tokens.pop_front().unwrap();
    match t {
        Token::StartExp => {
            let mut exp = Vec::new();
            while tokens
                .front()
                .ok_or(SchemeError("Unexpected EOF while parsing".to_string()))?
                != &Token::EndExp
            {
                exp.push(parse(tokens)?);
            }
            tokens.pop_front();
            return Ok(Exp::SubExp(exp));
        }
        Token::EndExp => Err(SchemeError("Unmatched ')'".to_string())),
        Token::Literal(atom) => match atom {
            Literal::Number(num) => Ok(Exp::Literal(Atom::Number(num.clone()))),
            Literal::Symbol(symbol) => {
                Ok(Exp::Literal(match symbol.to_ascii_lowercase().as_str() {
                    "nil" => Atom::Nil,
                    "let" => Atom::SpecialForm(SpecialForm::Let),
                    "define" => Atom::SpecialForm(SpecialForm::Define),
                    "lambda" => Atom::SpecialForm(SpecialForm::Lambda),
                    _ => Atom::Symbol(symbol.to_string()),
                }))
            }
            Literal::Boolean(b) => Ok(Exp::Literal(Atom::Boolean(*b))),
        },
    }
}
