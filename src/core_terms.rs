use core::fmt;

use crate::term::Term;

#[derive(Clone, Debug)]
pub enum CoreTerm {
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

impl CoreTerm {
    pub fn reduce(self) -> Self {
        match self {
            CoreTerm::Abstraction { .. } => self,
            CoreTerm::Application { mut l_term, r_term } => {
                if l_term.is_reducible() {
                    return l_term.reduce()
                } else if r_term.is_reducible() {
                    r_term.reduce();
                } else if let Term::CoreTerm(CoreTerm::Abstraction {
                    ref var_name,
                    body: _,
                }) = &mut *l_term
                {
                    if !r_term.is_reducible() {
                        let var_name = var_name.clone();
                        l_term.replace(&var_name, *r_term)
                    }
                }
            }
            CoreTerm::Variable { .. } => self,
        }
    }
}

impl fmt::Display for CoreTerm {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            CoreTerm::Abstraction { var_name, body } => {
                write!(f, "({}: {})", var_name, body)
            }
            CoreTerm::Application { l_term, r_term } => {
                write!(f, "({} {})", l_term, r_term)
            }
            CoreTerm::Variable { name } => {
                write!(f, "{}", name)
            }
        }
    }
}

impl PartialEq for CoreTerm {
    fn eq(&self, other: &Self) -> bool {
        match self {
            CoreTerm::Abstraction { var_name, body } => {
                if let CoreTerm::Abstraction {
                    var_name: other_var_name,
                    body: other_body,
                } = other
                {
                    var_name == other_var_name && body == other_body
                } else {
                    false
                }
            }
            CoreTerm::Application { l_term, r_term } => {
                if let CoreTerm::Application {
                    l_term: other_l_term,
                    r_term: other_r_term,
                } = other
                {
                    l_term == other_l_term && r_term == other_r_term
                } else {
                    false
                }
            }
            CoreTerm::Variable { name } => {
                if let CoreTerm::Variable { name: other_name } = other {
                    name == other_name
                } else {
                    false
                }
            }
        }
    }
}

#[macro_export]
macro_rules! abs {
    ($var_name:expr, $body:expr) => {
        Term::CoreTerm($crate::core_terms::CoreTerm::Abstraction {
            var_name: $var_name.to_string(),
            body: Box::new($body),
        })
    };
}

#[macro_export]
macro_rules! app {
    ($l_term:expr, $r_term:expr) => {
        Term::CoreTerm($crate::core_terms::CoreTerm::Application {
            l_term: Box::new($l_term),
            r_term: Box::new($r_term),
        })
    };
}

#[macro_export]
macro_rules! var {
    ($name:expr) => {
        Term::CoreTerm($crate::core_terms::CoreTerm::Variable {
            name: $name.to_string(),
        })
    };
}
