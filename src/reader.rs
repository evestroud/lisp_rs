use wasm_bindgen::prelude::*;

use crate::buffer::Buffer;
use crate::environment::Env;
use crate::evaluator::eval_all;
use crate::parser::parse_all;
use crate::tokenizer::tokenize;
use std::cell::RefCell;
use std::rc::Rc;

#[wasm_bindgen]
pub struct Reader {
    buffer: Buffer,
    env: Rc<RefCell<Env>>,
}

#[wasm_bindgen]
impl Reader {
    pub fn new() -> Self {
        Reader {
            buffer: Buffer::new(),
            env: Rc::new(RefCell::new(Env::new())),
        }
    }

    pub fn push(&mut self, input: String) -> Result<(), String> {
        if let Err(e) = tokenize(&input, &mut self.buffer) {
            return Err(e.to_string());
        }
        Ok(())
    }

    pub fn expression_complete(&self) -> bool {
        self.buffer.expression_complete()
    }

    pub fn new_expression(&self) -> bool {
        self.buffer.len() == 0
    }

    pub fn eval(&mut self) -> Result<String, String> {
        let expression = parse_all(&mut self.buffer).map_err(|e| e.to_string())?;

        let result = eval_all(&expression, &mut self.env).map_err(|e| e.to_string())?;

        Ok(format!("{}", result))
    }

    pub fn clear_buffer(&mut self) {
        self.buffer.clear()
    }
}
