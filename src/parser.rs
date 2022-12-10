use crate::{
    buffer::Buffer,
    lib::SchemeError,
    tokenizer::Token,
    types::{Exp, Value},
};

pub(crate) fn parse_all(buffer: &mut Buffer) -> Result<Vec<Exp>, SchemeError> {
    let mut result = Vec::new();
    while buffer.len() != 0 {
        result.push(parse(buffer)?);
    }
    Ok(result)
}

pub(crate) fn parse(buffer: &mut Buffer) -> Result<Exp, SchemeError> {
    if buffer.len() == 0 {
        return Err(SchemeError {
            message: "Unexpected EOF while parsing".to_string(),
        });
    }
    let t = &buffer.pop_front().unwrap();
    match t {
        Token::StartExp => {
            let mut exp = Vec::new();
            while buffer.front().ok_or(SchemeError {
                message: "Unexpected EOF while parsing".to_string(),
            })? != &Token::EndExp
            {
                exp.push(parse(buffer)?);
            }
            buffer.pop_front();
            return Ok(Exp::List(exp));
        }
        Token::EndExp => Err(SchemeError {
            message: "Unmatched '}'".to_string(),
        }),
        Token::Quote => Ok(Exp::Atom(Value::Quote(Box::from(parse(buffer)?)))),
        Token::Literal(value) => Ok(Exp::Atom(value.clone())),
    }
}
