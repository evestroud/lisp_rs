use std::{env, process};

mod buffer;
mod environment;
mod error;
mod evaluator;
mod parser;
mod reader;
mod tokenizer;
mod types;

fn main() {
    let filename = env::args().nth(1);

    if let Err(e) = reader::read_eval_print(filename) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

#[cfg(test)]
mod test;
