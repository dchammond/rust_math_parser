#![allow(dead_code)]

use regex::Regex;

#[repr(u32)]
#[derive(Clone)]
pub enum SubToken {
    Plus     = 0,
    Minus    = 1,
    Multiply = 2,
    Divide   = 3,

    Number   = 4,

    LParen   = 5,
    RParen   = 6,

    End      = 7,
}

#[derive(Clone)]
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

pub struct Lexer {
    input: String,
    return_previous_token: bool,
    previous_token: Option<Token>
}

impl Lexer {
    pub fn new(input: String) -> Self {
        Lexer{input: input, return_previous_token: false, previous_token: None}
    }
    pub fn get_next_token(&mut self) -> Token {
        if self.return_previous_token {
            self.return_previous_token = false;
            return self.previous_token.clone().unwrap();
        }
        let strip_input = strip_white_space(self.input.clone());
        lazy_static! {
            static ref PLUS_RE: Regex = Regex::new(r"\A\+").unwrap();
            static ref MINUS_RE: Regex = Regex::new(r"\A-").unwrap();
            static ref MULT_RE: Regex = Regex::new(r"\*").unwrap();
            static ref DIV_RE: Regex = Regex::new(r"\A\/").unwrap();
            static ref NUM_RE: Regex = Regex::new(r"\A\d+(\.\d+)?").unwrap();
            static ref LPAREN_RE: Regex = Regex::new(r"\A\(").unwrap();
            static ref RPAREN_RE: Regex = Regex::new(r"\A\)").unwrap();
        }
        let token: Token;
        if PLUS_RE.is_match(&strip_input[..]) {
            PLUS_RE.replace(&strip_input[..], "");
            token = Token::new(SubToken::Plus, None);
        } else if MINUS_RE.is_match(&strip_input[..]) {
            MINUS_RE.replace(&strip_input[..], "");
            token = Token::new(SubToken::Minus, None);
        } else if MULT_RE.is_match(&strip_input[..]) {
            MULT_RE.replace(&strip_input[..], "");
            token = Token::new(SubToken::Multiply, None);
        } else if DIV_RE.is_match(&strip_input[..]) {
            DIV_RE.replace(&strip_input[..], "");
            token = Token::new(SubToken::Divide, None);
        } else if NUM_RE.is_match(&strip_input[..]) {
            use std::str::FromStr;
            let value: Option<f64> = f64::from_str(NUM_RE.captures(&strip_input[..]).unwrap().at(0).unwrap()).ok();
            NUM_RE.replace(&strip_input[..], "");
            token = Token::new(SubToken::Number, value);
        } else if LPAREN_RE.is_match(&strip_input[..]) {
            LPAREN_RE.replace(&strip_input[..], "");
            token = Token::new(SubToken::LParen, None);
        } else if RPAREN_RE.is_match(&strip_input[..]) {
            RPAREN_RE.replace(&strip_input[..], "");
            token = Token::new(SubToken::RParen, None);
        } else if strip_input.is_empty() {
            token = Token::new(SubToken::End, None);
        } else {
            let mut error: String = String::from("Unknown token: ");
            error.push_str(&strip_input[..]);
            panic!(error);
        }
        self.previous_token = Some(token.clone());
        token
    }
    pub fn revert(&mut self) {
        self.return_previous_token = true;
    }
}

fn strip_white_space(input: String) -> String {
    input.split_whitespace().collect::<Vec<&str>>().join("")
}
