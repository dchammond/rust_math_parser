use cfg::{lsep, rsep, variable, numeric, op};

type Lsep = lsep::Lsep;
type Rsep = rsep::Rsep;
type Variable = variable::Variable;
type Numeric = numeric::Numeric;
type Op = op::Op;

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