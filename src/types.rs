use std::collections::HashMap;

#[derive(Clone, PartialEq)]
pub enum Types {
    // Boolean Types
    True,
    False,
    Not,
    Number,
    Function { left: Box<Types>, right: Box<Types> },
}

#[derive(Clone)]
struct Environment {
    mapping: HashMap<String, Types>,
}

impl Environment {
    fn new(mapping: HashMap<String, Types>) -> Self {
        Self { mapping }
    }

    pub fn comma(&mut self, var_name: &str, typ: Types) {
        self.mapping.insert(var_name.to_string(), typ);
    }

    pub fn type_of_variable_name(&self, name: &str) -> Option<&Types> {
        self.mapping.get(name)
    }
}
