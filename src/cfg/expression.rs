use cfg::{lsep, rsep, op, function, numeric, variable};

type Lsep = lsep::Lsep;
type Rsep = rsep::Rsep;
type Op = op::Op;
type Function = function::Function;
type Numeric = numeric::Numeric;
type Variable = variable::Variable;

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
    Function(Function),
    Numeric(Numeric),
    Variable(Variable),
    Expansion {
        lexpr: Box<Expression>,
        rexpr: Box<Expression>
    }
}

impl From<Function> for Expression {
    fn from(function: Function) -> Self {
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