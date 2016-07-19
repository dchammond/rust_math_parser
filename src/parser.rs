#![allow(dead_code)]

use lexer;

pub type Token = lexer::Token;
pub type Lexer = lexer::Lexer;

pub struct Parser {
    lexer: Lexer
}

impl Parser {
    pub fn new(input: String) -> Self {
        Parser{lexer: Lexer::new(input)}
    }
    pub fn parse(&mut self) -> f64 {
        let expression_value: f64 = self.expression();
        let token: Token = self.lexer.get_next_token();
        match token.get_kind() {
            lexer::SubToken::End => {
                return expression_value;
            }
            _ => {
                panic!("End expected");
            }
        }
    }
    pub fn expression(&mut self) -> f64 {
        let mut component1: f64 = self.factor();
        let mut token: Token = self.lexer.get_next_token();
        loop {
            match token.get_kind() {
                lexer::SubToken::Plus => {
                    let component2 = self.factor();
                    component1 += component2;
                    token = self.lexer.get_next_token();
                }
                lexer::SubToken::Minus => {
                    let component2 = self.factor();
                    component1 -= component2;
                    token = self.lexer.get_next_token();
                }
                _ => {
                    break;
                }
            }
        }
        self.lexer.revert();
        component1
    }
    pub fn factor(&mut self) -> f64 {
        let mut factor1: f64 = self.number();
        let mut token: Token = self.lexer.get_next_token();
        loop {
            match token.get_kind() {
                lexer::SubToken::Multiply => {
                    let factor2: f64 = self.number();
                    factor1 *= factor2;
                    token = self.lexer.get_next_token();
                }
                lexer::SubToken::Divide => {
                    let factor2: f64 = self.number();
                    factor1 /= factor2;
                    token = self.lexer.get_next_token();
                }
                _ => {
                    break;
                }
            }
        }
        self.lexer.revert();
        factor1
    }
    pub fn number(&mut self) -> f64 {
        let token: Token = self.lexer.get_next_token();
        let value: f64;
        match token.get_kind() {
            lexer::SubToken::LParen => {
                value = self.expression();
                self.lexer.get_next_token();
            }
            lexer::SubToken::Number => {
                value = token.get_value().unwrap();
            }
            _ => {
                panic!("Not a number");
            }
        }
        value
    }
}