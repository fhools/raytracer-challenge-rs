use types::*;
use utils::*;
use std::f64::consts::*;
#[test]
fn matrix4x4_translation() {
    let p = Vector4D::new_point(1.0,2.0,3.0);
    let tm = Matrix4x4::translation(1.0,1.0,1.0);
    let new_p = tm.mul_vector4d(&p);
    assert_f64_eq!(new_p.x, 2.0);
    assert_f64_eq!(new_p.y, 3.0);
    assert_f64_eq!(new_p.z, 4.0);
}

#[test]
fn matrix4x4_scaling() {
    let p = Vector4D::new_point(1.0, 2.0, 3.0);
    let sm = Matrix4x4::scaling(2.0, 3.0, 6.0);
    let new_p = sm.mul_vector4d(&p);
    assert_f64_eq!(new_p.x, 2.0);
    assert_f64_eq!(new_p.y, 6.0);
    assert_f64_eq!(new_p.z, 18.0);
}

#[test]
fn matrix4x4_rotate_x() {
    let p = Vector4D::new_vector(0.0, 1.0, 0.0);
    let pp = Matrix4x4::rotate_x(PI/2.0).mul_vector4d(&p);
    assert_f64_eq!(pp.x, 0.0);
    assert_f64_eq!(pp.y, 0.0);
    assert_f64_eq!(pp.z, 1.0);
}

#[test]
fn matrix4x4_rotate_y() {
    let p = Vector4D::new_vector(1.0, 0.0, 0.0);
    let pp = Matrix4x4::rotate_y(PI/2.0).mul_vector4d(&p);
    assert_f64_eq!(pp.x, 0.0);
    assert_f64_eq!(pp.y, 0.0);
    assert_f64_eq!(pp.z, -1.0);
}

#[test]
fn matrix4x4_rotate_z() {
    let p = Vector4D::new_vector(0.0, 1.0, 0.0);
    let pp = Matrix4x4::rotate_z(PI/2.0).mul_vector4d(&p);
    assert_f64_eq!(pp.x, -1.0);
    assert_f64_eq!(pp.y, 0.0);
    assert_f64_eq!(pp.z, 0.0);
}

fn matrix4x4_shear() {
    let p = Vector4D::new_vector(2.0, 3.0, 4.0);
    let pp = Matrix4x4::shearing(1.0, 0.0, 0.0, 0.0, 0.0, 0.0).mul_vector4d(&p);

    assert!(pp.eq(&Vector4D::new_vector(5.0, 3.0, 4.0)));
}


fn matrix4x4_chaining_operations() {
    let p = Vector4D::new_point(1.0, 0.0, 1.0);
    let op = MatrixChainer::new()
        .then(Matrix4x4::rotate_x(PI/2.0))
        .then(Matrix4x4::scaling(5.0, 5.0, 5.0))
        .then(Matrix4x4::translation(10.0, 5.0, 7.0))
        .finish();

    let pp = op.mul_vector4d(&p);

    assert!(pp.eq(&Vector4D::new_point(15.0, 0.0, 7.0)));
}
