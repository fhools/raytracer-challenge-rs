use utils::*;
use types::*;
use std::f64::consts::PI;
extern crate raytracer_challenge_rs;
use raytracer_challenge_rs::{Canvas, render_with_reflection};
use types::refractive_indices;

#[test]
fn test_ray_intersect_cube() {
    // (ray origin, ray direction, t1, t2)
    let test_cases: Vec<(Vector4D, Vector4D, f64, f64)> = vec![
        (Vector4D::new_point(5.0, 0.5, 0.0), Vector4D::new_vector(-1.0, 0.0, 0.0), 4.0, 6.0), // +x
        (Vector4D::new_point(-5.0, 0.5, 0.0), Vector4D::new_vector(1.0, 0.0, 0.0), 4.0, 6.0), // -x
        
        (Vector4D::new_point(0.5, 5.0, 0.0), Vector4D::new_vector(0.0, -1.0, 0.0), 4.0, 6.0), // +y
        (Vector4D::new_point(0.5, -5.0, 0.0), Vector4D::new_vector(0.0, 1.0, 0.0), 4.0, 6.0), // -y

        (Vector4D::new_point(0.5, 0.0, 5.0), Vector4D::new_vector(0.0, 0.0, -1.0), 4.0, 6.0), // +z
        (Vector4D::new_point(0.5, 0.0, -5.0), Vector4D::new_vector(0.0, 0.0, 1.0), 4.0, 6.0), // -z 
    ];


    let cube = Cube::new();
    for (p, v, t1, t2) in test_cases {
        let ray = Ray::new(p, v);
        let xs = cube.intersect(&ray);
        assert_eq!(xs.len(), 2);
        assert_f64_eq!(xs[0].t, t1);
        assert_f64_eq!(xs[1].t, t2);
    }
}

#[test]
fn test_ray_doesnt_intersect_cube() {
    // (ray origin, ray direction)
    let test_cases: Vec<(Vector4D, Vector4D)> = vec![
        (Vector4D::new_point(-2.0, 0.0, 0.0), Vector4D::new_vector(0.2673, 0.5345, 0.8018)),
        (Vector4D::new_point(0.0, -2.0, 0.0), Vector4D::new_vector(0.8018, 0.2673, 0.5345)),
        (Vector4D::new_point(0.0, 0.0, -2.0), Vector4D::new_vector(0.5345, 0.8018, 0.2673)),
        (Vector4D::new_point(2.0, 0.0, 2.0), Vector4D::new_vector(0.0, 0.0, -1.0)),
        (Vector4D::new_point(0.0, 2.0, 2.0), Vector4D::new_vector(0.0, -1.0, 0.0)),
        (Vector4D::new_point(2.0, 2.0, 0.0), Vector4D::new_vector(-1.0, 0.0, 0.0)),
    ];

    let cube = Cube::new();
    for (p, v) in test_cases {
        let ray = Ray::new(p, v);
        let xs = cube.intersect(&ray);
        assert_eq!(xs.len(), 0);
    }
}

#[test]
fn test_cube_normal_at() {
    // (point, normal)
    let test_cases: Vec<(Vector4D, Vector4D)> = vec![
        (Vector4D::new_point(1.0, 0.5, -0.8), Vector4D::new_vector(1.0, 0.0, 0.0)),
        (Vector4D::new_point(-1.0, -0.2, 0.9), Vector4D::new_vector(-1.0, 0.0, 0.0)),
        (Vector4D::new_point(-0.4, 1.0, -0.1), Vector4D::new_vector(0.0, 1.0, 0.0)),
        (Vector4D::new_point(0.3, -1.0, -0.7), Vector4D::new_vector(0.0, -1.0, 0.0)),
        (Vector4D::new_point(-0.6, 0.3, 1.0), Vector4D::new_vector(0.0, 0.0, 1.0)),
        (Vector4D::new_point(0.4, 0.4, -1.0), Vector4D::new_vector(0.0, 0.0, -1.0)),
        (Vector4D::new_point(1.0, 1.0, 1.0), Vector4D::new_vector(1.0, 0.0, 0.0)),
        (Vector4D::new_point(-1.0, -1.0, -1.0), Vector4D::new_vector(-1.0, 0.0, 0.0)),
    ];

    let cube = Cube::new();
    for (p, expected_norm) in test_cases {
        let norm = cube.normal_at(p);
        assert_vector4d_eq!(norm, expected_norm);
    }
}


#[test]
#[ignore="render"]
fn test_render_gold_cube() {
    let mut world: World = Default::default();
    world.objects.clear();

    let mut obj = Cube::new();
    obj.set_transform(MatrixChainer::new()
                        .then(Matrix4x4::rotate_x(PI/6.0))
                       .then(Matrix4x4::translation(0.2, -2.0, 0.3))
                       .then(Matrix4x4::scaling(1.7, 1.7, 1.7))
                       .finish());
    let mut obj_mat: Material = Default::default();
    obj_mat.color = Color::new(1.0, 0.8431, 0.0);
    obj_mat.reflective = 0.4;
    obj_mat.refractive_index = refractive_indices::GLASS;
    obj_mat.transparency = 1.0;
    obj.set_material(obj_mat);

    world.objects.push(Shape::Cube(obj));

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
                                          Vector4D::new_point(-10.0, 10.0, -25.0));

    let mut c = Camera::new(WIDTH_PX, HEIGHT_PX, PI/3.0);
    let from = Vector4D::new_point(0.0, 5.0, 0.0);
    let to = Vector4D::new_point(0.0, -1.0, 0.0);
    let up = Vector4D::new_vector(0.0, 0.0, 1.0);
    c.transform = view_transformation(from, to, up); 
    render_with_reflection(&c, &world, &mut canvas);

    canvas.write_ppm("ch12_gold_cube.ppm").unwrap();
}
