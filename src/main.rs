use lernda::term::Term;
use lernda::{abs, app, var};

fn main() {
    let x = var!("x");
    println!("{:?}", x);

    let abs = abs!("x", x);
    println!("{:?}", abs);

    let y = var!("y");
    let app = app!(abs, y);
    println!("{:?}", app);

    app.replace("y", &var!("z"));
    println!("{:?}", app);
}
