use utils::*;
use types::*;
use std::f64::consts::PI;
extern crate raytracer_challenge_rs;
use raytracer_challenge_rs::{Canvas, render_with_reflection};
use types::refractive_indices;


#[test]
fn test_ray_misses_cylinder() {
    // (ray origin, ray direction)
    let test_cases: Vec<(Vector4D, Vector4D)> = vec![
        (Vector4D::new_point(1.0, 0.0, 0.0), Vector4D::new_vector(0.0, 1.0, 0.0)),
        (Vector4D::new_point(0.0, 0.0, 0.0), Vector4D::new_vector(0.0, 1.0, 0.0)),
        (Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(1.0, 1.0, 1.0)),
    ];

    let cylinder = Cylinder::new();
    for (o, d) in test_cases {
        let ray = Ray::new(o, d);
        let xs = cylinder.intersect(&ray);
        assert_eq!(xs.len(), 0)
    }
}


#[test]
fn test_ray_intersects_cylinder() {
    // (ray origin, ray direction, t1, t2)
    let test_cases: Vec<(Vector4D, Vector4D, f64, f64)> = vec![
        (Vector4D::new_point(1.0, 0.0, -5.0), Vector4D::new_vector(0.0, 0.0, 1.0), 5.0, 5.0),
        (Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(0.0, 0.0, 1.0), 4.0, 6.0),
        // The following is bad needs to be adjusted a bit
        //(Vector4D::new_point(0.5, 0.0, -5.0), Vector4D::new_vector(0.1, 1.0, 1.0), 6.80798, 7.08872),
        (Vector4D::new_point(0.5, 0.0, -5.0), Vector4D::new_vector(0.1, 1.0, 1.0), 4.80198, 5.0),
    ];

    let cylinder = Cylinder::new();
    for (o, d, t1, t2) in test_cases {
        let ray = Ray::new(o, d);
        let xs = cylinder.intersect(&ray);
        println!("xs:{:?}", xs);
        assert_eq!(xs.len(), 2);
        assert_f64_eq!(xs[0].t, t1);
        assert_f64_eq!(xs[1].t, t2);
    }
}

#[test]
#[ignore="render"]
fn test_render_gold_cylinder() {
    let mut world: World = Default::default();
    world.objects.clear();

    let mut obj = Cylinder::new_truncated(-1.0, 1.0, true);
    obj.set_transform(MatrixChainer::new()
                        .then(Matrix4x4::rotate_x(-PI/3.0))
                        .then(Matrix4x4::rotate_z(PI/6.0))
                       .then(Matrix4x4::translation(0.2, -2.0, 0.3))
                       .then(Matrix4x4::scaling(1.7, 1.7, 1.7))
                       .finish());
    let mut obj_mat: Material = Default::default();
    obj_mat.color = Color::new(1.0, 0.8431, 0.0);
    obj_mat.refractive_index = refractive_indices::GLASS;
    obj_mat.reflective = 0.9;
    obj_mat.transparency = 0.8;
    obj.set_material(obj_mat);

    world.objects.push(Shape::Cylinder(obj));

    let mut wall = Plane::new();
    wall.set_transform(MatrixChainer::new()
                       .then(Matrix4x4::translation(0.0, -6.0, 0.0))
                       .finish());
    let mut wall_mat: Material = Default::default();
    wall_mat.color = Color::new(1.0, 0.8431, 0.0);
    wall_mat.specular = 0.8;
    wall_mat.transparency = 0.8;
    wall_mat.refractive_index = 1.83;
    wall_mat.pattern = Some(Box::new(Pattern::CheckeredPattern(CheckeredPattern::new(Color::new(1.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0)))));
    wall_mat.no_cast_shadow = true;
    wall.set_material(wall_mat);
    world.objects.push(Shape::Plane(wall));

    let mut canvas = Canvas::new(WIDTH_PX, HEIGHT_PX);
    const WIDTH_PX: usize = 800;
    const HEIGHT_PX: usize = 600;
    world.light_source = LightSource::new(Color::new(1.0, 1.0, 1.0), 
                                          Vector4D::new_point(-10.0, 10.0, -10.0));

    let mut c = Camera::new(WIDTH_PX, HEIGHT_PX, PI/3.0);
    let from = Vector4D::new_point(0.0, 5.0, 0.0);
    let to = Vector4D::new_point(0.0, -1.0, 0.0);
    let up = Vector4D::new_vector(0.0, 0.0, 1.0);
    c.transform = view_transformation(from, to, up); 
    render_with_reflection(&c, &world, &mut canvas);

    canvas.write_ppm("ch13_gold_cylinder.ppm").unwrap();
}

#[test]
fn test_cone_intersect() {
    // (ray origin, ray direction, t0, t1)
    let test_cases: Vec<(Vector4D, Vector4D, f64, f64)> = vec![
        (Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(0.0, 0.0, 1.0), 5.0, 5.0),
        (Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(1.0, 1.0, 1.0), 8.66025, 8.66025),
        (Vector4D::new_point(1.0, 1.0, -5.0), Vector4D::new_vector(-0.5, -1.0, 1.0), 4.55006, 49.44994),
    ];

    let cone = Cone::new();
    for (o, d, t0, t1) in test_cases {
        let ray = Ray::new(o, d.normalized());
        println!("ray: {:?}", ray);
        let xs = cone.intersect(&ray);
        assert_eq!(xs.len(), 2);
        assert_f64_eq!(t0, xs[0].t);
        assert_f64_eq!(t1, xs[1].t);
    }
}

