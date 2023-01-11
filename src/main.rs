use pico_args;
use reader::Config;
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
    let mut args = pico_args::Arguments::from_env();

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

fn config_from_args(mut args: pico_args::Arguments) -> Result<Config, pico_args::Error> {
    let filename = args.opt_free_from_str()?;
    let interactive = filename.is_none() || args.contains("-i");
    let verbose = args.contains("-v");

    Ok(Config::new(filename, verbose, interactive))
}

#[cfg(test)]
mod test;
