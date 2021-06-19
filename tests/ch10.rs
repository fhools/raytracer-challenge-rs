use types::*;
use utils::*;
use std::f64::consts::PI;
extern crate raytracer_challenge_rs;
use raytracer_challenge_rs::{Canvas, render};

#[test]
fn stripe_pattern_constant_y() {
    let sp = StripePattern::new(Color::blue(), Color::white());
    let p1 = sp.pattern_at(Vector4D::new_point(0.0, 0.0, 0.0));
    assert_color_eq!(p1, Color::blue());
    let p2 = sp.pattern_at(Vector4D::new_point(0.0, 1.0, 0.0));
    assert_color_eq!(p2, Color::blue());
    let p3 = sp.pattern_at(Vector4D::new_point(0.0, 2.0, 0.0));
    assert_color_eq!(p3, Color::blue());
}

#[test]
fn stripe_pattern_along_x() {
    let sp = StripePattern::new(Color::blue(), Color::white());
    let p1 = sp.pattern_at(Vector4D::new_point(0.0, 0.0, 0.0));
    assert_color_eq!(p1, Color::blue());
    let p2 = sp.pattern_at(Vector4D::new_point(0.9, 0.0, 0.0));
    assert_color_eq!(p2, Color::blue());
    let p3 = sp.pattern_at(Vector4D::new_point(1.0, 0.0, 0.0));
    assert_color_eq!(p3, Color::white());
    let p4 = sp.pattern_at(Vector4D::new_point(1.3, 0.0, 0.0));
    assert_color_eq!(p4, Color::white());
    let p5 = sp.pattern_at(Vector4D::new_point(2.0, 2.0, 0.0));
    assert_color_eq!(p5, Color::blue());
}

#[test]
fn pattern_at_object_object_transform() {
    let mut s = Sphere::new();
    s.set_transform(Matrix4x4::scaling(2.0, 2.0, 2.0));
    let p : StripePattern = Default::default();
    let c = p.pattern_at_object(&s, Vector4D::new_point(1.5, 0.0, 0.0));
    assert_color_eq!(c, Color::white());
}

#[test]
fn pattern_at_object_pattern_transform() {
    let s = Sphere::new();
    let mut p : StripePattern = Default::default();
    p.set_transform(Matrix4x4::scaling(2.0, 2.0, 2.0));
    let c = p.pattern_at_object(&s, Vector4D::new_point(1.5, 0.0, 0.0));
    assert_color_eq!(c, Color::white());
}

#[test]
fn pattern_at_object_object_and_pattern_transform() {
    let mut s = Sphere::new();
    s.set_transform(Matrix4x4::scaling(2.0, 2.0, 2.0));
    let mut p : StripePattern = Default::default();
    p.set_transform(Matrix4x4::translation(0.5, 0.0, 0.0));
    let c = p.pattern_at_object(&s, Vector4D::new_point(2.5, 0.0, 0.0));
    assert_color_eq!(c, Color::white());
}

#[test]
#[ignore="render"]
fn test_raytrace_with_camera_multiple_spheres_pattern() {
    const WIDTH_PX: usize = 320;
    const HEIGHT_PX: usize = 240;
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
    let mut pattern : RingPattern = Default::default();
    pattern.set_transform(MatrixChainer::new()
                          .then(Matrix4x4::scaling(0.1, 0.1, 0.1))
                          .finish());
    middle.material.pattern = Some(Box::new(Pattern::RingPattern(pattern)));
    world.objects.push(Shape::Sphere(middle));

    let mut right_sphere = Sphere::new();
    right_sphere.transform = MatrixChainer::new()
        .then(Matrix4x4::scaling(0.5, 0.5, 0.5))
        .then(Matrix4x4::translation(1.5, 0.5, -0.5))
        .finish();

    right_sphere.material.color = Color::new(0.5, 1.0, 0.1);
    right_sphere.material.diffuse = 0.7;
    right_sphere.material.specular = 0.3;
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
    render(&c, &world, &mut canvas);

    canvas.write_ppm("ch10_pattern.ppm").unwrap();
}

