use cfg::{assign, expression};

type Assign = assign::Assign;
type Expression = expression::Expression;

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