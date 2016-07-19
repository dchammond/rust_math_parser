mod lexer;
mod parser;

use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to Rust-Calculus!");
    println!("To evaluate an expression, simply type one in and hit RETURN.");
    println!("To set a variable, simply type VAR_NAME=EXPRESSION and hit RETURN.");
    println!("To define a function, simply type FUNC_NAME=EXPRESSION and hit RETURN.");
    println!("Valid commands are: sym_int, int, sym_def, and def.");
    println!("Type 'quit' to exit.");
    let mut input = String::new();
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        input.clear();
        print!(">>>> ");
        stdout.flush().ok();
        stdin.read_line(&mut input);
        let my_parser: parser::Parser = parser::Parser::new(input);
        println("{:?}", my_parser.parse(input));
    }
}