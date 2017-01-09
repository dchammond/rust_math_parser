#![allow(dead_code)]

pub enum Statement {
    Assignment(Assign),
    Expr(Expr),
}

pub enum Assign {
    A(V, Expr)
}

pub enum Func {
    Func(V, Lsep, Box<Expr>, Rsep)
}

pub enum Expr {
    ParenthizedExpression(Lsep, Box<Expr>, Rsep),
    OperatorExpression(Box<Expr>, Op, Box<Expr>),
    Function(Func),
    Numeric(Num),
    Variable(V),
    Expansition(Box<Expr>, Box<Expr>)
}

pub enum Lsep {
    Lsep(String)
}

pub enum Rsep {
    Rsep(String)
}

pub enum V {
    V(String)
}

pub enum Num {
    Num(f64)
}

pub enum Op {
    Plus,
    Minus,
    Multiply,
    Divide,
    Power,
    Modulo
}

pub enum Value {
    NonTerminating(NonTerminating),
    Terminating(Terminating)
}

pub enum NonTerminating {
    S(Statement),
    A(Assign),
    F(Func),
    Expr(Expr)
}

pub enum Terminating {
    Lsep(Lsep),
    Rsep(Rsep),
    V(V),
    Num(Num),
    Op(Op)
}
// Implement From for all the things
pub struct Lexer {
    input: Option<String>,
    stack: Vec<Value>
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
    pub fn transform(&self) -> Value {
        let x: Value = Value::Terminating(Terminating::Num(Num::Num(6.4)));
        x
    }
}
