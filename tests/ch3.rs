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
fn matrix4x4_submatrix() {
    let m = Matrix4x4::from_vector(&vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0, 10.0, 11.0, 12.0, 13.0, 14.0, 15.0, 16.0]);

    let n = m.submatrix(1,1);

    assert_f64_eq!(n.m[0][0], 1.0);
    assert_f64_eq!(n.m[0][1], 3.0);
    assert_f64_eq!(n.m[0][2], 4.0);
    assert_f64_eq!(n.m[1][0], 9.0);
    assert_f64_eq!(n.m[1][1], 11.0);
    assert_f64_eq!(n.m[1][2], 12.0);
    assert_f64_eq!(n.m[2][0], 13.0);
    assert_f64_eq!(n.m[2][1], 15.0);
    assert_f64_eq!(n.m[2][2], 16.0);
}

#[test]
fn matrix3x3_submatrix() {
    let m = Matrix3x3::from_vector(&vec![1.0, 2.0, 3.0, 4.0, 5.0, 6.0, 7.0, 8.0, 9.0]);

    let n = m.submatrix(1,1);

    assert!(f64_eq(n.m[0][0], 1.0));
    assert!(f64_eq(n.m[0][1], 3.0));
    assert!(f64_eq(n.m[1][0], 7.0));
    assert!(f64_eq(n.m[1][1], 9.0));
}

