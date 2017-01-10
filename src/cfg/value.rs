use cfg::{nonterminating, terminating};

type NonTerminating = nonterminating::NonTerminating;
type Terminating = terminating::Terminating;

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