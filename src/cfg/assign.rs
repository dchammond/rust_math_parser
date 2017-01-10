use cfg::{variable, expression};

type Variable = variable::Variable;
type Expression = expression::Expression;

pub struct Assign {
    variable: Variable,
    expression: Expression
}