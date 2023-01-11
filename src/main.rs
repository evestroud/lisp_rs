use pico_args::Arguments;
use std::process;

use crate::reader::read_eval_print;

mod buffer;
mod environment;
mod error;
mod evaluator;
mod parser;
mod reader;
mod tokenizer;
mod types;

fn main() {
    let mut args = Arguments::from_env();

    let filename = match args.opt_free_from_str() {
        Ok(f) => f,
        Err(e) => {
            eprintln!("{}", e);
            process::exit(1)
        }
    };

    if let Err(e) = read_eval_print(filename) {
        eprintln!("{}", e);
        process::exit(1);
    }
}

#[cfg(test)]
mod test;
