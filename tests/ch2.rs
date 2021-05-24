use types::*;
use utils::*;
#[test]
fn color_new() {
    let c = Color::new(0.5, 0.2, 1.0);
    assert!(f64_eq(c.red, 0.5));
    assert!(f64_eq(c.green, 0.2));
    assert!(f64_eq(c.blue, 1.0));
}

#[test]
fn color_mul_scalar() {
    let c = Color::new(0.3, 0.2, 1.0);
    let d = 2.0 * c;
    let e = c * 2.0;
    assert!(f64_eq(d.red, 0.6));
    assert!(f64_eq(d.green, 0.4));
    assert!(f64_eq(d.blue, 2.0));

    assert!(f64_eq(e.red, 0.6));
    assert!(f64_eq(e.green, 0.4));
    assert!(f64_eq(e.blue, 2.0));
}


#[test]
fn color_add() {
    let c = Color::new(0.1, 0.2, 0.3);
    let d = c + c;
    assert!(f64_eq(d.red, 0.2));
    assert!(f64_eq(d.green, 0.4));
    assert!(f64_eq(d.blue, 0.6));
}

#[test]
fn color_sub() {
    let c = Color::new(0.1, 0.2, 0.3);
    let mut d = Color::new(0.2, 0.3, 0.4);
    d = c - d;
    assert!(f64_eq(d.red, -0.1));
    assert!(f64_eq(d.green, -0.1));
    assert!(f64_eq(d.blue, -0.1));
}

