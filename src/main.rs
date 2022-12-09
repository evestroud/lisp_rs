use crate::environment::Env;
use crate::{evaluator::eval_all, parser::parse_all, tokenizer::tokenize};
use buffer::Buffer;
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};
use std::{cell::RefCell, rc::Rc};

mod buffer;
mod environment;
mod evaluator;
mod lib;
mod parser;
mod tokenizer;
mod types;

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;
    let mut env = Rc::new(RefCell::new(Env::new()));

    'repl: loop {
        let mut buffer = Buffer::new();
        while !buffer.expression_complete() {
            let readline = match buffer.len() {
                0 => rl.readline("> "),
                _ => rl.readline(". "),
            };
            match readline {
                Ok(line) => {
                    if line == "" {
                        continue;
                    }
                    rl.add_history_entry(line.as_str());

                    if let Err(e) = tokenize(&line, &mut buffer) {
                        println!("Syntax Error: {}", e);
                        break;
                    }
                    // println!("{:?}", buffer);
                }

                Err(ReadlineError::Interrupted) => continue 'repl,
                Err(ReadlineError::Eof) => break 'repl Ok(()),
                Err(err) => {
                    println!("Error: {:?}", err);
                    continue 'repl;
                }
            }
        }
        // println!("{:?}", buffer);

        let expression;
        match parse_all(&mut buffer) {
            Ok(exp) => expression = exp,
            Err(e) => {
                println!("Parse Error: {}", e);
                continue;
            }
        }
        // println!("{:?}", expression);

        let result;
        match eval_all(&expression, &mut env) {
            Ok(res) => result = res,
            Err(e) => {
                println!("Runtime Error: {}", e);
                continue;
            }
        }
        println!("{}", result);
        rl.save_history("history.txt")?;
    }
}

#[cfg(test)]
mod test;
