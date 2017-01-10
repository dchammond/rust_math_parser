use cfg::{statement, assign, function, expression};

type Statement = statement::Statement;
type Assign = assign::Assign;
type Function = function::Function;
type Expression = expression::Expression;

pub enum NonTerminating {
    Statement(Statement),
    Assignemnt(Assign),
    Function(Function),
    Expression(Expression)
}

impl From<Statement> for NonTerminating {
    fn from(statement: Statement) -> Self {
        NonTerminating::Statement(statement)
    }
}

impl From<Assign> for NonTerminating {
    fn from(assignment: Assign) -> Self {
        NonTerminating::Assignemnt(assignment)
    }
}

impl From<Function> for NonTerminating {
    fn from(function: Function) -> Self {
        NonTerminating::Function(function)
    }
}

impl From<Expression> for NonTerminating {
    fn from(expression: Expression) -> Self {
        NonTerminating::Expression(expression)
    }
}