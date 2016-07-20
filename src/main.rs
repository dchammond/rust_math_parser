extern crate rust_math_parser;

use std::io;
use std::io::Write;

fn main() {
    println!("Welcome to Rust-Calculus!");
    println!("To evaluate an expression, simply type one in and hit RETURN.");
    println!("To set a variable, simply type VAR_NAME=EXPRESSION and hit RETURN.");
    println!("To define a function, simply type FUNC_NAME=EXPRESSION and hit RETURN.");
    println!("Valid commands are: sym_int, int, sym_def, and def.");
    println!("Type 'quit' to exit.");
    let stdin = io::stdin();
    let mut stdout = io::stdout();
    loop {
        let mut input: String = String::new();
        print!(">>>> ");
        stdout.flush().ok();
        if let Err(x) = stdin.read_line(&mut input) {
            panic!(x);
        }
        input = strip_white_space(input);
        if input == "quit" {
            println!("Exiting...");
            break;
        }
        let mut my_parser: rust_math_parser::parser::Parser = rust_math_parser::parser::Parser::new(input);
        match my_parser.parse() {
            Ok(x) => println!("{}", x),
            Err(msg) => println!("{}", msg)
        }
    }
}

fn strip_white_space(input: String) -> String {
    input.split_whitespace().collect::<Vec<&str>>().join("")
}