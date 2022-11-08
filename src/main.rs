use brainfuck;
use brainfuck::BrainfuckError;

fn main() {
    let mut tape = vec![0; 1024];
    let mut pointer: usize = 0;
    handle_err(brainfuck::run("+++++++++++++++++++++++++++++++++.++++++++++.][".to_string(), & mut tape, &mut pointer));

    handle_err(brainfuck::run("++++++++++++++++++.".to_owned(), &mut tape, &mut pointer));
}

fn handle_err(err: Result<(), brainfuck::BrainfuckError>) {
    match err {
        Err(BrainfuckError::LexError(a)) => println!("LexError: {}", a),
        Err(BrainfuckError::ParseError(a)) => println!("ParseError: {}", a),
        Err(BrainfuckError::ExecuteError(a)) => println!("ExecuteError: {}", a),
        Ok(_) => ()
    };
}
