use lernda::term::Term;
use lernda::{abs, app, var};

#[test]
fn test_free_variables() {
    let x = var!("x");
    let y = var!("y");
    let a = var!("a");
    let b = var!("b");

    let flip = abs!("x", abs!("y", app!(y, x)));

    let apply_flip = app!(app!(flip, a), b);

    assert_eq!(apply_flip.free_variables(), vec!["a", "b"])
}

#[test]
fn test_replace() {
    let x = var!("x");
    let y = var!("y");
    let a = var!("a");
    let b = var!("b");

    let flip = abs!("x", abs!("y", app!(y, x)));

    // let apply_flip = app!(app!(flip, a), b);

    flip.replace("x", &var!["h"]);
    println!("{:?}", flip);
}
