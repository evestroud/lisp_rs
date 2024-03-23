use std::fmt::Display;

use crate::{buffer::Buffer, error::SchemeError};

#[derive(Clone, Debug, PartialEq)]
pub enum Token {
    StartExp,
    EndExp,
    Dot,
    // Quote,
    // Literal(Value),
    Number(i32),
    Boolean(bool),
    Symbol(String),
}

impl Display for Token {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{}",
            match self {
                Token::StartExp => "(".to_owned(),
                Token::EndExp => ")".to_owned(),
                Token::Dot => ".".to_owned(),
                Token::Number(n) => format!("{}", n),
                Token::Boolean(b) => format!("{}", b),
                Token::Symbol(s) => s.to_owned(),
            }
        )
    }
}

pub fn tokenize(input: &str, buffer: &mut Buffer) -> Result<(), SchemeError> {
    for token in input
        .replace('(', " ( ")
        .replace(')', " ) ")
        .replace('[', " ( ")
        .replace(']', " ) ")
        .replace('\'', " ' ")
        // .replace("\"", " \" ")
        .split_ascii_whitespace()
        .map(|token| match token {
            "(" | "[" => Ok(Token::StartExp),
            ")" | "]" => Ok(Token::EndExp),
            "." => Ok(Token::Dot),
            // "'" => Ok(Token::Quote),
            "\"" => todo!(),
            _ => tokenize_symbol(token),
        })
    {
        buffer.push(token?)?
    }
    Ok(())
}

fn tokenize_symbol(token: &str) -> Result<Token, SchemeError> {
    // let special_forms = [
    //     "define", "let", "lambda", "if", "and", "or", "eval", "apply",
    // ];

    if let Some(c) = token.chars().next() {
        if c.is_ascii_digit()
            // Numbers can start with . and -, but make sure it's not the whole token
            || (['.', '-'].contains(&c) && ![".", "-"].contains(&token))
        {
            Ok(Token::Number(token.parse::<i32>().map_err(|_| {
                SchemeError::new("Invalid number literal".to_string())
            })?))
        } else if ["true", "#t"].contains(&token.to_ascii_lowercase().as_str()) {
            Ok(Token::Boolean(true))
        } else if ["false", "#f"].contains(&token.to_ascii_lowercase().as_str()) {
            Ok(Token::Boolean(false))
        // } else if special_forms.contains(&token.to_ascii_lowercase().as_str()) {
        //     Ok(Token::Literal(Value::SpecialForm(SpecialForm::from(token))))
        } else {
            Ok(Token::Symbol(token.to_string()))
        }
    } else {
        Err(SchemeError::new("Tried to parse empty token".to_string()))
    }
}
