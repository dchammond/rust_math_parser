#![allow(dead_code)]

pub enum Statement {
    Assignment(Assign),
    Expression(Expression),
}

impl From<Assign> for Statement {
    fn from(assignment: Assign) -> Self {
        Statement::Assignment(assignment)
    }
}

impl From<Expression> for Statement {
    fn from(expression: Expression) -> Self {
        Statement::Expression(expression)
    }
}

pub struct Assign {
    variable: Variable,
    expression: Expression
}

pub enum Func {
    Func {
        variable: Variable,
        lsep: Lsep,
        expr: Box<Expression>,
        rsep: Rsep
    }
}

pub enum Expression {
    ParenthizedExpression {
        lsep: Lsep,
        expr: Box<Expression>,
        rsep: Rsep
    },
    OperatorExpression {
        lexpr: Box<Expression>,
        op: Op,
        rexpr: Box<Expression>
    },
    Function(Func),
    Numeric(Numeric),
    Variable(Variable),
    Expansion {
        lexpr: Box<Expression>,
        rexpr: Box<Expression>
    }
}

impl From<Func> for Expression {
    fn from(function: Func) -> Self {
        Expression::Function(function)
    }
}

impl From<Numeric> for Expression {
    fn from(numeric: Numeric) -> Self {
        Expression::Numeric(numeric)
    }
}

impl From<Variable> for Expression {
    fn from(variable: Variable) -> Self {
        Expression::Variable(variable)
    }
}

pub struct Lsep(String);

pub struct Rsep(String);

pub struct Variable(String);

pub struct Numeric(f64);

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
    Expr(Expression)
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

impl From<Expression> for NonTerminating {
    fn from(expression: Expression) -> Self {
        NonTerminating::Expr(expression)
    }
}

pub enum Terminating {
    Lsep(Lsep),
    Rsep(Rsep),
    Variable(Variable),
    Numeric(Numeric),
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

impl From<Variable> for Terminating {
    fn from(variable: Variable) -> Self {
        Terminating::Variable(variable)
    }
}

impl From<Numeric> for Terminating {
    fn from(numeric: Numeric) -> Self {
        Terminating::Numeric(numeric)
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
        let x: Value = Value::from(Terminating::from(Numeric(6.4)));
        x
    }
}
