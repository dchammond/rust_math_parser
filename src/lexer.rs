#![allow(dead_code)]

use cfg;

pub struct Lexer {
    input: Option<String>,
    stack: Vec<cfg::value::Value>
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            input: None,
            stack: Vec::default()
        }
    }
    pub fn set_input(&mut self, input: String) {
        self.input = Some(input);
    }
    pub fn transform(&self) -> cfg::value::Value {
        let x = cfg::Value::from(cfg::Terminating::from(cfg::Numeric(6.4)));
        x
    }
}
