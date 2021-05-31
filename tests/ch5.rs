use utils::*;
use types::*;
extern crate raytracer_challenge_rs;
use raytracer_challenge_rs::Canvas;

#[test]
fn ray_new() {
    let r = Ray::new(Vector4D::new_point(1.0, 2.0, 3.0), 
                     Vector4D::new_vector(1.0, 2.0, 3.0));
    assert_vector4d_eq!(r.origin(), Vector4D::new_point(1.0, 2.0, 3.0));
}

#[test]
fn ray_at_t() {
    let r = Ray::new(Vector4D::new_point(1.0, 2.0, 3.0), 
                     Vector4D::new_vector(1.0, 2.0, 3.0));
    assert_vector4d_eq!(r.at_t(2.0), r.origin() + Vector4D::new_vector(2.0, 4.0, 6.0));
    assert_vector4d_eq!(r.at_t(-1.0), r.origin() - Vector4D::new_vector(1.0, 2.0, 3.0));
}


#[test]
fn ray_intersect_sphere() {
    let r = Ray::new(Vector4D::new_point(0.0, 0.0, -5.0),
                     Vector4D::new_vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let intersections = r.intersect(&s);
    assert_eq!(intersections.len(), 2);
    assert_f64_eq!(intersections[0].t, 4.0);
    assert_f64_eq!(intersections[1].t, 6.0);
}

#[test]
fn ray_intersect_sphere_tangent() {
    let r = Ray::new(Vector4D::new_point(0.0, 1.0, -5.0),
                     Vector4D::new_vector(0.0, 0.0, 1.0));
    let s = Sphere::new();

    let intersections = r.intersect(&s);
    assert_eq!(intersections.len(), 2);
    assert_f64_eq!(intersections[0].t, 5.0);
    assert_f64_eq!(intersections[1].t, 5.0);
}

#[test]
fn ray_intersect_spehere_misses() {
    let r = Ray::new(Vector4D::new_point(0.0, 2.0, -5.0),
                     Vector4D::new_vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let intersections = r.intersect(&s);
    assert_eq!(intersections.len(), 0);
}

#[test]
fn intersection_new() {
    let s = Shape::Sphere(Sphere::new());
    let intersection = Intersection {
        obj: Box::new(s),
        t: 3.5
    };
    match *intersection.obj {
        Shape::Sphere(ref sphere) => {
            assert!(sphere.eq(s));
        },
        _ => { panic!("not equal to sphere s"); }
    }
    assert_f64_eq!(intersection.t, 3.5);
}
