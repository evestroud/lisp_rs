use crate::lib::SchemeError;
use crate::tokenizer::{tokenize, Token};
use std::collections::VecDeque;

#[derive(Clone, Debug)]
pub(crate) struct Buffer {
    open_sexp: i32,
    tokens: VecDeque<Token>,
}

impl Buffer {
    pub(crate) fn new() -> Self {
        Self {
            open_sexp: 0,
            tokens: VecDeque::new(),
        }
    }

    pub(crate) fn len(&self) -> usize {
        self.tokens.len()
    }

    pub(crate) fn front(&self) -> Option<&Token> {
        self.tokens.front()
    }

    pub(crate) fn pop_front(&mut self) -> Option<Token> {
        self.tokens.pop_front()
    }

    pub(crate) fn push(&mut self, token: Token) -> Result<(), SchemeError> {
        if token == Token::StartExp {
            self.open_sexp += 1;
        } else if token == Token::EndExp {
            self.open_sexp -= 1;
        }
        if self.open_sexp < 0 {
            return Err(SchemeError("Unexpected closing delimiter".to_string()));
        }
        self.tokens.push_back(token);
        Ok(())
    }

    pub(crate) fn expression_complete(&self) -> bool {
        self.open_sexp == 0 && self.tokens.len() > 0
    }
}

impl From<&str> for Buffer {
    fn from(s: &str) -> Self {
        let mut b = Buffer::new();
        match tokenize(&s, &mut b) {
            Ok(_) => b,
            Err(e) => panic!("Buffer::from failed: {}", e),
        }
    }
}
