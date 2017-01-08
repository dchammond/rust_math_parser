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
    Modulo,

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

#[allow(non_snake_case)]
#[derive(Debug)]
pub struct Lexer {
    input: Option<String>,
    return_previous_token: bool,
    previous_token: Option<Token>,
    PLUS_RE: Regex,
    MINUS_RE: Regex,
    MULT_RE: Regex,
    DIV_RE: Regex,
    POW_RE: Regex,
    MOD_RE: Regex,
    NUM_RE: Regex,
    LPAREN_RE: Regex,
    RPAREN_RE: Regex
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            input: None,
            return_previous_token: false,
            previous_token: None,
            PLUS_RE: Regex::new(r"\A\+").unwrap(),
            MINUS_RE: Regex::new(r"\A-").unwrap(),
            MULT_RE: Regex::new(r"\A\*").unwrap(),
            DIV_RE: Regex::new(r"\A/").unwrap(),
            POW_RE: Regex::new(r"\A\^").unwrap(),
            MOD_RE: Regex::new(r"\A%").unwrap(),
            NUM_RE: Regex::new(r"(\A\d+(\.\d+)?)|(\A(\.\d+))").unwrap(),
            LPAREN_RE: Regex::new(r"\A\(").unwrap(),
            RPAREN_RE: Regex::new(r"\A\)").unwrap()
        }
    }
    pub fn set_input(&mut self, input: String) {
        self.input = Some(input);
    }
    pub fn get_next_token(&mut self) -> Result<Token, String> {
        if self.return_previous_token {
            self.return_previous_token = false;
            return match self.previous_token.clone() {
                Some(t) => Ok(t),
                None => Err(String::from("No previous token"))
            }
        }
        let mut strip_input = self.input.clone().unwrap();
        let token: Token;
        let temp = &(strip_input.clone())[..];
        if self.PLUS_RE.is_match(temp) {
            strip_input = self.PLUS_RE.replace(temp, "").into_owned();
            token = Token::new(SubToken::Plus, None);
        } else if self.MINUS_RE.is_match(temp) {
            strip_input = self.MINUS_RE.replace(temp, "").into_owned();
            token = Token::new(SubToken::Minus, None);
        } else if self.MULT_RE.is_match(temp) {
            strip_input = self.MULT_RE.replace(temp, "").into_owned();
            token = Token::new(SubToken::Multiply, None);
        } else if self.DIV_RE.is_match(temp) {
            strip_input = self.DIV_RE.replace(temp, "").into_owned();
            token = Token::new(SubToken::Divide, None);
        } else if self.POW_RE.is_match(temp) {
            strip_input = self.POW_RE.replace(temp, "").into_owned();
            token = Token::new(SubToken::Power, None);
        } else if self.MOD_RE.is_match(temp) {
            strip_input = self.MOD_RE.replace(temp, "").into_owned();
            token = Token::new(SubToken::Modulo, None);
        } else if self.NUM_RE.is_match(temp) {
            use std::str::FromStr;
            let value: Option<f64> = f64::from_str(self.NUM_RE.captures(temp).unwrap().get(0).unwrap().as_str()).ok();
            strip_input = self.NUM_RE.replace(temp, "").into_owned();
            token = Token::new(SubToken::Number, value);
        } else if self.LPAREN_RE.is_match(temp) {
            strip_input = self.LPAREN_RE.replace(temp, "").into_owned();
            token = Token::new(SubToken::LParen, None);
        } else if self.RPAREN_RE.is_match(temp) {
            strip_input = self.RPAREN_RE.replace(temp, "").into_owned();
            token = Token::new(SubToken::RParen, None);
        } else if strip_input.is_empty() {
            token = Token::new(SubToken::End, None);
        } else {
            let mut error: String = String::from("Unknown token: ");
            error.push_str(temp);
            return Err(error);
        }
        self.input = Some(strip_input);
        self.previous_token = Some(token.clone());
        Ok(token)
    }
    pub fn revert(&mut self) {
        self.return_previous_token = true;
    }
}
