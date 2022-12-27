use crate::{
    buffer::Buffer,
    error::SchemeError,
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
        return Err(SchemeError::new("Unexpected EOF while parsing".to_string()));
    }
    let t = &buffer.pop_front().unwrap();
    match t {
        Token::StartExp => {
            let mut exp = Vec::new();
            while ![Token::EndExp, Token::Dot].contains(
                buffer
                    .front()
                    .ok_or(SchemeError::new("Unexpected EOF while parsing".to_string()))?,
            ) {
                exp.push(parse(buffer)?);
            }

            let last = buffer.pop_front().unwrap();
            match last {
                Token::EndExp => Ok(Exp::from(&exp[..])),
                Token::Dot => {
                    let cdr = parse(buffer)?;
                    exp.push(cdr);
                    if buffer
                        .pop_front()
                        .ok_or(SchemeError::new("Unexpected EOF while parsing".to_string()))?
                        != Token::EndExp
                    {
                        return Err(SchemeError::new("'.' missing ')'".to_string()));
                    }
                    Ok(Exp::imp_from(&exp))
                }
                _ => panic!("Fatal error while parsing: List terminator was: {:?}", last),
            }
        }
        Token::EndExp => Err(SchemeError::new("Unmatched ')'".to_string())),
        Token::Dot => Err(SchemeError::new("Unbound pair".to_string())),
        Token::Quote => Ok(Exp::Atom(Value::Quote(Box::from(parse(buffer)?)))),
        Token::Literal(value) => Ok(Exp::Atom(value.clone())),
    }
}
