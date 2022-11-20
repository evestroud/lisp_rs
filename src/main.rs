use std::io::{self, Write};

use crate::{parser::parse, tokenizer::tokenize};

mod tokenizer;

mod parser;

fn main() {
    loop {
        print!("> ");
        io::stdout().flush();

        let mut input = String::new();

        io::stdin().read_line(&mut input);

        match input.as_str() {
            "" => break,
            "\n" => continue,
            _ => match tokenize(&input) {
                Ok(mut tokens) => match parse(&mut tokens) {
                    Ok(output) => println!("{:?}", output),
                    Err(e) => println!("Parse Error: {}", e),
                },
                Err(e) => println!("Syntax Error: {}", e),
            },
        }
    }
}
