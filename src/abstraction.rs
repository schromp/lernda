use core::fmt;

use crate::term::Term;

#[derive(Clone, Debug)]
pub struct Abstraction {
    var_name: String,
    body: Box<Term>,
}

impl Abstraction {
    pub fn new(var_name: String, body: Box<Term>) -> Self {
        Self { var_name, body }
    }

    pub fn var_name(&self) -> &str {
        &self.var_name
    }

    pub fn body(&self) -> &Term {
        &self.body
    }
}

#[macro_export]
macro_rules! abs {
    ($var_name:expr, $body:expr) => {
        Term::Abstraction($crate::abstraction::Abstraction::new(
            $var_name.to_string(),
            Box::new($body),
        ))
    };
}
