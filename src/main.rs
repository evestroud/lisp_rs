use pico_args;
use reader::{read_eval_print, Config};
use std::process;

mod buffer;
mod environment;
mod error;
mod evaluator;
mod parser;
mod reader;
mod tokenizer;
mod types;

fn main() {
    match config_from_args(pico_args::Arguments::from_env()) {
        Ok(config) => {
            if let Err(error) = read_eval_print(config) {
                eprintln!("{}", error);
                process::exit(1);
            }
        }
        Err(error) => {
            eprintln!("{}", error);
            process::exit(1);
        }
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
