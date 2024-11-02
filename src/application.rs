use crate::lt_term::{LTTerm, Term};

pub struct Application {
    l_term: Box<Term>,
    r_term: Box<Term>,
}

impl Application {
    pub fn l_term(&self) -> &Term {
        &self.l_term
    }

    pub fn r_term(&self) -> &Term {
        &self.r_term
    }
}

impl LTTerm for Application {
    fn free_variable(&self) -> Vec<&str> {
        let l = *self.l_term.as_ref();
    }
}

// impl PartialEq for Application {
//     fn eq(&self, other: &Self) -> bool {
//         self.l_term == self.r_term
//     }
// }
