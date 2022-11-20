use std::{
    collections::VecDeque,
    io::{self, Write},
};

fn tokenize(input: &str) -> VecDeque<String> {
    input
        .replace("(", " ( ")
        .replace(")", " ) ")
        .split_ascii_whitespace()
        .map(|token| token.to_string())
        .collect()
}

mod parser {
    use std::{collections::VecDeque, fmt};

    #[derive(Debug)]
    pub(crate) struct ParseError(String);

    impl fmt::Display for ParseError {
        fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
            write!(f, "{}", self.0)
        }
    }

    #[derive(Debug, PartialEq)]
    pub(crate) enum Token {
        Start,
        End,
        Int(i32),
        Float(f32),
        Symbol(String),
    }

    #[derive(Debug)]
    pub(crate) enum Exp {
        SubExp(Vec<Exp>),
        Literal(Token),
    }

    pub(crate) fn parse(tokens: &mut VecDeque<String>) -> Result<Exp, ParseError> {
        if tokens.len() == 0 {
            return Err(ParseError("Unexpected EOF while parsing".to_string()));
        }
        let t = parse_token(&tokens.pop_front().unwrap())?;
        match t {
            Token::Start => {
                let mut exp = Vec::new();
                while parse_token(
                    tokens
                        .front()
                        .ok_or(ParseError("Unexpected EOF while parsing".to_string()))?,
                )? != Token::End
                {
                    exp.push(parse(tokens)?);
                }
                tokens.pop_front();
                return Ok(Exp::SubExp(exp));
            }
            Token::End => Err(ParseError("Unmatched ')'".to_string())),
            Token::Int(num) => Ok(Exp::Literal(Token::Int(num))),
            Token::Float(num) => Ok(Exp::Literal(Token::Float(num))),
            Token::Symbol(symbol) => Ok(Exp::Literal(Token::Symbol(symbol))),
        }
    }

    pub(crate) fn parse_token(token: &str) -> Result<Token, ParseError> {
        match token {
            "(" => Ok(Token::Start),
            ")" => Ok(Token::End),
            _ => {
                if let Some(c) = token.chars().next() {
                    if c.is_ascii_digit() || ['.', '-'].contains(&c) {
                        if token.contains('.') {
                            Ok(Token::Float(token.parse().map_err(|_| {
                                ParseError("Invalid number literal".to_string())
                            })?))
                        } else {
                            Ok(Token::Int(token.parse().map_err(|_| {
                                ParseError("Invalid number literal".to_string())
                            })?))
                        }
                    } else {
                        Ok(Token::Symbol(token.to_string()))
                    }
                } else {
                    Err(ParseError("Tried to parse empty token".to_string()))
                }
            }
        }
    }
}

fn main() {
    loop {
        print!("> ");
        io::stdout().flush();

        let mut input = String::new();

        io::stdin().read_line(&mut input);

        print!("{:?}\n", parser::parse(&mut tokenize(&input)));
    }
}
