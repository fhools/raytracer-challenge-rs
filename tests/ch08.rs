use types::*;

#[test]
fn is_shadow_true() {
    let w: World = Default::default();
    let p = Vector4D::new_point(10.0, -10.0, 10.0);
    assert!(w.is_shadowed(p));
}

#[test]
fn is_shadow_light_between_point_and_object() {
    let w: World = Default::default();
    let p = Vector4D::new_point(-20.0, 20.0, -20.0);
    assert!(!w.is_shadowed(p));
}

#[test]
fn is_shadow_object_behind_point() {
    let w: World = Default::default();
    let p = Vector4D::new_point(-2.0, 2.0, -2.0);
    assert!(!w.is_shadowed(p));
}

