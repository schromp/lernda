use core::fmt;

use crate::term::Term;

#[derive(Clone, Debug)]
pub struct Application {
    l_term: Box<Term>,
    r_term: Box<Term>,
}

impl Application {
    pub fn new(l_term: Box<Term>, r_term: Box<Term>) -> Self {
        Self { l_term, r_term }
    }

    pub fn l_term(&self) -> &Term {
        &self.l_term
    }

    pub fn r_term(&self) -> &Term {
        &self.r_term
    }
}

#[macro_export]
macro_rules! app {
    ($l_term:expr, $r_term:expr) => {
        Term::Application($crate::application::Application::new(
            Box::new($l_term),
            Box::new($r_term),
        ))
    };
}
