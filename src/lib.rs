extern crate regex;
#[macro_use]
extern crate lazy_static;

pub mod lexer;
pub mod parser;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        let input: String = String::from("1+2");
        let mut my_parser: super::parser::Parser = super::parser::Parser::new(input);
        println!("{:?}", my_parser.parse());
    }
}
