use utils::*;
use types::*;
use std::f64::consts::PI;
extern crate raytracer_challenge_rs;
use raytracer_challenge_rs::{Canvas, render_with_reflection};
use types::refractive_indices;

#[test]
fn test_new_group() {
    let g = Group::new(0);
    assert_eq!(g.is_empty(), true);
}

#[test]
fn test_testshape_no_parent() {
    let t = TestShape::new();
    assert_eq!(t.get_parent().is_none(), true);
}

#[test]
fn test_group_add() {
    let mut g = Group::new(100);
    let t = TestShape::new();
    g.add_child(Shape::TestShape(t)); 
    let parent = g.children[0].get_parent().unwrap();
    assert_eq!(parent.id, 100);
}

#[test]
fn test_group_intersect() {
    let mut g = Group::new(100);
    let s1 = Sphere::new();
    let mut s2 = Sphere::new();
    s2.set_transform(Matrix4x4::translation(0.0, 0.0, -3.0));

    let mut s3 = Sphere::new();
    s3.set_transform(Matrix4x4::translation(5.0, 0.0, 0.0));

    g.add_child(Shape::Sphere(s1.clone()));
    g.add_child(Shape::Sphere(s2.clone()));
    g.add_child(Shape::Sphere(s3.clone()));
    
    let ray = Ray::new(Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(0.0, 0.0, 1.0));
    let xs = g.intersect(&ray);
    println!("xs: {:?}", xs);
    assert_eq!(xs.len(), 4);
    assert!(xs[0].obj.eq(&Shape::Sphere(s2.clone())));
    assert!(xs[1].obj.eq(&Shape::Sphere(s2.clone())));
    assert!(xs[2].obj.eq(&Shape::Sphere(s1.clone())));
    assert!(xs[3].obj.eq(&Shape::Sphere(s1.clone())));
}


#[test]
fn test_group_transformed_instersect() {
    let mut group_repo = GroupRepository::new();
    let mut g = group_repo.new_group();
    g.set_transform(Matrix4x4::scaling(2.0, 2.0, 2.0));
    let mut s = Sphere::new();
    s.set_transform(Matrix4x4::translation(5.0, 0.0, 0.0));
    g.add_child(Shape::Sphere(s.clone()));
    let ray = Ray::new(Vector4D::new_point(10.0, 0.0, -10.0), Vector4D::new_vector(0.0, 0.0, 1.0));
    let xs = g.intersect(&ray);
    assert_eq!(xs.len(), 2);
}


#[test]
fn test_group_world_to_object_space() {
    let mut s = Sphere::new();
    s.set_transform(Matrix4x4::translation(5.0, 0.0, 0.0));
    let mut g2 = Group::new(100);
    g2.set_transform(Matrix4x4::scaling(2.0, 2.0, 2.0));
    g2.add_child(Shape::Sphere(s));


    let mut g1 = Group::new(101);
    g1.set_transform(Matrix4x4::rotate_y(PI/2.0));
    g1.add_child(Shape::Group(g2));

    match g1.children[0] {
        Shape::Group(ref g) => {
            let point = g.children[0].world_to_object(Vector4D::new_point(-2.0, 0.0, -10.0));
            assert_vector4d_eq!(point, Vector4D::new_point(0.0, 0.0, -1.0));
        }
        _ => {}
    }
}

#[test]
fn test_normal_to_world() {
    let mut s = Sphere::new();
    s.set_transform(Matrix4x4::translation(5.0, 0.0, 0.0));
    let mut g2 = Group::new(100);
    g2.set_transform(Matrix4x4::scaling(1.0, 2.0, 3.0));
    g2.add_child(Shape::Sphere(s));
    let mut g1 = Group::new(101);
    g1.set_transform(Matrix4x4::rotate_y(PI/2.0));
    g1.add_child(Shape::Group(g2));

    match g1.children[0] {
        Shape::Group(ref g) => {
            let normal = g.children[0].normal_to_world(Vector4D::new_vector(3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0, 3.0f64.sqrt()/3.0));
            assert_vector4d_eq!(normal, Vector4D::new_vector(0.2857, 0.4286, -0.8571));
        }
        _ => {}
    }
}
