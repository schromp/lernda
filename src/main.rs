use term::Term;

mod term;

fn main() {
    let x = var!("x");
    let y = var!("y");
    let a = var!("a");
    let b = var!("b");

    let flip = abs!("x", abs!("y", app!(y, x)));

    let apply_flip = app!(app!(flip, a), b);

    println!("{:?}", apply_flip.free_variables());
}
