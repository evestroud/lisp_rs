use crate::atom::SchemeExp;
use crate::atom::{Atom, SpecialForm};
use crate::lib::SchemeError;
use crate::tokenizer::{Literal, Token};
use std::collections::VecDeque;

pub(crate) fn parse_all(tokens: &mut VecDeque<Token>) -> Result<Vec<SchemeExp>, SchemeError> {
    let mut result = Vec::new();
    while tokens.len() != 0 {
        result.push(parse(tokens)?);
    }
    Ok(result)
}

pub(crate) fn parse(tokens: &mut VecDeque<Token>) -> Result<SchemeExp, SchemeError> {
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
            return Ok(SchemeExp::List(exp));
        }
        Token::EndExp => Err(SchemeError("Unmatched ')'".to_string())),
        Token::Quote => Ok(SchemeExp::Atom(Atom::Quote(Box::from(parse(tokens)?)))),
        Token::Literal(atom) => match atom {
            Literal::Number(num) => Ok(SchemeExp::Atom(Atom::Number(num.clone()))),
            Literal::Symbol(symbol) => Ok(SchemeExp::Atom(
                match symbol.to_ascii_lowercase().as_str() {
                    "nil" => Atom::Nil,
                    "let" => Atom::SpecialForm(SpecialForm::Let),
                    "define" => Atom::SpecialForm(SpecialForm::Define),
                    "lambda" => Atom::SpecialForm(SpecialForm::Lambda),
                    "if" => Atom::SpecialForm(SpecialForm::If),
                    "and" => Atom::SpecialForm(SpecialForm::And),
                    "or" => Atom::SpecialForm(SpecialForm::Or),
                    "eval" => Atom::SpecialForm(SpecialForm::Eval),
                    "apply" => Atom::SpecialForm(SpecialForm::Apply),
                    _ => Atom::Symbol(symbol.to_string()),
                },
            )),
            Literal::Boolean(b) => Ok(SchemeExp::Atom(Atom::Boolean(*b))),
        },
        Token::StringDelim => Err(SchemeError("Strings not implemented yet".to_string())),
    }
}
