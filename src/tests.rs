use crate::canvas::*;
use utils::*;

#[test]
fn canvas_new() {
    let c = Canvas::new(2,2);
    assert_eq!(c.width, 2);
    assert_eq!(c.height, 2);
    for i in 0..c.width {
        for j in 0..c.height {
            assert!(f64_eq(c.canvas[j][i].red, 0.0));
            assert!(f64_eq(c.canvas[j][i].green, 0.0));
            assert!(f64_eq(c.canvas[j][i].blue, 0.0));
        }
    }
}