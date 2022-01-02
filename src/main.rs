use std::env;
use std::fs;

mod error;
mod scanner;
mod token;

use crate::error::RloxError;
fn main() -> Result<(), RloxError> {
    let args = env::args().skip(1).collect::<Vec<_>>();
    execute(args)
}

pub fn execute(args: Vec<String>) -> Result<(), RloxError> {
    match args.len() {
        l if l > 1 => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        }
        1 => run_file(&args[0]),
        _ => run_repl(),
    }
}

fn run_file(file_path: &str) -> Result<(), RloxError> {
    let data = fs::read_to_string(&file_path)?;
    run(&data)
}

fn run(data: &str) -> Result<(), RloxError> {
    data.chars().for_each(|c| print!("{}", c));
    Ok(())
}

fn run_repl() -> Result<(), RloxError> {
    let stdin = std::io::stdin();
    loop {
        print!("> ");
        let mut buffer = String::new();
        stdin.read_line(&mut buffer)?;
        if buffer == "exit" {
            break Ok(());
        }
        let _result = run(&buffer);
    }
}
