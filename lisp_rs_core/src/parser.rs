use std::mem;

use crate::{buffer::Buffer, error::SchemeError, lexer::Token, types::Cell};

pub fn parse(buffer: &mut Buffer) -> Result<Cell, SchemeError> {
    // TODO this check could be a part of Buffer.pop_front and return an Err
    if buffer.len() == 0 {
        return Err(SchemeError::new("Unexpected EOF while parsing".to_string()));
    }
    let t = buffer.pop_front().unwrap();
    match t {
        Token::StartExp => parse_list(buffer),
        Token::EndExp => Err(SchemeError::new("Unexpected ')'".to_string())),
        Token::Dot => Err(SchemeError::new("Unexpected '.'".to_string())),
        Token::Quote => Ok(Cell::Quote(Box::new(parse(buffer)?))),
        Token::Literal(mut value) => Ok(mem::take(&mut value)),
    }
}

fn parse_list(buffer: &mut Buffer) -> Result<Cell, SchemeError> {
    if buffer.len() == 0 {
        return Err(SchemeError::new("Unexpected EOF while parsing".to_string()));
    }
    let t = buffer.pop_front().unwrap();
    match t {
        Token::StartExp => parse_list(buffer),
        Token::EndExp => Ok(Cell::Nil),
        Token::Dot => parse_improper_list(buffer),
        Token::Quote => todo!(),
        Token::Literal(value) => {
            let car = value;
            let cdr = parse_list(buffer)?;
            Ok(Cell::Pair(car, Box::new(cdr)))
        }
    }
}

fn parse_improper_list(buffer: &mut Buffer) -> Result<Cell, SchemeError> {
    if buffer.len() == 0 {
        return Err(SchemeError::new("Unexpected EOF while parsing".to_string()));
    }

    let cdr = parse(buffer)?;

    if let Token::EndExp = buffer.pop_front().unwrap() {
        Ok(cdr)
    } else {
        Err(SchemeError::new("Missing close paren ')'".to_owned()))
    }
}
