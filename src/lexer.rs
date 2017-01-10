use std::str::FromStr;

use regex::Regex;

use cfg;

pub enum Symbol {
    raw(String),
    atom(cfg::Atom),
}

pub struct Lexer {
    input: Option<String>,
    remain: Vec<Symbol>,
    stack: Vec<Symbol>,
    number: Regex,
    variable: Regex,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            input: None,
            remain: Vec::default(),
            stack: Vec::default(),
            number: Regex::new(r"(+|-)?\d+(\.\d+)?|(\.\d+)").unwrap(),
            variable: Regex::new(r"([a-zA-Z]|_)+").unwrap()
        }
    }
    pub fn set_input(&mut self, input: String) {
        self.input = Some(input);
    }
    pub fn transform(&mut self) -> cfg::Atom {
        let mut val: cfg::Atom;
        if let Some(top) = self.stack.pop() {
            val = match top {
                Symbol::raw(s) => match s.as_ref() {
                    "(" => cfg::Atom::from(cfg::Lsep),
                    ")" => cfg::Atom::from(cfg::Rsep),
                    "=" => cfg::Atom::from(cfg::Equal),
                    "+" => cfg::Atom::from(cfg::Op::Plus),
                    "-" => cfg::Atom::from(cfg::Op::Minus),
                    "/" => cfg::Atom::from(cfg::Op::Divide),
                    "*" => cfg::Atom::from(cfg::Op::Multiply),
                    "^" => cfg::Atom::from(cfg::Op::Power),
                    "%" => cfg::Atom::from(cfg::Op::Modulo),
                    _ => {
                        if self.number.is_match(s.as_ref()) {
                            cfg::Atom::from(cfg::Numeric(f64::from_str(s.as_ref()).unwrap()))
                        } else if self.variable.is_match(s.as_ref()) {
                            cfg::Atom::from(cfg::Variable(s))
                        } else {
                            panic!("String did not match any atom");
                        }
                    }
                },
                Symbol::atom(a) => cfg::Function
            }
        } else {
            panic!("Nothing to pop on stack");
        }
        val
    }
}
