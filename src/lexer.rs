#![allow(dead_code)]

use regex::Regex;

#[repr(u32)]
#[derive(Debug, Clone)]
pub enum SubToken {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,

    Number,

    LParen,
    RParen,

    End
}

#[derive(Debug, Clone)]
pub struct Token {
    kind: SubToken,
    value: Option<f64>
}

impl Token {
    pub fn new(kind: SubToken, value: Option<f64>) -> Self {
        Token{kind: kind, value: value}
    }
    pub fn set_kind(&mut self, kind: SubToken) {
        self.kind = kind;
    }
    pub fn set_value(&mut self, value: Option<f64>) {
        self.value = value;
    }
    pub fn get_kind(&self) -> SubToken {
        self.kind.clone()
    }
    pub fn get_value(&self) -> Option<f64> {
        self.value.clone()
    }
}

#[derive(Debug)]
pub struct Lexer {
    input: String,
    return_previous_token: bool,
    previous_token: Option<Token>
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer{input: input, return_previous_token: false, previous_token: None}
    }
    pub fn get_next_token(&mut self) -> Result<Token, String> {
        if self.return_previous_token {
            self.return_previous_token = false;
            return match self.previous_token.clone() {
                Some(t) => Ok(t),
                None => Err(String::from("No previous token"))
            }
        }
        let mut strip_input = strip_white_space(self.input.clone());
        lazy_static! {
            static ref NEG_RE: Regex = Regex::new(r"\A-\d+(\.\d+)?").unwrap();
            static ref PLUS_RE: Regex = Regex::new(r"\A\+").unwrap();
            static ref MINUS_RE: Regex = Regex::new(r"\A-").unwrap();
            static ref MULT_RE: Regex = Regex::new(r"\A\*").unwrap();
            static ref DIV_RE: Regex = Regex::new(r"\A/").unwrap();
            static ref POW_RE: Regex = Regex::new(r"\A\^").unwrap();
            static ref NUM_RE: Regex = Regex::new(r"\A\d+(\.\d+)?").unwrap();
            static ref LPAREN_RE: Regex = Regex::new(r"\A\(").unwrap();
            static ref RPAREN_RE: Regex = Regex::new(r"\A\)").unwrap();
        }
        let token: Token;
        let temp = &(strip_input.clone())[..];
        if PLUS_RE.is_match(temp) {
            strip_input = PLUS_RE.replace(temp, "");
            token = Token::new(SubToken::Plus, None);
        } else if MINUS_RE.is_match(temp) {
            strip_input = MINUS_RE.replace(temp, "");
            token = Token::new(SubToken::Minus, None);
        } else if MULT_RE.is_match(temp) {
            strip_input = MULT_RE.replace(temp, "");
            token = Token::new(SubToken::Multiply, None);
        } else if DIV_RE.is_match(temp) {
            strip_input = DIV_RE.replace(temp, "");
            token = Token::new(SubToken::Divide, None);
        } else if POW_RE.is_match(temp) {
            strip_input = POW_RE.replace(temp, "");
            token = Token::new(SubToken::Power, None);
        } else if NEG_RE.is_match(temp) {
            use std::str::FromStr;
            let value: Option<f64> = f64::from_str(NEG_RE.captures(temp).unwrap().at(0).unwrap()).ok();
            strip_input = NEG_RE.replace(temp, "");
            token = Token::new(SubToken::Number, value);
        } else if NUM_RE.is_match(temp) {
            use std::str::FromStr;
            let value: Option<f64> = f64::from_str(NUM_RE.captures(temp).unwrap().at(0).unwrap()).ok();
            strip_input = NUM_RE.replace(temp, "");
            token = Token::new(SubToken::Number, value);
        } else if LPAREN_RE.is_match(temp) {
            strip_input = LPAREN_RE.replace(temp, "");
            token = Token::new(SubToken::LParen, None);
        } else if RPAREN_RE.is_match(temp) {
            strip_input = RPAREN_RE.replace(temp, "");
            token = Token::new(SubToken::RParen, None);
        } else if strip_input.is_empty() {
            token = Token::new(SubToken::End, None);
        } else {
            let mut error: String = String::from("Unknown token: ");
            error.push_str(temp);
            return Err(error);
        }
        self.input = strip_input;
        self.previous_token = Some(token.clone());
        println!("{:?}", token);
        Ok(token)
    }
    pub fn revert(&mut self) {
        self.return_previous_token = true;
    }
}

fn strip_white_space(input: String) -> String {
    input.split_whitespace().collect::<Vec<&str>>().join("")
}