#[test]
fn matrix3x3_minor() {
    let m = Matrix3x3::from_vector(&vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
    let minor_1_0 = m.minor(1, 0);
    assert_f64_eq!(minor_1_0, 25.0);
}

#[test]
fn matrix3x3_cofactor() {
    let m = Matrix3x3::from_vector(&vec![3.0, 5.0, 0.0, 2.0, -1.0, -7.0, 6.0, -1.0, 5.0]);
    assert_f64_eq!(m.cofactor(0, 0), -12.0);
    assert_f64_eq!(m.cofactor(1, 0), -25.0);
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

#[test]
fn matrix2x2_transpose() {
    let mut m = Matrix2x2::new();
    m.m[0][0] = 2.0;
    m.m[0][1] = 3.0;
    m.m[1][0] = 5.0;
    m.m[1][1] = 4.0;

    let n = m.transpose();

    assert!(f64_eq(n.m[0][0], 2.0));
    assert!(f64_eq(n.m[0][1], 5.0));
    assert!(f64_eq(n.m[1][0], 3.0));
    assert!(f64_eq(n.m[1][1], 4.0));
}

#[test]
fn matrix3x3_transpose() {
    let mut m = Matrix3x3::new();
    m.m[0][0] = 1.0;
    m.m[0][1] = 2.0;
    m.m[0][2] = 3.0;
    m.m[1][0] = 4.0;
    m.m[1][1] = 5.0;
    m.m[1][2] = 6.0;
    m.m[2][0] = 7.0;
    m.m[2][1] = 8.0;
    m.m[2][2] = 9.0;

    let n = m.transpose();

    assert!(f64_eq(n.m[0][0], 1.0));
    assert!(f64_eq(n.m[0][1], 4.0));
    assert!(f64_eq(n.m[0][2], 7.0));
    assert!(f64_eq(n.m[1][0], 2.0));
    assert!(f64_eq(n.m[1][1], 5.0));
    assert!(f64_eq(n.m[1][2], 8.0));
    assert!(f64_eq(n.m[2][0], 3.0));
    assert!(f64_eq(n.m[2][1], 6.0));
    assert!(f64_eq(n.m[2][2], 9.0));
}

#[test]
fn matrix4x4_transpose() {
    let mut m = Matrix4x4::new();
    m.m[0][0] = 1.0;
    m.m[0][1] = 2.0;
    m.m[0][2] = 3.0;
    m.m[0][3] = 4.0;
    m.m[1][0] = 5.0;
    m.m[1][1] = 6.0;
    m.m[1][2] = 7.0;
    m.m[1][3] = 8.0;
    m.m[2][0] = 9.0; 
    m.m[2][1] = 10.0;
    m.m[2][2] = 11.0;
    m.m[2][3] = 12.0;
    m.m[3][0] = 13.0;
    m.m[3][1] = 14.0;
    m.m[3][2] = 15.0;
    m.m[3][3] = 16.0;

    let n = m.transpose();

    assert!(f64_eq(n.m[0][0], 1.0));
    assert!(f64_eq(n.m[0][1], 5.0));
    assert!(f64_eq(n.m[0][2], 9.0));
    assert!(f64_eq(n.m[0][3], 13.0));
    assert!(f64_eq(n.m[1][0], 2.0));
    assert!(f64_eq(n.m[1][1], 6.0));
    assert!(f64_eq(n.m[1][2], 10.0));
    assert!(f64_eq(n.m[1][3], 14.0));
    assert!(f64_eq(n.m[2][0], 3.0));
    assert!(f64_eq(n.m[2][1], 7.0));
    assert!(f64_eq(n.m[2][2], 11.0));
    assert!(f64_eq(n.m[2][3], 15.0));
    assert!(f64_eq(n.m[3][0], 4.0));
    assert!(f64_eq(n.m[3][1], 8.0));
    assert!(f64_eq(n.m[3][2], 12.0));
    assert!(f64_eq(n.m[3][3], 16.0));
}


#[test]
fn matrix2x2_determinant() {
    let m = Matrix2x2::from_vector(&vec![1.0, 5.0, -3.0, 2.0]);
    assert!(f64_eq(m.det(), 17.0));
}


#[test]
fn matrix3x3_determinant() {
    let m = Matrix3x3::from_vector(&vec![1.0, 2.0, 6.0, -5.0, 8.0, -4.0, 2.0, 6.0, 4.0]);
    assert_f64_eq!(m.det(), -196.0);
}

#[test]
fn matrix4x4_determinant() {
    let m = Matrix4x4::from_vector(&vec![-2.0, -8.0, 3.0, 5.0, -3.0, 1.0, 7.0, 3.0, 1.0, 2.0, -9.0, 6.0, -6.0, 7.0, 7.0, -9.0]);
    assert_f64_eq!(m.det(), -4071.0);
}

#[test]
fn matrix3x3_is_invertible() {
    let m = Matrix4x4::from_vector(&vec![-4.0, 2.0, -2.0, -3.0, 9.0, 6.0, 2.0, 6.0, 0.0, -5.0, 1.0, -5.0, 0.0, 0.0, 0.0, 0.0]);
    assert_eq!(false, m.is_invertible());
}

#[test]
fn matrix4x4_inverse() {
    let m = Matrix4x4::from_vector(&vec![8.0, -5.0, 9.0, 2.0, 7.0, 5.0, 6.0, 1.0, -6.0, 0.0, 9.0, 6.0, -3.0, 0.0, -9.0, -4.0]);

    let m_inv = m.inverse();

    assert_eq!(format!("{:.5}", m_inv.m[0][0]), format!("{:.5}", -0.15385));
    assert_eq!(format!("{:.5}", m_inv.m[0][1]), format!("{:.5}",-0.15385));
    assert_eq!(format!("{:.5}", m_inv.m[0][2]), format!("{:.5}",-0.28205));
    assert_eq!(format!("{:.5}", m_inv.m[0][3]), format!("{:.5}",-0.53846));
    assert_eq!(format!("{:.5}", m_inv.m[1][0]), format!("{:.5}",-0.07692));
    assert_eq!(format!("{:.5}", m_inv.m[1][1]), format!("{:.5}",0.12308));
    assert_eq!(format!("{:.5}", m_inv.m[1][2]), format!("{:.5}",0.02564));
    assert_eq!(format!("{:.5}", m_inv.m[1][3]), format!("{:.5}",0.03077));
    assert_eq!(format!("{:.5}", m_inv.m[2][0]), format!("{:.5}",0.35897));
    assert_eq!(format!("{:.5}", m_inv.m[2][1]), format!("{:.5}",0.35897));
    assert_eq!(format!("{:.5}", m_inv.m[2][2]), format!("{:.5}",0.43590));
    assert_eq!(format!("{:.5}", m_inv.m[2][3]), format!("{:.5}",0.92308));
    assert_eq!(format!("{:.5}", m_inv.m[3][0]), format!("{:.5}",-0.69231));
    assert_eq!(format!("{:.5}", m_inv.m[3][1]), format!("{:.5}",-0.69231));
    assert_eq!(format!("{:.5}", m_inv.m[3][2]), format!("{:.5}",-0.76923));
    assert_eq!(format!("{:.5}", m_inv.m[3][3]), format!("{:.5}",-1.92308));

    let n = Matrix4x4::from_vector(&vec![9.0, 3.0, 0.0, 9.0, -5.0, -2.0, -6.0, -3.0, -4.0, 9.0, 6.0, 4.0, -7.0, 6.0, 6.0, 2.0]);
    let n_inv = n.inverse();

    assert_eq!(format!("{:.5}", n_inv.m[0][0]), format!("{:.5}", -0.04074));
    assert_eq!(format!("{:.5}", n_inv.m[0][1]), format!("{:.5}", -0.07778));
    assert_eq!(format!("{:.5}", n_inv.m[0][2]), format!("{:.5}", 0.14444));
    assert_eq!(format!("{:.5}", n_inv.m[0][3]), format!("{:.5}", -0.22222));
    assert_eq!(format!("{:.5}", n_inv.m[1][0]), format!("{:.5}", -0.07778));
    assert_eq!(format!("{:.5}", n_inv.m[1][1]), format!("{:.5}", 0.03333));
    assert_eq!(format!("{:.5}", n_inv.m[1][2]), format!("{:.5}", 0.36667));
    assert_eq!(format!("{:.5}", n_inv.m[1][3]), format!("{:.5}", -0.33333));
    assert_eq!(format!("{:.5}", n_inv.m[2][0]), format!("{:.5}", -0.02901));
    assert_eq!(format!("{:.5}", n_inv.m[2][1]), format!("{:.5}", -0.14630));
    assert_eq!(format!("{:.5}", n_inv.m[2][2]), format!("{:.5}", -0.10926));;
    assert_eq!(format!("{:.5}", n_inv.m[2][3]), format!("{:.5}", 0.12963));
    assert_eq!(format!("{:.5}", n_inv.m[3][0]), format!("{:.5}", 0.17778));
    assert_eq!(format!("{:.5}", n_inv.m[3][1]), format!("{:.5}", 0.06667));
    assert_eq!(format!("{:.5}", n_inv.m[3][2]), format!("{:.5}", -0.26667));
    assert_eq!(format!("{:.5}", n_inv.m[3][3]), format!("{:.5}", 0.33333));
}


#[test]
fn matrix4x4_inverse_and_product() {
    let a = Matrix4x4::from_vector(&vec![3.0, -9.0, 7.0, 3.0, 3.0, -8.0, 2.0, -9.0, -4.0, 4.0, 4.0, 1.0, -6.0, 5.0, -1.0, 1.0]);
    let b = Matrix4x4::from_vector(&vec![8.0, 2.0, 2.0, 2.0, 3.0, -1.0, 7.0, 0.0, 7.0, 0.0, 5.0, 4.0, 6.0, -2.0, 0.0, 5.0]);

    let c = a.mul(&b);

    let should_a = c.mul(&b.inverse());

    assert_f64_eq!(should_a.m[0][0], 3.0);
    assert_f64_eq!(should_a.m[0][1], -9.0);
    assert_f64_eq!(should_a.m[0][2], 7.0);
    assert_f64_eq!(should_a.m[0][3], 3.0);
    assert_f64_eq!(should_a.m[1][0], 3.0);
    assert_f64_eq!(should_a.m[1][1], -8.0);
    assert_f64_eq!(should_a.m[1][2], 2.0);
    assert_f64_eq!(should_a.m[1][3], -9.0);
    assert_f64_eq!(should_a.m[2][0], -4.0);
    assert_f64_eq!(should_a.m[2][1], 4.0);
    assert_f64_eq!(should_a.m[2][2], 4.0);
    assert_f64_eq!(should_a.m[2][3], 1.0);
    assert_f64_eq!(should_a.m[3][0], -6.0);
    assert_f64_eq!(should_a.m[3][1], 5.0);
    assert_f64_eq!(should_a.m[3][2], -1.0);
    assert_f64_eq!(should_a.m[3][3], 1.0);
}