#[test]
#[ignore="render"]
fn test_raytrace_with_camera_multiple_spheres_gradient_pattern() {
    const WIDTH_PX: usize = 800;
    const HEIGHT_PX: usize = 600;
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
    let mut pattern : GradientPattern = GradientPattern::new(Color::red(), Color::green()); 
    pattern.set_transform(MatrixChainer::new()
                          .then(Matrix4x4::scaling(1.2, 1.5, 1.5))
                          .finish());
    middle.material.pattern = Some(Box::new(Pattern::GradientPattern(pattern)));
    world.objects.push(Shape::Sphere(middle));

    let mut right_sphere = Sphere::new();
    right_sphere.transform = MatrixChainer::new()
        .then(Matrix4x4::scaling(0.5, 0.5, 0.5))
        .then(Matrix4x4::translation(1.5, 0.5, -0.5))
        .finish();

    right_sphere.material.color = Color::new(0.5, 1.0, 0.1);
    right_sphere.material.diffuse = 0.7;
    right_sphere.material.specular = 0.3;
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
    render(&c, &world, &mut canvas);

    canvas.write_ppm("ch10_gradient_pattern.ppm").unwrap();
}

#[test]
#[ignore="render"]
fn test_raytrace_with_camera_multiple_spheres_checkered_pattern() {
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
    render(&c, &world, &mut canvas);

    canvas.write_ppm("ch10_checkered_pattern.ppm").unwrap();
}

#[test]
#[ignore="render"]
fn test_raytrace_with_camera_multiple_checkered_and_test_pattern() {
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
    let mut pattern= TestPattern::new();
    pattern.set_transform(MatrixChainer::new()
                          .then(Matrix4x4::scaling(0.5, 0.5, 0.5))
                          .finish());
    middle.material.pattern = Some(Box::new(Pattern::TestPattern(pattern)));
    world.objects.push(Shape::Sphere(middle));

    let mut right_sphere = Sphere::new();
    right_sphere.transform = MatrixChainer::new()
        .then(Matrix4x4::scaling(0.6, 0.6, 0.6))
        .then(Matrix4x4::translation(1.5, 0.5, -0.5))
        .finish();

    right_sphere.material.color = Color::new(0.85, 0.3, 0.85);
    right_sphere.material.diffuse = 0.7;
    right_sphere.material.specular = 0.3;
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
    render(&c, &world, &mut canvas);

    canvas.write_ppm("ch10_test_pattern.ppm").unwrap();
}


#[test]
fn test_schlick_total_internal_reflection() {
    let shape = Sphere::new_glass();
    let ray = Ray::new(Vector4D::new_point(0.0, 0.0, 2.0f64.sqrt()/2.0), 
                       Vector4D::new_vector(0.0, 1.0, 0.0));
    let xs = vec![Intersection { t: -2.0f64.sqrt()/2.0, obj: Box::new(Shape::Sphere(shape.clone())) },
                  Intersection { t: 2.0f64.sqrt()/2.0, obj: Box::new(Shape::Sphere(shape.clone())) }];
    let sc = ray.prepare_computations(&xs[1], &xs);
    let reflectance = schlick(&sc);
    assert_f64_eq!(reflectance, 1.0);
}

#[test]
fn test_schlick_perpendicular() {
    let shape = Sphere::new_glass();
    let ray = Ray::new(Vector4D::new_point(0.0, 0.0, 0.0), 
                       Vector4D::new_vector(0.0, 1.0, 0.0));
    let xs = vec![Intersection { t: -1.0, obj: Box::new(Shape::Sphere(shape.clone())) },
                  Intersection { t: 1.0, obj: Box::new(Shape::Sphere(shape.clone())) }];
    let sc = ray.prepare_computations(&xs[1], &xs);
    let reflectance = schlick(&sc);
    assert_f64_eq!(reflectance, 0.04);
}

#[test]
fn test_schlick_n2_gt_n1() {
    let shape = Sphere::new_glass();
    let ray = Ray::new(Vector4D::new_point(0.0, 0.99, -2.0), 
                       Vector4D::new_vector(0.0, 0.0, 1.0));
    let xs = vec![Intersection { t: 1.8589, obj: Box::new(Shape::Sphere(shape.clone())) }];
    let sc = ray.prepare_computations(&xs[0], &xs);
    let reflectance = schlick(&sc);
    assert_f64_eq!(reflectance, 0.48873);
}

