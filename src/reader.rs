use crate::buffer::Buffer;
use crate::environment::Env;
use crate::error::SchemeError;
use crate::evaluator::eval_all;
use crate::parser::parse_all;
use crate::tokenizer::tokenize;
use std::cell::RefCell;
use std::rc::Rc;

pub struct Reader {
    buffer: Buffer,
    env: Rc<RefCell<Env>>,
}

impl Reader {
    pub fn new() -> Self {
        Reader {
            buffer: Buffer::new(),
            env: Rc::new(RefCell::new(Env::new())),
        }
    }

    pub fn push(&mut self, input: String) -> Result<(), SchemeError> {
        tokenize(&input, &mut self.buffer)
    }

    pub fn expression_complete(&self) -> bool {
        self.buffer.expression_complete()
    }

    pub fn new_expression(&self) -> bool {
        self.buffer.len() == 0
    }

    pub fn eval(&mut self) -> Result<String, SchemeError> {
        let expression = parse_all(&mut self.buffer)?;

        let result = eval_all(&expression, &mut self.env)?;

        Ok(format!("{}", result))
    }
}
