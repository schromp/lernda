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
fn test_replace_abstraction() {
    let x = var!("x");
    let y = var!("y");

    let flip = abs!("x", abs!("y", app!(y, x)));

    let _res = flip.replace("x", &var!["h"]);

    // assert_eq!(res, abs!(""))
}

// TODO: is reducible tests

/// .simple reduction test
/// expected:
/// (x: x) y -> y
#[test]
fn test_reduce_simple() {
    let x = var!("x");
    let y = var!("y");
    let abs = abs!("x", x);
    let app = app!(abs, y);

    let result = app.reduce();

    assert_eq!("y", &result.to_string())
}

#[test]
fn test_reduce_flip() {
    let x = var!("x");
    let y = var!("y");

    let flip = abs!("x", abs!("y", app!(y, x)));
    let apply_flip = app!(app!(flip, var!("a")), var!("b"));

    println!("{}", apply_flip);

    let first = apply_flip.reduce();
    println!("{}", first);

    assert_eq!("(y x)", first.reduce().to_string())

}
