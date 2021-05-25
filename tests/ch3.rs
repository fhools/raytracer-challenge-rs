use types::*;
use utils::*;
use std::cmp::Eq;

#[test]
fn matrix4x4_new() {
    let m = Matrix4x4::new();
    assert!(m.m[0][0] == 1.0);
}
