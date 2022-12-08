use crate::lib::SchemeError;
use crate::tokenizer::Token;
use std::collections::VecDeque;

#[derive(Clone, Debug)]
struct Buffer {
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

    pub(crate) fn push(&mut self, token: Token) -> Result<(), SchemeError> {
        if token == Token::StartExp {
            self.open_sexp += 1;
        } else if token == Token::EndExp {
            self.open_sexp -= 1;
        }
        if self.open_sexp < 0 {
            return Err(SchemeError("Unexpected ')'".to_string()));
        }
        self.tokens.push_back(token);
        Ok(())
    }

    pub(crate) fn expression_complete(&self) -> bool {
        self.open_sexp == 0 && self.tokens.len() > 0
    }
}
