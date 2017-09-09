use std::str::FromStr;

use regex::Regex;

use cfg;

pub enum Symbol {
    Raw(String),
    Atom(cfg::Atom),
}

pub struct Lexer {
    input: Option<String>,
    stack: Vec<Symbol>,
    number: Regex,
    variable: Regex,
}

impl Lexer {
    pub fn new() -> Self {
        Lexer {
            input: None,
            stack: Vec::default(),
            number: Regex::new(r"(+|-)?\d+(\.\d+)?|(\.\d+)").unwrap(),
            variable: Regex::new(r"([a-zA-Z]|_)+").unwrap()
        }
    }
    pub fn set_input(&mut self, input: String) {
        self.input = Some(input);
    }
    pub fn push_input_to_stack(mut self) {
        self.stack.push(Symbol::Raw(self.input.unwrap()))
    }
    pub fn transform(&self, stack: Vec<Symbol>, top: Symbol) -> Vec<Symbol> {
        let mut rev_stack = stack;
        rev_stack.reverse();
        let mut val: cfg::Atom;
        val = match top {
            Symbol::Raw(s) => match s.as_ref() {
                "(" => cfg::Atom::from(cfg::Lsep),
                ")" => cfg::Atom::from(cfg::Rsep),
                "=" => cfg::Atom::from(cfg::Equal),
                "+" => cfg::Atom::from(cfg::Op::Plus),
                "-" => cfg::Atom::from(cfg::Op::Minus),
                "/" => cfg::Atom::from(cfg::Op::Divide),
                "*" => cfg::Atom::from(cfg::Op::Multiply),
                "^" => cfg::Atom::from(cfg::Op::Power),
                "%" => cfg::Atom::from(cfg::Op::Modulo),
                "," => cfg::Atom::from(cfg::Msep),
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
            Symbol::Atom(a) => match a {
                cfg::Atom::Numeric(n) => cfg::Atom::from(cfg::Expression::from(n)),
                cfg::Atom::Variable(v) => cfg::Atom::from(cfg::Expression::from(v)),
                cfg::Atom::Function(f) => cfg::Atom::from(cfg::Expression::from(f)),
                cfg::Atom::Lsep(r) => {
                    let next_two = (rev_stack.pop().unwrap(), rev_stack.pop().unwrap());
                    match next_two {
                        (Symbol::Atom(left), Symbol::Atom(right)) => panic!("")
                    }
                },
                // match rev_stack.pop().unwrap() {
                //     Symbol::Atom(inner_a) => match inner_a {
                //         cfg::Atom::Expression(expr) => panic!("")
                //     },
                    // Symbol::Raw(raw) => panic!("There was a string inside the stack")
            }
        };
        return Vec::default()
    }
}
