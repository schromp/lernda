use crate::lt_term::{LTTerm, Term};

pub struct Application {
    l_term: Box<Term>,
    r_term: Box<Term>,
}

impl LTTerm for Application {
    fn free_variable(&self) -> Vec<&str> {
        todo!()
    }
}

// impl PartialEq for Application {
//     fn eq(&self, other: &Self) -> bool {
//         self.l_term == self.r_term
//     }
// }
