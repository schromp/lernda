use term::Term;

mod term;

fn main() {
    let x = Term::Variable {
        name: "x".to_string(),
    };
    let abs = Term::Abstraction {
        var_name: "y".to_string(),
        body: Box::new(Term::Variable {
            name: "y".to_string(),
        }),
    };

    let application = Term::Application {
        l_term: Box::new(abs),
        r_term: Box::new(x),
    };

    let vars = application.free_variables();
    println!("{:?}", vars);
}
