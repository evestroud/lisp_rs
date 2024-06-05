use std::mem;

use crate::{
    buffer::Buffer,
    error::SchemeError,
    lexer::Token,
    types::{Cell, Value},
};

pub(crate) fn parse(buffer: &mut Buffer) -> Result<Value, SchemeError> {
    if buffer.len() == 0 {
        return Err(SchemeError::new("Unexpected EOF while parsing".to_string()));
    }
    let t = &buffer.pop_front().unwrap();
    match t {
        Token::StartExp => {
            let list = Cell::default();
            let mut tail = list;
            while ![Token::EndExp, Token::Dot].contains(
                buffer
                    .front()
                    .ok_or(SchemeError::new("Unexpected EOF while parsing".to_string()))?,
            ) {
                // exp.push(parse(buffer)?);
                tail.car = parse(buffer)?;
                let mut new_tail = Cell::default();
                tail.cdr = Value::List(Box::new(mem::take(&mut new_tail)));
                tail = new_tail;
            }
            Ok(Value::List(Box::new(list)))

            // let last = buffer.pop_front().unwrap();
            // match last {
            //     Token::EndExp => Ok(Exp::from(&exp[..])),
            //     Token::Dot => {
            //         let cdr = parse(buffer)?;
            //         exp.push(cdr);
            //         if buffer
            //             .pop_front()
            //             .ok_or(SchemeError::new("Unexpected EOF while parsing".to_string()))?
            //             != Token::EndExp
            //         {
            //             return Err(SchemeError::new("'.' missing ')'".to_string()));
            //         }
            //         Ok(Exp::imp_from(&exp))
            //     }
            //     _ => panic!("Fatal error while parsing: List terminator was: {:?}", last),
            // }
        }
        Token::EndExp => Err(SchemeError::new("Unmatched ')'".to_string())),
        Token::Dot => Err(SchemeError::new("Unbound pair".to_string())),
        Token::Quote => Ok(Value::Quote(Box::from(parse(buffer)?))),
        Token::Literal(value) => Ok(value.clone()),
    }
}

pub(crate) fn parse_token(token: Token) -> Value {
    match token {
        Token::StartExp => todo!(),
        Token::EndExp => todo!(),
        Token::Dot => todo!(),
        Token::Quote => todo!(),
        Token::Literal(_) => todo!(),
    }
}
