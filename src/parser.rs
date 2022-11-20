use std::{collections::VecDeque, fmt};

use crate::tokenizer::Token;

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
    Literal(Token),
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
        Token::Int(num) => Ok(Exp::Literal(Token::Int(*num))),
        Token::Float(num) => Ok(Exp::Literal(Token::Float(*num))),
        Token::Symbol(symbol) => Ok(Exp::Literal(Token::Symbol(symbol.to_string()))),
    }
}
