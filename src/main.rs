use brainfuck;
use brainfuck::BrainfuckError;
use clap::Parser;

#[derive(Parser)]
struct Input {
    path: std::path::PathBuf
}

fn main() {
    let args = Input::parse();
    let content = match std::fs::read_to_string(&args.path) {
        Ok(str) => str,
        Err(e) => panic!("{}", e)
    };
    let mut tape = vec![0; 1024];
    let mut pointer: usize = 0;
    
    brainfuck::run(content, &mut tape, &mut pointer);
}

fn handle_err(err: Result<(), brainfuck::BrainfuckError>) {
    match err {
        Err(BrainfuckError::LexError(a)) => println!("LexError: {}", a),
        Err(BrainfuckError::ParseError(a)) => println!("ParseError: {}", a),
        Err(BrainfuckError::ExecuteError(a)) => println!("ExecuteError: {}", a),
        Ok(_) => ()
    };
}
