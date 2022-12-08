use crate::environment::Env;
use crate::{evaluator::eval_all, parser::parse_all, tokenizer::tokenize};
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

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                if line == "" {
                    continue;
                }
                rl.add_history_entry(line.as_str());

                match tokenize(&line) {
                    Ok(mut tokens) => {
                        // println!("{:?}", tokens);
                        match parse_all(&mut tokens) {
                            Ok(exp) => {
                                // println!("{:?}", exp);
                                match eval_all(&exp, &mut env) {
                                    Ok(result) => {
                                        println!("{}", result);
                                    }
                                    Err(e) => println!("Evaluation Error: {}", e),
                                }
                            }
                            Err(e) => println!("Parse Error: {}", e),
                        }
                    }
                    Err(e) => println!("Syntax Error: {}", e),
                }
            }

            // TODO - when multiline input is available - C-c clears buffer
            Err(ReadlineError::Interrupted) => continue,
            Err(ReadlineError::Eof) => break,
            Err(err) => {
                println!("Error: {:?}", err);
                break;
            }
        }
    }
    rl.save_history("history.txt")
}

#[cfg(test)]
mod test;
