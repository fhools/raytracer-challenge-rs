use utils::*;
use types::*;
#[test]
fn new_point() {
    let p = Vector4D::new(4.3, -4.2, 3.1, 1.0);
    assert!(f64_eq(p.x, 4.3));
    assert!(f64_eq(p.y, -4.2));
    assert!(f64_eq(p.z, 3.1));
    assert!(f64_eq(p.w, 1.0));
    assert!(p.is_point());
    assert!(!p.is_vector());
}
