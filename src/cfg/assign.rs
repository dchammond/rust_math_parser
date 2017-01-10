use cfg::{variable, expression, equal};

type Variable = variable::Variable;
type Expression = expression::Expression;
type Equal = equal::Equal;

pub struct Assign {
    variable: Variable,
    equal: Equal,
    expression: Expression
}