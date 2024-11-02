use crate::lt_term::{LTTerm, Term};

pub struct Abstraction {
    var_name: String,
    body: Box<Term>,
}

impl LTTerm for Abstraction {
    fn free_variable(&self) -> Vec<&str> {
        match self.body.as_ref() {
            Term::Variable(v) => {
                if v.name() == self.var_name {
                    vec![v.name()]
                } else {
                    vec![]
                }
            }
            Term::Abstraction(a) => {
                let mut rec_vars = a.free_variable();
                if let Some(pos) = rec_vars.iter().position(|x| x == &self.var_name) {
                    rec_vars.remove(pos);
                }
                rec_vars
            }
            Term::Application(a) => {
                let mut rec_vars = a.free_variable();
                if let Some(pos) = rec_vars.iter().position(|x| x == &self.var_name) {
                    rec_vars.remove(pos);
                }
                rec_vars
            }
        }
    }
}
