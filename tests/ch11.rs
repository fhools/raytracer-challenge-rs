use utils::*;
use types::*;
use std::f64::consts::PI;
extern crate raytracer_challenge_rs;
use raytracer_challenge_rs::{Canvas, render_with_reflection};
#[test]
fn test_prepare_computations_produce_reflectv() {
    let plane = Plane::new();
    let ray = Ray::new(Vector4D::new_point(0.0, 1.0, -1.0),
                       Vector4D::new_vector(0.0, -2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0));
    let i = Intersection {
            t: 2.0f64.sqrt(),
            obj: Box::new(Shape::Plane(plane))
    };

    let comps = ray.prepare_computations(&i);
    assert_vector4d_eq!(comps.reflectv, Vector4D::new_vector(0.0, 2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0));
}

#[test]
fn test_reflected_color_no_reflexivity() {
    let mut w : World = Default::default();
    let r = Ray::new(Vector4D::new_point(0.0, 0.0, 0.0), Vector4D::new_vector(0.0, 0.0, 1.0));
    let mut o = w.objects[1].clone();
    let mut m = o.get_material();
    m.ambient = 1.0;
    o.set_material(&m);
    w.objects[1] = o.clone();
    let i = Intersection { 
        t: 1.0,
        obj: Box::new(o.clone())
    };
    let comps  = r.prepare_computations(&i);
    let color = w.reflected_color(&comps, 1);
    assert_color_eq!(color, Color::BLACK);
}

#[test]
fn test_reflected_color_reflective_material() {
    let mut w : World = Default::default();
    let r = Ray::new(Vector4D::new_point(0.0, 0.0, -3.0), Vector4D::new_vector(0.0, -2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0));
    let mut plane = Plane::new();
    plane.set_transform(Matrix4x4::translation(0.0, -1.0, 0.0));
    let mut m = plane.get_material();
    m.reflexivity = 0.5;
    plane.set_material(m);
    w.objects.push(Shape::Plane(plane));
    let i = Intersection { 
        t: 2.0f64.sqrt(),
        obj: Box::new(w.objects[2].clone())
    };
    let comps  = r.prepare_computations(&i);
    let color = w.reflected_color(&comps, 1);
    assert_color_eq!(color,  Color::new(0.19033, 0.23791, 0.14274));
}

#[test]
fn test_raytrace_with_camera_multiple_spheres_checkered_pattern_reflection() {
    const WIDTH_PX: usize = 480;
    const HEIGHT_PX: usize = 320;
    let mut canvas = Canvas::new(WIDTH_PX, HEIGHT_PX);
    let mut world: World = Default::default();
    world.objects.clear();

    let mut floor_obj = Sphere::new();
    floor_obj.transform = Matrix4x4::scaling(10.0, -0.01, 10.0);
    floor_obj.material.color = Color::new(1.0, 0.9, 0.9);
    floor_obj.material.specular = 0.0; 

    world.objects.push(Shape::Sphere(floor_obj.clone()));

    let mut left_wall_obj = Sphere::new();
    left_wall_obj.transform = MatrixChainer::new()
        .then(Matrix4x4::scaling(10.0, 0.01, 10.0))
        .then(Matrix4x4::rotate_x(PI/2.0))
        .then(Matrix4x4::rotate_y(-PI/4.0))
        .then(Matrix4x4::translation(0.0, 0.0, 5.0))
        .finish();
    left_wall_obj.material = floor_obj.material.clone();
    world.objects.push(Shape::Sphere(left_wall_obj));

    let mut right_wall_obj = Sphere::new();
    right_wall_obj.transform = MatrixChainer::new()
        .then(Matrix4x4::scaling(10.0, 0.01, 10.0))
        .then(Matrix4x4::rotate_x(PI/2.0))
        .then(Matrix4x4::rotate_y(PI/4.0))
        .then(Matrix4x4::translation(0.0, 0.0, 5.0))
        .finish();
    right_wall_obj.material = floor_obj.material.clone();
    world.objects.push(Shape::Sphere(right_wall_obj));
    

    let mut middle = Sphere::new();
    middle.transform = MatrixChainer::new()
        .then(Matrix4x4::rotate_x(PI/4.0))
        .then(Matrix4x4::translation(-0.5, 1.0, 0.5))
        .finish();
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
    let mut pattern : CheckeredPattern = CheckeredPattern::new(Color::red(), Color::green()); 
    pattern.set_transform(MatrixChainer::new()
                          .then(Matrix4x4::scaling(0.5, 0.5, 0.5))
                          .finish());
    middle.material.pattern = Some(Box::new(Pattern::CheckeredPattern(pattern)));
    world.objects.push(Shape::Sphere(middle));

    let mut right_sphere = Sphere::new();
    right_sphere.transform = MatrixChainer::new()
        .then(Matrix4x4::scaling(0.6, 0.6, 0.6))
        .then(Matrix4x4::translation(1.5, 0.5, -0.5))
        .finish();

    right_sphere.material.color = Color::new(0.85, 0.3, 0.85);
    right_sphere.material.diffuse = 0.7;
    right_sphere.material.specular = 0.3;
    right_sphere.material.reflexivity = 0.8;
    world.objects.push(Shape::Sphere(right_sphere));

    let mut left_sphere = Sphere::new();
    left_sphere.transform = MatrixChainer::new()
        .then(Matrix4x4::scaling(0.33, 0.33, 0.33))
        .then(Matrix4x4::translation(-1.5, 0.33, -0.75))
        .finish();

    left_sphere.material.color = Color::new(1.0, 0.8, 0.1);
    left_sphere.material.diffuse = 0.7;
    left_sphere.material.specular = 0.3;
    world.objects.push(Shape::Sphere(left_sphere));
    
    world.light_source = LightSource::new(Color::new(1.0, 1.0, 1.0), 
                                          Vector4D::new_point(-10.0, 10.0, -10.0));

    let mut c = Camera::new(WIDTH_PX, HEIGHT_PX, PI/3.0);
    let from = Vector4D::new_point(0.0, 1.5, -5.0);
    let to = Vector4D::new_point(0.0, 1.0, 0.0);
    let up = Vector4D::new_vector(0.0, 1.0, 0.0);
    c.transform = view_transformation(from, to, up); 
    render_with_reflection(&c, &world, &mut canvas);

    canvas.write_ppm("ch11_checkered_pattern_reflect.ppm").unwrap();
}
