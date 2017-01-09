#![allow(dead_code)]

pub enum Statement {
    Assignment(Assign),
    Expression(Expr),
}

impl From<Assign> for Statement {
    fn from(assignment: Assign) -> Self {
        Statement::Assignment(assignment)
    }
}

impl From<Expr> for Statement {
    fn from(expression: Expr) -> Self {
        Statement::Expression(expression)
    }
}

pub struct Assign {
    variable: V,
    expression: Expr
}

pub enum Func {
    Func {
        variable: V,
        lsep: Lsep,
        expr: Box<Expr>,
        rsep: Rsep
    }
}

pub enum Expr {
    ParenthizedExpression {
        lsep: Lsep,
        expr: Box<Expr>,
        rsep: Rsep
    },
    OperatorExpression {
        lexpr: Box<Expr>,
        op: Op,
        rexpr: Box<Expr>
    },
    Function(Func),
    Numeric(Num),
    Variable(V),
    Expansion {
        lexpr: Box<Expr>,
        rexpr: Box<Expr>
    }
}

impl From<Func> for Expr {
    fn from(function: Func) -> Self {
        Expr::Function(function)
    }
}

impl From<Num> for Expr {
    fn from(numeric: Num) -> Self {
        Expr::Numeric(numeric)
    }
}

impl From<V> for Expr {
    fn from(variable: V) -> Self {
        Expr::Variable(variable)
    }
}

pub struct Lsep(String);

pub struct Rsep(String);

pub struct V(String);

pub struct Num(f64);

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

impl From<NonTerminating> for Value {
    fn from(non_terminating: NonTerminating) -> Self {
        Value::NonTerminating(non_terminating)
    }
}

impl From<Terminating> for Value {
    fn from(terminating: Terminating) -> Self {
        Value::Terminating(terminating)
    }
}

pub enum NonTerminating {
    S(Statement),
    A(Assign),
    F(Func),
    Expr(Expr)
}

impl From<Statement> for NonTerminating {
    fn from(statement: Statement) -> Self {
        NonTerminating::S(statement)
    }
}

impl From<Assign> for NonTerminating {
    fn from(assignment: Assign) -> Self {
        NonTerminating::A(assignment)
    }
}

impl From<Func> for NonTerminating {
    fn from(function: Func) -> Self {
        NonTerminating::F(function)
    }
}

impl From<Expr> for NonTerminating {
    fn from(expression: Expr) -> Self {
        NonTerminating::Expr(expression)
    }
}

pub enum Terminating {
    Lsep(Lsep),
    Rsep(Rsep),
    V(V),
    Num(Num),
    Op(Op)
}

impl From<Lsep> for Terminating {
    fn from(lsep: Lsep) -> Self {
        Terminating::Lsep(lsep)
    }
}

impl From<Rsep> for Terminating {
    fn from(rsep: Rsep) -> Self {
        Terminating::Rsep(rsep)
    }
}

impl From<V> for Terminating {
    fn from(variable: V) -> Self {
        Terminating::V(variable)
    }
}

impl From<Num> for Terminating {
    fn from(numeric: Num) -> Self {
        Terminating::Num(numeric)
    }
}

impl From<Op> for Terminating {
    fn from(operator: Op) -> Self {
        Terminating::Op(operator)
    }
}

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
        let x: Value = Value::from(Terminating::from(Num(6.4)));
        x
    }
}
