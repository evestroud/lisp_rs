use std::collections::VecDeque;

use crate::{atom::rational::Rational, lib::SchemeError};

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    StartExp,
    EndExp,
    Literal(Literal),
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Literal {
    Number(Rational),
    Symbol(String),
    Boolean(bool),
}

pub(crate) fn tokenize(input: &str) -> Result<VecDeque<Token>, SchemeError> {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_ascii_whitespace()
        .map(|token| match token {
            "(" => Ok(Token::StartExp),
            ")" => Ok(Token::EndExp),
            _ => {
                if let Some(c) = token.chars().next() {
                    if c.is_ascii_digit()
                        || (['.', '-'].contains(&c) && ![".", "-"].contains(&token))
                    {
                        Ok(Token::Literal(Literal::Number(Rational::from(
                            token
                                .parse::<f32>()
                                .map_err(|_| SchemeError("Invalid number literal".to_string()))?,
                        ))))
                    } else if ["true", "#t"].contains(&token.to_ascii_lowercase().as_str()) {
                        Ok(Token::Literal(Literal::Boolean(true)))
                    } else if ["false", "#f"].contains(&token.to_ascii_lowercase().as_str()) {
                        Ok(Token::Literal(Literal::Boolean(false)))
                    } else {
                        Ok(Token::Literal(Literal::Symbol(token.to_string())))
                    }
                } else {
                    Err(SchemeError("Tried to parse empty token".to_string()))
                }
            }
        })
        .collect()
}
