use crate::{
    lib::SchemeError,
    types::{rational::Rational, SpecialForm, Value},
};
use std::collections::VecDeque;

#[derive(Clone, Debug, PartialEq)]
pub(crate) enum Token {
    StartExp,
    EndExp,
    Quote,
    Literal(Value),
}

pub(crate) fn tokenize(input: &str) -> Result<VecDeque<Token>, SchemeError> {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .replace("[", " ( ")
        .replace("]", " ) ")
        .replace("'", " ' ")
        .replace("\"", " \" ")
        .split_ascii_whitespace()
        .map(|token| match token {
            "(" | "[" => Ok(Token::StartExp),
            ")" | "]" => Ok(Token::EndExp),
            "'" => Ok(Token::Quote),
            "\"" => todo!(),
            _ => tokenize_str(token),
        })
        .collect()
}

fn tokenize_str(token: &str) -> Result<Token, SchemeError> {
    let special_forms = [
        "define", "let", "lambda", "if", "and", "or", "eval", "apply",
    ];

    if let Some(c) = token.chars().next() {
        if c.is_ascii_digit()
            // Numbers can start with . and -, but make sure it's not the whole token
            || (['.', '-'].contains(&c) && ![".", "-"].contains(&token))
        {
            Ok(Token::Literal(Value::Number(Rational::from(
                token
                    .parse::<f32>()
                    .map_err(|_| SchemeError("Invalid number literal".to_string()))?,
            ))))
        } else if ["true", "#t"].contains(&token.to_ascii_lowercase().as_str()) {
            Ok(Token::Literal(Value::Boolean(true)))
        } else if ["false", "#f"].contains(&token.to_ascii_lowercase().as_str()) {
            Ok(Token::Literal(Value::Boolean(false)))
        } else if special_forms.contains(&token.to_ascii_lowercase().as_str()) {
            Ok(Token::Literal(Value::SpecialForm(SpecialForm::from(token))))
        } else {
            Ok(Token::Literal(Value::Symbol(token.to_string())))
        }
    } else {
        Err(SchemeError("Tried to parse empty token".to_string()))
    }
}
