use crate::{evaluator::eval_all, parser::parse_all, tokenizer::tokenize};
use rustyline::error::ReadlineError;
use rustyline::{Editor, Result};

mod atom;
mod builtin;
mod environment;
mod evaluator;
mod parser;
mod tokenizer;

fn main() -> Result<()> {
    let mut rl = Editor::<()>::new()?;

    let mut env = environment::Env::new();

    loop {
        let readline = rl.readline("> ");
        match readline {
            Ok(line) => {
                if line == "" {
                    continue;
                }
                rl.add_history_entry(line.as_str());
                match tokenize(&line) {
                    Ok(mut tokens) => match parse_all(&mut tokens) {
                        Ok(expression) => match eval_all(&expression, &mut env) {
                            Ok(output) => match output.to_string().as_str() {
                                "Nil" => (),
                                _ => println!("{}", output),
                            },
                            Err(e) => println!("Evaluation Error: {}", e),
                        },
                        Err(e) => println!("Parse Error: {}", e),
                    },
                    Err(e) => println!("Syntax Error: {}", e),
                }
            }
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
