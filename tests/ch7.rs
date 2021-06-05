use types::*;
use utils::*;
extern crate raytracer_challenge_rs;

#[test]
fn test_default_world() {
    let world : World = Default::default();
}

#[test]
fn ray_intersect_world() {
    let world : World = Default::default();
    let ray = Ray::new(Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(0.0,0.0,1.0));
    let xs = ray.intersect_world(&world);
    assert_eq!(xs.len() , 4);
    assert_f64_eq!(xs[0].t, 4.0);
    assert_f64_eq!(xs[1].t, 4.5);
    assert_f64_eq!(xs[2].t, 5.5);
    assert_f64_eq!(xs[3].t, 6.0);
}
