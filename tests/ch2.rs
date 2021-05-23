use types::*;
use utils::*;
#[test]
fn color_new() {
    let c = Color::new(0.5, 0.2, 1.0);
    assert!(f64_eq(c.red, 0.5));
    assert!(f64_eq(c.green, 0.5));
    assert!(f64_eq(c.blue, 1.0));
}

