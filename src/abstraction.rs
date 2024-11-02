use crate::lt_term::{LTTerm, Term};

pub struct Abstraction {
    var_name: String,
    body: Box<Term>,
}

impl LTTerm for Abstraction {
    fn free_variable(&self) -> Vec<&str> {
        todo!()
    }
}
