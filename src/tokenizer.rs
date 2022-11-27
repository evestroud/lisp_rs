use std::{collections::VecDeque, fmt};

use crate::atom::{Atom, Rational};

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
                    if c.is_ascii_digit()
                        || (['.', '-'].contains(&c) && ![".", "-"].contains(&token))
                    {
                        Ok(Token::Literal(Atom::Number(Rational::from(
                            token
                                .parse::<f32>()
                                .map_err(|_| SyntaxError("Invalid number literal".to_string()))?,
                        ))))
                    } else {
                        match token {
                            "define" => {
                                return Ok(Token::Literal(Atom::SpecialForm(
                                    crate::atom::SpecialForm::Define,
                                )))
                            }
                            "let" => {
                                return Ok(Token::Literal(Atom::SpecialForm(
                                    crate::atom::SpecialForm::Let,
                                )))
                            }
                            _ => return Ok(Token::Literal(Atom::Symbol(token.to_string()))),
                        }
                    }
                } else {
                    Err(SyntaxError("Tried to parse empty token".to_string()))
                }
            }
        })
        .collect()
}
