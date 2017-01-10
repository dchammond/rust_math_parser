use cfg::{variable, lsep, rsep, expression};

type Variable = variable::Variable;
type Lsep = lsep::Lsep;
type Rsep = rsep::Rsep;
type Expression = expression::Expression;

pub enum Function {
    Function {
        variable: Variable,
        lsep: Lsep,
        expr: Box<Expression>,
        rsep: Rsep
    }
}