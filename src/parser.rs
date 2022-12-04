use crate::{
    lib::SchemeError,
    tokenizer::Token,
    types::{Exp, Value},
};
use std::collections::VecDeque;

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
            return Ok(Exp::List(exp));
        }
        Token::EndExp => Err(SchemeError("Unmatched ')'".to_string())),
        Token::Quote => Ok(Exp::Atom(Value::Quote(Box::from(parse(tokens)?)))),
        Token::Literal(value) => Ok(Exp::Atom(value.clone())),
    }
}
