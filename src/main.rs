use lernda::term::Term;
use lernda::{abs, app, var};

fn main() {
    let x = var!("x");
    let y = var!("y");

    let flip = abs!("x", abs!("y", app!(y, x)));

    // let res = flip.replace("x", &var!["h"]);
    //
    // let res = flip.replace("y", &var!("z"));
    // println!("{}", res);
}
