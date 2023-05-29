use lisp_rs::reader::Reader;
use pico_args;
use rustyline::error::ReadlineError;
use rustyline::Editor;
use std::error::Error;
use std::fs;
use std::process;

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

struct Config {
    filename: Option<String>,
    interactive: bool,
    verbose: bool,
}

impl Config {
    fn new(filename: Option<String>, interactive: bool, verbose: bool) -> Self {
        Config {
            filename,
            interactive,
            verbose,
        }
    }
}

fn config_from_args(mut args: pico_args::Arguments) -> Result<Config, pico_args::Error> {
    let filename = args.opt_free_from_str()?;
    let interactive = filename.is_none() || args.contains("-i");
    let verbose = args.contains("-v");

    Ok(Config::new(filename, verbose, interactive))
}

fn read_eval_print(config: Config) -> Result<(), Box<dyn Error>> {
    let mut reader = Reader::new();

    read_from_file(String::from("std.scm"), &mut reader)?;
    if let Some(f) = config.filename {
        read_from_file(f, &mut reader)?;
    }

    let mut rl = Editor::<()>::new()?;
    'repl: loop {
        while !reader.expression_complete() {
            let readline = match reader.new_expression() {
                true => rl.readline("> "),
                false => rl.readline(". "),
            };
            match readline {
                Ok(line) => {
                    if line == "" {
                        continue;
                    }
                    rl.add_history_entry(line.as_str());

                    if let Err(error) = reader.push(line) {
                        println!("{:?}", error);
                    }
                }

                Err(ReadlineError::Interrupted) => continue 'repl,
                Err(ReadlineError::Eof) => break 'repl Ok(()),
                Err(err) => {
                    println!("Error: {:?}", err);
                    continue 'repl;
                }
            }
        }

        match reader.eval() {
            Ok(result) => println!("{}", result),
            Err(error) => println!("{:?}", error),
        }

        rl.save_history("history.txt")?;
    }
}

fn read_from_file(f: String, reader: &mut Reader) -> Result<(), Box<dyn Error>> {
    let contents = fs::read_to_string(f)?;
    reader.push(contents)?;
    reader.eval()?;
    Ok(())
}

#[cfg(test)]
mod test;
