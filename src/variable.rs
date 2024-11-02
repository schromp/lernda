use crate::lt_term::{self, LTTerm, Term};

pub struct Variable {
    name: String,
}

impl LTTerm for Variable {
    fn free_variable(&self) -> Vec<&str> {
        vec![&self.name]
    }

    fn reduce(&self) -> lt_term::Term {
        todo!()
    }

    fn replace(&self, var_name: String, t: lt_term::Term) -> lt_term::Term {
        todo!()
    }

    fn equals(&self, t: Term) -> bool {
        match t {
            Term::Variable(v) => v.name == self.name,
            _ => false,
        }
    }
}