#[test]
fn test_cone_intersect_parallel_to_a_half() {
    let cone = Cone::new();
    let ray = Ray::new(Vector4D::new_point(0.0, 0.0, -1.0), Vector4D::new_vector(0.0, 1.0, 1.0).normalized());
    let xs = cone.intersect(&ray);
    assert_eq!(xs.len(), 1);
    assert_f64_eq!(xs[0].t, 0.35355);
}

#[test]
fn test_cone_normal() {
    let test_cases: Vec<(Vector4D, Vector4D)> = vec![
        (Vector4D::new_point(0.0, 0.0, 0.0), Vector4D::new_vector(0.0, 0.0, 0.0)),
        (Vector4D::new_point(1.0, 1.0, 1.0), Vector4D::new_vector(1.0, -2.0f64.sqrt(), 1.0)),
        (Vector4D::new_point(-1.0, -1.0, 0.0), Vector4D::new_vector(-1.0, 1.0, 0.0)),
    ];

    let c = Cone::new();
    for (p, n) in test_cases {
        let tn = c.normal_at_local(p);
        assert_vector4d_eq!(tn, n);
    }

}

#[test]
fn test_cone_end_caps() {
    let test_cases: Vec<_> = vec![
        //(Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(0.0, 1.0, 0.0), 0),
        (Vector4D::new_point(1.3999999, 0.0, 0.0), Vector4D::new_vector(0.0, 1.0, 0.0), 4),
        //(Vector4D::new_point(0.0, 0.0, -0.25), Vector4D::new_vector(0.0, 1.0, 0.0), 4),
    ];

    let cone = Cone::new_truncated(-1.4, 1.4, true);
    for (o, d, c) in test_cases {
        let ray = Ray::new(o, d.normalized());
        let mut xs = cone.intersect(&ray);
        xs.dedup_by(|a, b| f64_eq(a.t, b.t));
        if xs.len() > 0 {
            for (i, xs) in xs.iter().enumerate() {
            println!("xs.t: {}", xs.t);
            let p = ray.origin() + xs.t * ray.dir();
            let n = cone.normal_at_local(p);
            println!("{} point of intersection: {:?} normal: {:?}", i, p, n);
            }
        }
        //assert_eq!(xs.len(), c);
    }
}
#[test]
#[ignore="render"]
fn test_render_gold_cone() {
    let mut world: World = Default::default();
    world.objects.clear();

    let mut obj = Cone::new_truncated(-1.5, 1.5, true);
    //let mut obj = Cone::new();
    obj.set_transform(MatrixChainer::new()
                        .then(Matrix4x4::rotate_x(-PI/6.0))
                        .then(Matrix4x4::rotate_z(PI/2.0))
                        //.then(Matrix4x4::rotate_z(PI/2.0))
                       .then(Matrix4x4::translation(0.5, -1.0, 0.5))
                       .then(Matrix4x4::scaling(1.0, 1.0, 1.0))
                       .finish());
    let mut obj_mat: Material = Default::default();
    obj_mat.color = Color::new(1.0, 0.8431, 0.0);
    obj_mat.refractive_index = refractive_indices::DEFAULT; 
    obj_mat.reflective = 0.0;
    obj_mat.ambient = 0.3;
    //obj_mat.transparency = 0.8;
    obj.set_material(obj_mat);

    world.objects.push(Shape::Cone(obj));

    let mut wall = Plane::new();
    wall.set_transform(MatrixChainer::new()
                       .then(Matrix4x4::translation(0.0, -6.0, 0.0))
                       .finish());
    let mut wall_mat: Material = Default::default();
    wall_mat.color = Color::new(1.0, 1.0, 1.0);
    wall_mat.specular = 0.8;
    wall_mat.transparency = 0.0;
    wall_mat.reflective = 0.0;
    wall_mat.refractive_index = refractive_indices::DEFAULT; 
    //wall_mat.pattern = Some(Box::new(Pattern::CheckeredPattern(CheckeredPattern::new(Color::new(1.0, 0.0, 0.0), Color::new(1.0, 1.0, 1.0)))));
    wall_mat.no_cast_shadow = true;
    wall.set_material(wall_mat);
    world.objects.push(Shape::Plane(wall));

    let mut canvas = Canvas::new(WIDTH_PX, HEIGHT_PX);
    const WIDTH_PX: usize = 700;
    const HEIGHT_PX: usize = 500;
    world.light_source = LightSource::new(Color::new(1.0, 1.0, 1.0), 
                                          // shadow glitches
                                          Vector4D::new_point(-20.0, 10.0, -5.0));
                                          //Vector4D::new_point(0.0, 10.0, 0.0));

    let mut c = Camera::new(WIDTH_PX, HEIGHT_PX, PI/3.0);
    let from = Vector4D::new_point(0.0, 5.0, 0.0);
    let to = Vector4D::new_point(0.0, -1.0, 0.0);
    let up = Vector4D::new_vector(0.0, 0.0, 1.0);
    c.transform = view_transformation(from, to, up); 
    render_with_reflection(&c, &world, &mut canvas);

    canvas.write_ppm("ch13_gold_cone.ppm").unwrap();
}

