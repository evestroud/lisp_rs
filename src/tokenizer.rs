use std::{collections::VecDeque, fmt};

use crate::atom::Atom;

#[derive(Debug)]
pub(crate) struct SyntaxError(String);

impl fmt::Display for SyntaxError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, PartialEq)]
pub(crate) enum Token {
    StartExp,
    EndExp,
    Literal(Atom),
}

pub(crate) fn tokenize(input: &str) -> Result<VecDeque<Token>, SyntaxError> {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_ascii_whitespace()
        .map(|token| match token {
            "(" => Ok(Token::StartExp),
            ")" => Ok(Token::EndExp),
            _ => {
                if let Some(c) = token.chars().next() {
                    if c.is_ascii_digit() || ['.', '-'].contains(&c) {
                        if token.contains('.') {
                            Ok(Token::Literal(Atom::Float(token.parse().map_err(
                                |_| SyntaxError("Invalid number literal".to_string()),
                            )?)))
                        } else {
                            Ok(Token::Literal(Atom::Int(token.parse().map_err(|_| {
                                SyntaxError("Invalid number literal".to_string())
                            })?)))
                        }
                    } else {
                        Ok(Token::Literal(Atom::Symbol(token.to_string())))
                    }
                } else {
                    Err(SyntaxError("Tried to parse empty token".to_string()))
                }
            }
        })
        .collect()
}
