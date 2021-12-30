use std::env;

fn main() {
    let args = env::args().skip(1).collect::<Vec<_>>();
    match args.len() {
        l if l > 1 => {
            println!("Usage: rlox [script]");
            std::process::exit(64);
        }
        1 => run_file(&args[0]),
        _ => run_prompt(),
    }
}

fn run_file(file_path: &str) {
    todo!()
}

fn run_prompt() {
    todo!()
}
