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
#[test]
fn new_vector() {
    let p = Vector4D::new_vector(4.3, -4.2, 3.1);
    assert!(f64_eq(p.x, 4.3));
    assert!(f64_eq(p.y, -4.2));
    assert!(f64_eq(p.z, 3.1));
    assert!(f64_eq(p.w, 0.0));
    assert!(!p.is_point());
    assert!(p.is_vector());
}


#[test]
fn scalar_mul_vector() {
    let v = Vector4D::new_vector(1.0, 2.0, 3.0);
    let w = 4f64 * v;

    println!("w: {:?}", w);
    assert!(f64_eq(4f64 * v.x, w.x));
    assert!(f64_eq(4f64 * v.y, w.y));
    assert!(f64_eq(4f64 * v.z, w.z));
    assert!(f64_eq(v.w, w.w));
}

#[test]
fn vector_divide_by_scalar() {
    let v = Vector4D::new(1.0, 2.0, 3.0, 0.0);
    let w = v / 2.0;

    assert!(f64_eq(v.x / 2.0, w.x));
    assert!(f64_eq(v.y / 2.0, w.y));
    assert!(f64_eq(v.z / 2.0, w.z));
    assert!(f64_eq(v.w, w.w));
}

#[test]
fn vector_addition() {
    let v = Vector4D::new(1.0, 2.0, 3.0, 0.0);
    let w = Vector4D::new(2.0, 3.0, 4.0, 0.0);
    let z = v + w;
    assert!(f64_eq(z.x, 3.0));
    assert!(f64_eq(z.y, 5.0));
    assert!(f64_eq(z.z, 7.0));
    assert!(f64_eq(z.w, 0.0));
}

#[test]
fn vector_subtraction() {
    let v = Vector4D::new(1.0, 2.0, 3.0, 0.0);
    let w = Vector4D::new(2.0, 3.0, 4.0, 0.0);
    let z = v - w;
    assert!(f64_eq(z.x, -1.0));
    assert!(f64_eq(z.y, -1.0));
    assert!(f64_eq(z.z, -1.0));
    assert!(f64_eq(z.w, 0.0));
}

#[test]
fn vector_negation() {
    let v = Vector4D::new(1.0, 2.0, 3.0, 0.0);
    let z = - v;
    assert!(f64_eq(z.x, -1.0));
    assert!(f64_eq(z.y, -2.0));
    assert!(f64_eq(z.z, -3.0));
    assert!(f64_eq(z.w, 0.0));
}


#[test]
fn vector_norm() {
    let v = Vector4D::new(2.0, 2.0, 2.0, 0.0);
    assert_eq!(v.norm(), ((4.0 + 4.0 + 4.0) as f64).sqrt());
}

#[test]
fn vector_crossproduct() {
    let v = Vector4D::new_vector(1.0, 2.0, 3.0);
    let w = Vector4D::new_vector(2.0, 3.0, 4.0);
    let vxw = v.cross(w);
    let wxv = w.cross(v);


    assert!(f64_eq(vxw.x, -1.0));
    assert!(f64_eq(vxw.y, 2.0));
    assert!(f64_eq(vxw.z, -1.0));

    assert!(f64_eq(wxv.x, 1.0));
    assert!(f64_eq(wxv.y, -2.0));
    assert!(f64_eq(wxv.z, 1.0));
}


#[test]
fn vector_dot() {
    let v = Vector4D::new_vector(1.0, 2.0, 3.0);
    let w = Vector4D::new_vector(2.0, 3.0, 4.0);
    let d = v.dot(w);
    assert_f64_eq!(d, 20.0)
}

