use std::env;
use std::fmt::{self, Display, Formatter};
use std::fs;

#[derive(Debug)]
enum RloxError {
    IoError(std::io::Error),
}

impl From<std::io::Error> for RloxError {
    fn from(e: std::io::Error) -> Self {
        Self::IoError(e)
    }
}

impl Display for RloxError {
    fn fmt(&self, f: &mut Formatter<'_>) -> fmt::Result {
        use RloxError::*;
        match self {
            IoError(e) => write!(f, "error reading script: {}", e),
        }
    }
}

fn main() -> Result<(), RloxError> {
    let args = env::args().skip(1).collect::<Vec<_>>();
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
    println!("{}", data);
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
        run(&buffer);
    }
}
