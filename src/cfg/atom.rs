use cfg::{statement, assign, function, expression, lsep, rsep, variable, numeric, op, equal};

type Lsep = lsep::Lsep;
type Rsep = rsep::Rsep;
type Variable = variable::Variable;
type Numeric = numeric::Numeric;
type Op = op::Op;
type Statement = statement::Statement;
type Assign = assign::Assign;
type Function = function::Function;
type Expression = expression::Expression;
type Eq = eq::Eq;

pub enum Atom {
    Statement(Statement),
    Assignemnt(Assign),
    Function(Function),
    Expression(Expression),
    Lsep(Lsep),
    Rsep(Rsep),
    Variable(Variable),
    Numeric(Numeric),
    Op(Op),
    Equal(Equal),
}

impl From<Statement> for Atom {
    fn from(statement: Statement) -> Self {
        Atom::Statement(statement)
    }
}

impl From<Assign> for Atom {
    fn from(assignment: Assign) -> Self {
        Atom::Assignemnt(assignment)
    }
}

impl From<Function> for Atom {
    fn from(function: Function) -> Self {
        Atom::Function(function)
    }
}

impl From<Expression> for Atom {
    fn from(expression: Expression) -> Self {
        Atom::Expression(expression)
    }
}

impl From<Lsep> for Atom {
    fn from(lsep: Lsep) -> Self {
        Atom::Lsep(lsep)
    }
}

impl From<Rsep> for Atom {
    fn from(rsep: Rsep) -> Self {
        Atom::Rsep(rsep)
    }
}

impl From<Variable> for Atom {
    fn from(variable: Variable) -> Self {
        Atom::Variable(variable)
    }
}

impl From<Numeric> for Atom {
    fn from(numeric: Numeric) -> Self {
        Atom::Numeric(numeric)
    }
}

impl From<Op> for Atom {
    fn from(operator: Op) -> Self {
        Atom::Op(operator)
    }
}

impl From<Equal> for Atom {
    fn from(equal: Equal) -> Self {
        Atom::Equal(equal)
    }
}