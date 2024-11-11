use crate::{abs, app, term::Term, types::Types, var};

#[derive(Clone, Debug, PartialEq)]
pub enum BuiltinTerms {
    // Boolean Terms
    True,
    False,
    Not,
}

impl BuiltinTerms {
    pub fn as_term(&self) -> Term {
        match self {
            BuiltinTerms::True => {
                abs!("x", abs!("y", var!("x")))
            }
            BuiltinTerms::False => {
                abs!("x", abs!("y", var!("y")))
            }
            BuiltinTerms::Not => {
                abs!(
                    "b",
                    app!(
                        app!(var!("b"), BuiltinTerms::False.as_term()),
                        BuiltinTerms::True.as_term()
                    )
                )
            }
        }
    }

    pub fn get_type(&self) -> Types {
        match self {
            BuiltinTerms::True => Types::True,
            BuiltinTerms::False => Types::False,
            BuiltinTerms::Not => Types::Not,
        }
    }
}
