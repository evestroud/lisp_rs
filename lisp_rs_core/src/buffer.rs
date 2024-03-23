use crate::error::SchemeError;
use crate::lexer::{tokenize, Token};
use std::collections::VecDeque;
use std::fmt::Display;

#[derive(Clone, Debug, Default)]
pub struct Buffer {
    open_sexp: i32,
    tokens: VecDeque<Token>,
}

impl Display for Buffer {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "{:?}",
            self.tokens
                .iter()
                .map(|token| token.to_string())
                .collect::<Vec<String>>()
        )
    }
}

impl Buffer {
    pub fn clear(&mut self) {
        self.open_sexp = 0;
        self.tokens = VecDeque::new();
    }

    pub fn len(&self) -> usize {
        self.tokens.len()
    }

    pub fn is_empty(&self) -> bool {
        self.tokens.is_empty()
    }

    pub fn front(&self) -> Option<&Token> {
        self.tokens.front()
    }

    pub fn pop_front(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    pub fn push(&mut self, token: Token) -> Result<(), SchemeError> {
        if token == Token::StartExp {
            self.open_sexp += 1;
        } else if token == Token::EndExp {
            self.open_sexp -= 1;
        }
        if self.open_sexp < 0 {
            return Err(SchemeError::new("Unexpected closing delimiter".to_string()));
        }
        self.tokens.push_back(token);
        Ok(())
    }

    pub fn expression_complete(&self) -> bool {
        self.open_sexp == 0 && self.tokens.is_empty()
    }
}

impl From<&str> for Buffer {
    fn from(s: &str) -> Self {
        let mut b = Buffer::default();
        match tokenize(s, &mut b) {
            Ok(_) => b,
            Err(e) => panic!("Buffer::from failed: {}", e),
        }
    }
}
