use core::fmt;

use crate::core_terms::CoreTerm;

#[derive(Clone, Debug, PartialEq)]
pub enum Term {
    CoreTerm(CoreTerm),
    BuiltinTerms,
}

impl Term {
    pub fn free_variables(&self) -> Vec<&str> {
        if let Term::CoreTerm(core_term) = self {
            match core_term {
                CoreTerm::Abstraction { var_name, body } => {
                    let mut free_vars = body.free_variables();
                    if let Some(pos) = free_vars.iter().position(|x| *x == var_name) {
                        free_vars.remove(pos);
                    }

                    free_vars
                }
                CoreTerm::Application { l_term, r_term } => {
                    let mut free_vars = l_term.free_variables();
                    free_vars.extend(r_term.free_variables());

                    free_vars
                }
                CoreTerm::Variable { name } => {
                    vec![name]
                }
            }
        } else {
            vec![] // TODO: Can builtin functions contain free variables?
        }
    }

    pub fn replace(&mut self, var_to_replace: &str, term: Term) {
        if let Term::CoreTerm(core_term) = self {
            match core_term {
                CoreTerm::Abstraction { var_name, body } => {
                    // FIX: i think alpha conversion should be used here in the case free variables
                    // would be overwritten

                    body.replace(var_to_replace, term);
                }
                CoreTerm::Application { l_term, r_term } => {
                    l_term.replace(var_to_replace, term.clone());
                    r_term.replace(var_to_replace, term);
                }
                CoreTerm::Variable { name } => {
                    if name == var_to_replace {
                        *self = term;
                    }
                }
            }
        } else {
            todo!()
            // not possible
        }
    }

    pub fn is_reducible(&self) -> bool {
        if let Term::CoreTerm(core_term) = self {
            match core_term {
                CoreTerm::Abstraction { .. } => false,
                CoreTerm::Application { l_term, r_term } => {
                    l_term.is_reducible()
                        || r_term.is_reducible()
                        || matches!(**l_term, Term::CoreTerm(CoreTerm::Abstraction { .. }))
                }
                CoreTerm::Variable { .. } => false,
            }
        } else {
            false
        }
    }

    pub fn reduce(self) {
        match self {
            Term::CoreTerm(a) => a.reduce(),
            Term::BuiltinTerms => todo!("Cant reduce builtin terms"),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::CoreTerm(core_term) => {
                write!(f, "{}", core_term)
            }
            Term::BuiltinTerms => todo!(),
        }
    }
}

