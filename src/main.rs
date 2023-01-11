use crate::environment::Env;
use crate::{evaluator::eval_all, parser::parse_all, tokenizer::tokenize};
use buffer::Buffer;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::error::Error;
use std::{cell::RefCell, rc::Rc};
use std::{env, fs};

mod buffer;
mod environment;
mod error;
mod evaluator;
mod parser;
mod tokenizer;
mod types;

fn main() {
    let filename = env::args().nth(1);

    if let Err(e) = read_eval_print(filename) {
        eprintln!("{}", e);
    }
}

fn read_eval_print(filename: Option<String>) -> Result<(), Box<dyn Error>> {
    let mut env = Rc::new(RefCell::new(Env::new()));

    read_from_file(String::from("std.scm"), &mut env)?;

    if let Some(f) = filename {
        read_from_file(f, &mut env)?;
    }

    let mut rl = Editor::<()>::new()?;

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
        // println!("{:#?}", expression);

        let result;
        match eval_all(&expression, &mut env) {
            Ok(res) => result = res,
            Err(e) => {
                println!("Runtime Error: {}", e);
                continue;
            }
        }
        // println!("{:?}", result);
        println!("{}", result);

        rl.save_history("history.txt")?;
    }
}

fn read_from_file(f: String, env: &mut Rc<RefCell<Env>>) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(f)?;
    let mut b = Buffer::from(contents.as_str());
    let e = parse_all(&mut b)?;
    eval_all(&e, env)?;
    Ok(())
}

#[cfg(test)]
mod test;
