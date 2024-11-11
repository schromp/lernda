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

    let mut flip = abs!("x", abs!("y", app!(y.clone(), x)));

    flip.replace("x", var!("a"));

    assert_eq!(flip, abs!("x", abs!("y", app!(y, var!("a")))));
}

#[test]
fn test_coreterm_is_reducible_variable() {
    let x = var!("x");
    assert!(!x.is_reducible())
}

#[test]
fn test_coreterm_is_reducible_abstraction() {
    let x = var!("x");
    let abs = abs!("x", x);
    assert!(!abs.is_reducible())
}

#[test]
fn test_coreterm_is_reducible_application_true() {
    let x = var!("x");
    let y = var!("y");
    let abs = abs!("x", x);
    let app = app!(abs, y);
    assert!(app.is_reducible())
}

#[test]
fn test_coreterm_is_reducible_application_false() {
    let x = var!("x");
    let y = var!("y");
    let app = app!(x, y);
    assert!(!app.is_reducible())
}

/// .simple reduction test
/// expected:
/// (x: x) y -> y
#[test]
fn test_reduce_simple() {
    let x = var!("x");
    let y = var!("y");
    let abs = abs!("x", x);
    let app = app!(abs, y);

    app.reduce();

    assert_eq!("y", &app.to_string())
}

#[test]
fn test_reduce_flip() {
    let x = var!("x");
    let y = var!("y");

    let flip = abs!("x", abs!("y", app!(y, x)));
    let apply_flip = app!(app!(flip, var!("a")), var!("b"));

    let first = apply_flip.reduce();

    // assert_eq!("(b a)", first.reduce().to_string())
}
