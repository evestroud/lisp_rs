use std::{collections::VecDeque, fmt};

use crate::tokenizer::{Literal, Token};

#[derive(Debug)]
pub(crate) struct ParseError(String);

impl fmt::Display for ParseError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug)]
pub(crate) enum Exp {
    SubExp(Vec<Exp>),
    Atom(Literal),
}

pub(crate) fn parse(tokens: &mut VecDeque<Token>) -> Result<Exp, ParseError> {
    if tokens.len() == 0 {
        return Err(ParseError("Unexpected EOF while parsing".to_string()));
    }
    let t = &tokens.pop_front().unwrap();
    match t {
        Token::StartExp => {
            let mut exp = Vec::new();
            while tokens
                .front()
                .ok_or(ParseError("Unexpected EOF while parsing".to_string()))?
                != &Token::EndExp
            {
                exp.push(parse(tokens)?);
            }
            tokens.pop_front();
            return Ok(Exp::SubExp(exp));
        }
        Token::EndExp => Err(ParseError("Unmatched ')'".to_string())),
        Token::Atom(atom) => match atom {
            Literal::Int(num) => Ok(Exp::Atom(Literal::Int(*num))),
            Literal::Float(num) => Ok(Exp::Atom(Literal::Float(*num))),
            Literal::Symbol(symbol) => Ok(Exp::Atom(Literal::Symbol(symbol.to_string()))),
        },
    }
}
