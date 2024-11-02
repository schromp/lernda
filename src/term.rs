pub enum Term {
    Abstraction {
        var_name: String,
        body: Box<Term>,
    },
    Application {
        l_term: Box<Term>,
        r_term: Box<Term>,
    },
    Variable {
        name: String,
    },
}

impl Term {
    pub fn free_variables(&self) -> Vec<&str> {
        match self {
            Term::Abstraction { var_name, body } => {
                let mut free_vars = body.as_ref().free_variables();
                if let Some(pos) = free_vars.iter().position(|x| x == var_name) {
                    free_vars.remove(pos);
                }

                free_vars
            }
            Term::Application { l_term, r_term } => {
                let mut free_vars = l_term.as_ref().free_variables();
                free_vars.extend(r_term.as_ref().free_variables());

                free_vars
            }
            Term::Variable { name } => {
                vec![name]
            }
        }
    }
}
