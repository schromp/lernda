use crate::lt_term::LTTerm;

pub struct Variable {
    name: String,
}

impl Variable {
    pub fn name(&self) -> &str {
        &self.name
    }
}

impl LTTerm for Variable {
    fn free_variable(&self) -> Vec<&str> {
        vec![&self.name]
    }
}
