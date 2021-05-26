use types::*;
use utils::*;
use std::cmp::Eq;

#[test]
fn matrix4x4_new() {
    let m = Matrix4x4::new();
    assert!(m.m[0][0] == 1.0);
}

#[test]
fn matrix4x4_from_vec() {
    let r1 = vec![1.0, 2.0, 3.0, 4.0];
    let r2 = vec![5.0, 2.0, 9.0, 6.0];
    let r3 = vec![9.0, 8.0, 5.0, 1.0];
    let r4 = vec![4.0, 3.0, 11.0, 10.0];
    let m = Matrix4x4::from_vectors(&r1, &r2, &r3, &r4);
    assert!(f64_eq(m.m[0][0], 1.0));
    assert!(f64_eq(m.m[1][0], 5.0));
    assert!(f64_eq(m.m[2][0], 9.0));
    assert!(f64_eq(m.m[3][0], 4.0));
}


#[test]
fn matrix3x3_from_vec() {
    let r1 = vec![1.0, 2.0, 3.0];
    let r2 = vec![5.0, 2.0, 9.0];
    let r3 = vec![9.0, 8.0, 5.0];
    let m = Matrix3x3::from_vectors(&r1, &r2, &r3);
    assert!(f64_eq(m.m[0][0], 1.0));
    assert!(f64_eq(m.m[1][0], 5.0));
    assert!(f64_eq(m.m[2][0], 9.0));
}

#[test]
fn matrix2x2_from_vec() {
    let r1 = vec![1.0, 2.0];
    let r2 = vec![5.0, 2.0];
    let m = Matrix2x2::from_vectors(&r1, &r2);
    assert!(f64_eq(m.m[0][0], 1.0));
    assert!(f64_eq(m.m[1][0], 5.0));
}

#[test]
fn matrix2x2_mul() {
    let r1 = vec![1.0, 2.0];
    let r2 = vec![5.0, 2.0];
    let m = Matrix2x2::from_vectors(&r1, &r2);
    let n = Matrix2x2::from_vectors(&r1, &r2);
    let p = m.mul(&n);

    println!("p: {:?}", p);
    assert!(f64_eq(p.m[0][0], 11.0));
    assert!(f64_eq(p.m[0][1], 6.0));
    assert!(f64_eq(p.m[1][0], 5.0 + 10.0));
    assert!(f64_eq(p.m[1][1], 10.0 + 4.0));
}

#[test]
fn matrix_eq() {
    let m = Matrix4x4::new();
    let n = Matrix4x4::new();

    assert!(m.eq(&n));
    
    let o = Matrix3x3::new();
    let p = Matrix3x3::new();
    assert!(p.eq(&o));

    let q = Matrix3x3::new();
    let r = Matrix3x3::new();
    assert!(q.eq(&r));

    let mut s = Matrix3x3::new();
    s.m[0][0] = 2.0;

    let t = Matrix3x3::new();
    assert!(!s.eq(&t));
    assert!(!t.eq(&s));




}



