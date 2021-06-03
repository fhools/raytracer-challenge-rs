use utils::*;
use types::*;
extern crate raytracer_challenge_rs;
use raytracer_challenge_rs::Canvas;
use std::f64::consts::PI;
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

#[test]
fn sohere_translated_normal_at() {
    let mut s = Sphere::new();
    s.set_transform(Matrix4x4::translation(0.0, 1.0, 0.0));
    let n = s.normal_at(Vector4D::new_point(0.0, 1.70711, -0.70711));
    assert!(n.eq(&Vector4D::new_vector(0.0, 0.7071067811, -0.7071067811)));

}

#[test]
fn sphere_tranformed__normal_at() {
    let mut s = Sphere::new();
    s.set_transform(MatrixChainer::new()
                    .then(Matrix4x4::rotate_z(PI/5.0))
                    .then(Matrix4x4::scaling(1.0, 0.5, 1.0))
                    .finish());
    let n = s.normal_at(Vector4D::new_point(0.0, 2.0f64.sqrt() / 2.0, - 2.0f64.sqrt() / 2.0));
    assert!(n.eq(&Vector4D::new_vector(0.0, 0.970142500, -0.2425356250)));
}

#[test]
fn reflect_45degree() {
    let v = Vector4D::new_vector(1.0, -1.0, 0.0);
    let n = Vector4D::new_vector(0.0, 1.0, 0.0);
    let r = reflect(v, n);
    assert!(r.eq(&Vector4D::new_vector(1.0, 1.0, 0.0)));
}


#[test]
fn reflect_obtuse_anglee() {
    let v = Vector4D::new_vector(0.0, -1.0, 0.0);
    let n = Vector4D::new_vector(2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0, 0.0);
    let r = reflect(v, n);
    assert_vector4d_eq!(r, Vector4D::new_vector(1.0, 0.0, 0.0));
}


