use lernda::term::Term;
use lernda::{abs, app, var};

fn main() {
    let x = var!("x");
    let y = var!("y");

    let mut flip = abs!("x", abs!("y", app!(y, x)));

    flip.replace("x", var!("h"));

    flip.replace("y", var!("z"));
    println!("{}", flip);
}
