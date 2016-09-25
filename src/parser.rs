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
    pub fn parse(&mut self) -> Result<f64, String> {
        let expression_value: f64 = match self.expression() {
            Ok(v) => v,
            Err(msg) => return Err(msg)
        };
        let token: Token = match self.lexer.get_next_token() {
            Ok(t) => t,
            Err(msg) => return Err(msg)
        };
        match token.get_kind() {
            lexer::SubToken::End => {
                return Ok(expression_value);
            }
            _ => {
                return Err(String::from("End expected"));
            }
        }
    }
    pub fn expression(&mut self) -> Result<f64, String> {
        let mut component1: f64 = match self.factor() {
            Ok(x) => x,
            Err(msg) => return Err(msg)
        };
        let mut token: Token = match self.lexer.get_next_token() {
            Ok(t) => t,
            Err(msg) => return Err(msg)
        };
        loop {
            match token.get_kind() {
                lexer::SubToken::Plus => {
                    let component2 = match self.factor() {
                        Ok(x) => x,
                        Err(msg) => return Err(msg)
                    };
                    component1 += component2;
                    token = match self.lexer.get_next_token() {
                        Ok(t) => t,
                        Err(msg) => return Err(msg)
                    };
                }
                lexer::SubToken::Minus => {
                    let component2 = match self.factor() {
                        Ok(x) => x,
                        Err(msg) => return Err(msg)
                    };
                    component1 -= component2;
                    token = match self.lexer.get_next_token() {
                        Ok(t) => t,
                        Err(msg) => return Err(msg)
                    };
                }
                _ => {
                    break;
                }
            }
        }
        self.lexer.revert();
        Ok(component1)
    }
    pub fn factor(&mut self) -> Result<f64, String> {
        let mut factor1: f64 = match self.number() {
            Ok(x) => x,
            Err(msg) => return Err(msg)
        };
        let mut token: Token = match self.lexer.get_next_token() {
            Ok(t) => t,
            Err(msg) => return Err(msg)
        };
        loop {
            match token.get_kind() {
                lexer::SubToken::Multiply => {
                    let factor2: f64 = match self.number() {
                        Ok(x) => x,
                        Err(msg) => return Err(msg)
                    };
                    factor1 *= factor2;
                    token = match self.lexer.get_next_token() {
                        Ok(t) => t,
                        Err(msg) => return Err(msg)
                    };
                }
                lexer::SubToken::Divide => {
                    let factor2: f64 = match self.number() {
                        Ok(x) => x,
                        Err(msg) => return Err(msg)
                    };
                    factor1 /= factor2;
                    token = match self.lexer.get_next_token() {
                        Ok(t) => t,
                        Err(msg) => return Err(msg)
                    };
                }
                lexer::SubToken::Power => {
                    let factor2: f64 = match self.number() {
                        Ok(x) => x,
                        Err(msg) => return Err(msg)
                    };
                    factor1 = factor1.powf(factor2);
                    token = match self.lexer.get_next_token() {
                        Ok(t) => t,
                        Err(msg) => return Err(msg)
                    };
                }
                _ => {
                    break;
                }
            }
        }
        self.lexer.revert();
        Ok(factor1)
    }
    pub fn number(&mut self) -> Result<f64, String> {
        let token: Token = match self.lexer.get_next_token() {
            Ok(t) => t,
            Err(msg) => return Err(msg)
        };
        let value: f64;
        match token.get_kind() {
            lexer::SubToken::LParen => {
                value = match self.expression() {
                    Ok(v) => v,
                    Err(msg) => return Err(msg)
                };
                match self.lexer.get_next_token() {
                    Err(msg) => return Err(msg),
                    _ => ()
                };
            }
            lexer::SubToken::Number => {
                value = match token.get_value() {
                    Some(v) => v,
                    None => return Err(String::from("No value"))
                };
            }
            _ => {
                return Err(String::from("Not a number"));
            }
        }
        Ok(value)
    }
}