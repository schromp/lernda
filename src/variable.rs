use core::fmt;

#[derive(Clone, Debug)]
pub struct Variable {
    name: String,
}

impl Variable {
    pub fn new(name: String) -> Self {
        Self { name }
    }

    pub fn name(&self) -> &str {
        &self.name
    }
}

#[macro_export]
macro_rules! var {
    ($name:expr) => {
        Term::Variable($crate::variable::Variable::new($name.to_string()))
    };
}
