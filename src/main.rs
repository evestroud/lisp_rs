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

    #[derive(Debug)]
    pub(crate) enum Token {
        Start,
        End,
        Num(i32),
        Symbol(String),
    }

    pub(crate) fn parse(mut tokens: VecDeque<String>) -> Result<Vec<Token>, ParseError> {
        if tokens.len() == 0 {
            return Err(ParseError("Unexpected EOF while parsing".to_string()));
        }
        let exp = Vec::new();
        let t = parse_token(&tokens.pop_front().unwrap())?;
        match t {
            // need to define a recursive data structure for Vec<Vec?...<Token>>
            // can enums be recursive ??????? lol
            Token::Start => todo!(),
            Token::End => todo!(),
            Token::Num(_) => todo!(),
            Token::Symbol(_) => todo!(),
        }
        Ok(exp)
    }

    pub(crate) fn parse_token(token: &str) -> Result<Token, ParseError> {
        match token {
            "(" => Ok(Token::Start),
            ")" => Ok(Token::End),
            _ => {
                if let Some(c) = token.chars().next() {
                    if c.is_ascii_digit() || ['.', '-'].contains(&c) {
                        Ok(Token::Num(token.parse().map_err(|_| {
                            ParseError("Invalid number literal".to_string())
                        })?))
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

        print!("{:?}\n", tokenize(&input));
    }
}
