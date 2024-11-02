use crate::application::Application;
use crate::variable::Variable;
use crate::abstraction::Abstraction;

pub trait LTTerm {
    /// Get all free variables recursively
    fn free_variable(&self) -> Vec<&str>;
}

pub enum Term {
    Abstraction(Abstraction),
    Application(Application),
    Variable(Variable),
}
