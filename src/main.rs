use lernda::builtin_terms::BuiltinTerms;
use lernda::term::Term;
use lernda::{abs, app, var};

fn main() {
    let x = var!("x");
    let y = var!("y");

    let mut flip = abs!("x", abs!("y", app!(y, x)));

    flip.replace("x", var!("h"));

    flip.replace("y", var!("z"));
    println!("{}", flip);

    let t = BuiltinTerms::True;
    let not = BuiltinTerms::Not;

    // let mut app = app!(lernda::term::Term::BuiltinTerms(not), lernda::term::Term::BuiltinTerms(t));
    //
    // app.reduce();
    //
    // println!("{}", app);
}
