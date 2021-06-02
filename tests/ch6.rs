use utils::*;
use types::*;
extern crate raytracer_challenge_rs;
use raytracer_challenge_rs::Canvas;

#[test]
fn sphere_normal_at() {
    let s = Sphere::new();
    let mut n = s.normal_at(Vector4D::new_point(1.0, 0.0, 0.0));
    assert!(n.eq(&Vector4D::new_vector(1.0, 0.0, 0.0)));
    n = s.normal_at(Vector4D::new_point(0.0, 1.0, 0.0));
    assert!(n.eq(&Vector4D::new_vector(0.0, 1.0, 0.0)));
    n = s.normal_at(Vector4D::new_point(0.0, 0.0, 1.0));
    assert!(n.eq(&Vector4D::new_vector(0.0, 0.0, 1.0)));
}
