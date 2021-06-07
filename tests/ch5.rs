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
    let mut p = r.origin() + Vector4D::new_vector(2.0, 4.0, 6.0);
    assert_vector4d_eq!(r.at_t(2.0), p);
    p =r.origin() - Vector4D::new_vector(1.0, 2.0, 3.0); 
    assert_vector4d_eq!(r.at_t(-1.0), p);
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
    assert_eq!(intersections[0].obj.eq(&Shape::Sphere(s.clone())), true);
    assert_eq!(intersections[1].obj.eq(&Shape::Sphere(s.clone())), true);
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
    assert_eq!(intersections[0].obj.eq(&Shape::Sphere(s.clone())), true);
    assert_eq!(intersections[1].obj.eq(&Shape::Sphere(s.clone())), true);
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
fn ray_intersect_sphere_ray_inside() {
    let r = Ray::new(Vector4D::new_point(0.0, 0.0, 0.0), 
                Vector4D::new_vector(0.0, 0.0, 1.0));
    let s = Sphere::new();
    let intersections = r.intersect(&s);
    assert_eq!(intersections.len(), 2);
    assert_f64_eq!(intersections[0].t, -1.0);
    assert_f64_eq!(intersections[1].t, 1.0);
}

#[test]
fn intersection_new() {
    let s = Shape::Sphere(Sphere::new());
    let intersection = Intersection {
        obj: Box::new(s.clone()),
        t: 3.5
    };
    match *intersection.obj {
        Shape::Sphere(ref sphere) => {
            assert!(sphere.eq(&s));
        },
        _ => {} 
    }
    assert_f64_eq!(intersection.t, 3.5);
}

#[test]
fn test_hit() {
    let s = Sphere::new();
    let i1 = Intersection {
        obj: Box::new(Shape::Sphere(s.clone())),
        t: 1.0
    };
    let i2 = Intersection {
        obj: Box::new(Shape::Sphere(s.clone())),
        t: 2.0
    };

    let intersections : Intersections = vec![i2, i1];
    let i = hit(&intersections).unwrap();
    assert!(i.obj.eq(&Shape::Sphere(s)));
    assert_f64_eq!(i.t, 1.0);
}

#[test]
fn test_hit_negative() {
    let s = Sphere::new();
    let i1 = Intersection {
        obj: Box::new(Shape::Sphere(s.clone())),
        t: -1.0
    };
    let i2 = Intersection {
        obj: Box::new(Shape::Sphere(s.clone())),
        t: 1.0
    };

    let intersections : Intersections = vec![i2, i1];
    let i = hit(&intersections).unwrap();
    assert!(i.obj.eq(&Shape::Sphere(s)));
    assert_f64_eq!(i.t, 1.0);
}

#[test]
fn test_hit_all_negative() {
    let s = Sphere::new();
    let i1 = Intersection {
        obj: Box::new(Shape::Sphere(s.clone())),
        t: -1.0
    };
    let i2 = Intersection {
        obj: Box::new(Shape::Sphere(s.clone())),
        t: -2.0
    };

    let intersections : Intersections = vec![i2, i1];
    let i = hit(&intersections);
    assert!(i.is_none());
}

#[test]
fn test_ray_transform_translation() {
    let r = Ray::new(Vector4D::new_point(1.0, 2.0, 3.0),
                     Vector4D::new_vector(0.0, 1.0, 0.0));
    let m = MatrixChainer::new()
        .then(Matrix4x4::translation(3.0, 4.0, 5.0))
        .finish();
    let r2 = r.transform(&m);
    assert!(r2.origin.eq(&Vector4D::new_point(4.0, 6.0, 8.0)));
    assert!(r2.direction.eq(&Vector4D::new_vector(0.0, 1.0, 0.0)));
}

#[test]
fn test_ray_transform_scaling() {
    let r = Ray::new(Vector4D::new_point(1.0, 2.0, 3.0),
                     Vector4D::new_vector(0.0, 1.0, 0.0));
    let m = MatrixChainer::new()
        .then(Matrix4x4::scaling(2.0, 3.0, 4.0))
        .finish();
    let r2 = r.transform(&m);
    assert!(r2.origin.eq(&Vector4D::new_point(2.0, 6.0, 12.0)));
    assert!(r2.direction.eq(&Vector4D::new_vector(0.0, 3.0, 0.0)));
}

#[test]
fn test_sphere_set_transform() {
    let mut s = Sphere::new();
    let m = Matrix4x4::translation(2.0, 3.0, 4.0);
    s.set_transform(m);
    assert_f64_eq!(s.transform.m[0][3], 2.0);
    assert_f64_eq!(s.transform.m[1][3], 3.0);
    assert_f64_eq!(s.transform.m[2][3], 4.0);
}

#[test]
fn test_ray_intersect_scaled_sphere() {
    let r = Ray::new(Vector4D::new_point(0.0, 0.0, -5.0),
                     Vector4D::new_vector(0.0, 0.0, 1.0));
    let mut s = Sphere::new();
    s.set_transform(Matrix4x4::scaling(2.0, 2.0, 2.0));
    let xs = r.intersect(&s);
    assert_eq!(xs.len(), 2);
    assert_f64_eq!(xs[0].t, 3.0);
    assert_f64_eq!(xs[1].t, 7.0);
} 


#[test]
#[ignore = "slow test"]
fn test_raytrace_sphere() {
    let ray_origin = Vector4D::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_width_pixels = 400.0;
    let pixel_size = wall_size / canvas_width_pixels;
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_width_pixels as usize, canvas_width_pixels as usize);
    let color = Color::new(1.0, 0.0, 0.0);
    let shape = Sphere::new();

    for y in 0..(canvas_width_pixels as usize  - 1) {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..(canvas_width_pixels as usize - 1) {
            let world_x = -half + pixel_size * (x as f64);
            let pos = Vector4D::new_point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (pos - ray_origin).normalized());
            let xs = ray.intersect(&shape);
            match hit(&xs) {
                Some(_) => {
                    canvas.set_pixel(x, y, &color);
                },
                None => {}
            }
        }
    }
    canvas.write_ppm("ch5.ppm").unwrap();
}

#[test]
#[ignore = "slow test"]
fn test_raytrace_sphere_scaled() {
    let ray_origin = Vector4D::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_width_pixels = 400.0;
    let pixel_size = wall_size / canvas_width_pixels;
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_width_pixels as usize, canvas_width_pixels as usize);
    let color = Color::new(1.0, 0.0, 0.0);
    let mut shape = Sphere::new();
    shape.set_transform(Matrix4x4::scaling(1.0, 0.5, 1.0));

    for y in 0..(canvas_width_pixels as usize  - 1) {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..(canvas_width_pixels as usize - 1) {
            let world_x = -half + pixel_size * (x as f64);
            let pos = Vector4D::new_point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (pos - ray_origin).normalized());
            let xs = ray.intersect(&shape);
            match hit(&xs) {
                Some(_) => {
                    canvas.set_pixel(x, y, &color);
                },
                None => {}
            }
        }
    }
    canvas.write_ppm("ch5_sphere.ppm").unwrap();
}
