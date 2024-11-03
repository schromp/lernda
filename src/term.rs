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

// Macro for creating an Abstraction
#[macro_export]
macro_rules! abs {
    ($var_name:expr, $body:expr) => {
        Term::Abstraction {
            var_name: $var_name.to_string(),
            body: Box::new($body),
        }
    };
}

// Macro for creating an Application
#[macro_export]
macro_rules! app {
    ($l_term:expr, $r_term:expr) => {
        Term::Application {
            l_term: Box::new($l_term),
            r_term: Box::new($r_term),
        }
    };
}

// Macro for creating a Variable
#[macro_export]
macro_rules! var {
    ($name:expr) => {
        Term::Variable {
            name: $name.to_string(),
        }
    };
}
