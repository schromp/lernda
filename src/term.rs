use core::fmt;

use crate::{abstraction::Abstraction, application::Application, variable::Variable};

#[derive(Clone, Debug)]
pub enum Term {
    Abstraction(Abstraction),
    Application(Application),
    Variable(Variable),
}

impl Term {
    pub fn free_variables(&self) -> Vec<&str> {
        match self {
            Term::Abstraction(a) => {
                let mut free_vars = a.body().free_variables();
                if let Some(pos) = free_vars.iter().position(|x| *x == a.var_name()) {
                    free_vars.remove(pos);
                }

                free_vars
            }
            Term::Application(a) => {
                let mut free_vars = a.l_term().free_variables();
                free_vars.extend(a.r_term().free_variables());

                free_vars
            }
            Term::Variable(v) => {
                vec![v.name()]
            }
        }
    }

    pub fn replace(&self, var_to_replace: &str, term: &Term) -> Term {
        match self {
            Term::Abstraction(a) => {
                let new_var_name = match term {
                    Term::Variable(v) => {
                        if *v.name() != *a.var_name() && a.var_name() == var_to_replace {
                            v.name()
                        } else {
                            a.var_name()
                        }
                    }
                    _ => a.var_name(),
                };

                // FIX: i think alpha conversion should be used here in the case free variables
                // would be overwritten

                if a.var_name() == var_to_replace {
                    a.body().replace(var_to_replace, term)
                } else {
                    Term::Abstraction(Abstraction::new(
                        a.var_name().to_string(),
                        Box::new(a.body().replace(var_to_replace, term)),
                    ))
                }
            }
            Term::Application(a) => {
                let l = a.l_term().replace(var_to_replace, term);
                let r = a.r_term().replace(var_to_replace, term);

                Term::Application(Application::new(Box::new(l), Box::new(r)))
            }
            Term::Variable(v) => {
                if v.name() == var_to_replace {
                    term.clone()
                } else {
                    self.clone()
                }
            }
        }
    }

    fn is_reducible(&self) -> bool {
        match self {
            Term::Abstraction { .. } => false,
            Term::Application(a) => {
                a.l_term().is_reducible()
                    || a.r_term().is_reducible()
                    || matches!(a.l_term(), Term::Abstraction { .. })
            }
            Term::Variable { .. } => false,
        }
    }

    pub fn reduce(&self) -> Term {
        match self {
            Term::Abstraction(_) => self.clone(),
            Term::Application(a) => {
                if a.l_term().is_reducible() {
                    return Term::Application(Application::new(Box::new(a.l_term().reduce()), Box::new(a.r_term().clone())));
                } else if a.r_term().is_reducible() {
                    return Term::Application(Application::new(Box::new(a.l_term().clone()), Box::new(a.r_term().reduce())));
                } else if let Term::Abstraction(abs) = a.l_term() {
                    if !a.r_term().is_reducible() {
                        Term::replace(a.l_term(), abs.var_name(), a.r_term())
                    } else {
                        unreachable!("Right and left terms should already be reduced so this should be unreachable")
                    }
                } else {
                    self.clone()
                }
            }
            Term::Variable(_) => self.clone(),
        }
    }
}

impl fmt::Display for Term {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match self {
            Term::Abstraction(a) => {
                write!(f, "({}: {})", a.var_name(), a.body())
            }
            Term::Application(a) => {
                write!(f, "({} {})", a.l_term(), a.r_term())
            }
            Term::Variable(v) => {
                write!(f, "{}", v.name())
            }
        }
    }
}
