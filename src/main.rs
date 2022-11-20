use std::io::{self, Write};

use crate::{parser::parse, tokenizer::tokenize};

mod tokenizer {
    use std::{collections::VecDeque, fmt};

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
        Int(i32),
        Float(f32),
        Symbol(String),
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
                                Ok(Token::Float(token.parse().map_err(|_| {
                                    SyntaxError("Invalid number literal".to_string())
                                })?))
                            } else {
                                Ok(Token::Int(token.parse().map_err(|_| {
                                    SyntaxError("Invalid number literal".to_string())
                                })?))
                            }
                        } else {
                            Ok(Token::Symbol(token.to_string()))
                        }
                    } else {
                        Err(SyntaxError("Tried to parse empty token".to_string()))
                    }
                }
            })
            .collect()
    }
}

mod parser {
    use std::{collections::VecDeque, fmt};

    use crate::tokenizer::Token;

    #[derive(Debug)]
    pub(crate) struct ParseError(String);

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug)]
    pub(crate) enum Exp {
        SubExp(Vec<Exp>),
        Literal(Token),
    }

    pub(crate) fn parse(tokens: &mut VecDeque<Token>) -> Result<Exp, ParseError> {
        if tokens.len() == 0 {
            return Err(ParseError("Unexpected EOF while parsing".to_string()));
        }
        let t = &tokens.pop_front().unwrap();
        match t {
            Token::StartExp => {
                let mut exp = Vec::new();
                while tokens
                    .front()
                    .ok_or(ParseError("Unexpected EOF while parsing".to_string()))?
                    != &Token::EndExp
                {
                    exp.push(parse(tokens)?);
                }
                tokens.pop_front();
                return Ok(Exp::SubExp(exp));
            }
            Token::EndExp => Err(ParseError("Unmatched ')'".to_string())),
            Token::Int(num) => Ok(Exp::Literal(Token::Int(*num))),
            Token::Float(num) => Ok(Exp::Literal(Token::Float(*num))),
            Token::Symbol(symbol) => Ok(Exp::Literal(Token::Symbol(symbol.to_string()))),
        }
    }
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush();

        let mut input = String::new();

        io::stdin().read_line(&mut input);

        match input.as_str() {
            "" => break,
            "\n" => continue,
            _ => match tokenize(&input) {
                Ok(mut tokens) => match parse(&mut tokens) {
                    Ok(output) => println!("{:?}", output),
                    Err(e) => println!("Parse Error: {}", e),
                },
                Err(e) => println!("Syntax Error: {}", e),
            },
        }
    }
}
