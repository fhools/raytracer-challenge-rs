use utils::*;
use types::*;
use std::f64::consts::PI;
extern crate raytracer_challenge_rs;
use raytracer_challenge_rs::{Canvas, render_with_reflection};
use types::refractive_indices;

#[test]
fn test_prepare_computations_produce_reflectv() {
    let plane = Plane::new();
    let ray = Ray::new(Vector4D::new_point(0.0, 1.0, -1.0),
                       Vector4D::new_vector(0.0, -2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0));
    let i = Intersection {
            t: 2.0f64.sqrt(),
            obj: Box::new(Shape::Plane(plane))
    };

    let comps = ray.prepare_computations(&i, &vec![]);
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
    let comps  = r.prepare_computations(&i, &vec![]);
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
    let comps  = r.prepare_computations(&i, &vec![]);
    let color = w.reflected_color(&comps, 1);
    assert_color_eq!(color,  Color::new(0.19033, 0.23791, 0.14274));
}

#[test]
#[ignore="render"]
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

#[test]
fn test_prepare_computations_calculates_refractive_indexes() {
    let test_cases : Vec<(usize, f64, f64)> = 
        vec![(0, 1.0, 1.5), 
             (1, 1.5, 2.0),
             (2, 2.0, 2.5),
             (3, 2.5, 2.5),
             (4, 2.5, 1.5),
             (5, 1.5, 1.0)];

    let ray = Ray::new(Vector4D::new_point(0.0, 0.0, -4.0), Vector4D::new_vector(0.0, 0.0, 1.0));
    let mut sphere_A = Sphere::new_glass();
    let mut sphere_B = Sphere::new_glass();
    let mut sphere_C = Sphere::new_glass();
    sphere_A.set_transform(Matrix4x4::scaling(2.0, 2.0, 2.0));
    sphere_B.set_transform(Matrix4x4::translation(0.0, 0.0, -0.25));
    sphere_C.set_transform(Matrix4x4::translation(0.0, 0.0, 0.25));

    sphere_A.material.refractive_index = 1.5;
    sphere_B.material.refractive_index = 2.0;
    sphere_C.material.refractive_index = 2.5;
    let xs: Intersections = vec![Intersection { 
        t: 2.0,
        obj: Box::new(Shape::Sphere(sphere_A.clone()))
    },
    Intersection {
        t: 2.75,
        obj: Box::new(Shape::Sphere(sphere_B.clone()))
    },
    Intersection {
        t: 3.25,
        obj: Box::new(Shape::Sphere(sphere_C.clone()))
    },
    Intersection {
        t: 4.75,
        obj: Box::new(Shape::Sphere(sphere_B))
    },
    Intersection {
        t: 5.25,
        obj: Box::new(Shape::Sphere(sphere_C))
    },
    Intersection {
        t: 6.0,
        obj: Box::new(Shape::Sphere(sphere_A))
    }];

    for (index, n1, n2) in test_cases.iter() {
        let hit : &Intersection = &xs[*index];
        let sc = ray.prepare_computations(hit, &xs);
        assert_f64_eq!(sc.n1, n1);
        assert_f64_eq!(sc.n2, n2);
    }
}

#[test]
fn test_under_point() {
    let ray = Ray::new(Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(0.0, 0.0, 1.0));
    let mut sphere = Sphere::new_glass();
    sphere.set_transform(Matrix4x4::translation(0.0, 0.0, 1.0));

    // intersects sphere at z = 0. 
    let i = Intersection {
        t: 5.0,
        obj: Box::new(Shape::Sphere(sphere))
    };
    let xs = vec![i.clone()];
    let sc = ray.prepare_computations(&i, &xs);
    assert!(sc.under_point.z > utils::EPSILON/2.0 && 
            sc.point.z  < sc.under_point.z); 

}

#[test]
fn test_refracted_color_opaque() {
    let mut world : World = Default::default();
    let shape = world.objects[1].clone();
    let ray = Ray::new(Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(0.0, 0.0, 1.0));
    let xs = 
        vec![Intersection { t: 4.0, obj: Box::new(shape.clone()) }, 
             Intersection { t: 6.0, obj: Box::new(shape.clone())}];
    let sc = ray.prepare_computations(&xs[0], &xs);
    let refracted_color = world.refracted_color(&sc, 5);
    assert_eq!(refracted_color, Color::BLACK);
    let refracted_color = world.refracted_color(&sc, 0);
    assert_eq!(refracted_color, Color::BLACK);
}

#[test]
fn test_total_internal_reflection() {
    let mut world : World = Default::default();
    let mut shape = world.objects[1].clone();
    let ray = Ray::new(Vector4D::new_point(0.0, 0.0,  2.0f64.sqrt()/2.0), Vector4D::new_vector(0.0, 1.0, 0.0));
    let mut material = shape.get_material();
    material.transparency = 1.0;
    material.refractive_index = 1.5;
    shape.set_material(&material);
    let xs =
        vec![Intersection { t: -2.0f64.sqrt()/2.0, obj: Box::new(shape.clone()) },
             Intersection { t: 2.0f64.sqrt()/2.0, obj: Box::new(shape.clone()) }];
    let sc = ray.prepare_computations(&xs[1], &xs);
    let c = world.refracted_color(&sc, 5);
    assert_color_eq!(c, Color::BLACK);
}


#[test]
#[ignore="not working"]
fn test_refracted_ray() {
   let mut world : World = Default::default();
   let mut shape1 = world.objects[0].clone();
   let mut shape1_mat  = shape1.get_material();
   shape1_mat.ambient = 1.0;
   shape1_mat.pattern = Some(Box::new(Pattern::TestPattern(TestPattern::new()))); 
   shape1.set_material(&shape1_mat);
   let mut shape2 = world.objects[1].clone();
   let mut shape2_mat  = shape2.get_material();
   shape2_mat.transparency = 1.0;
   shape2_mat.refractive_index = 1.5;
   shape2.set_material(&shape2_mat);
   let ray = Ray::new(Vector4D::new_point(0.0, 0.0, 0.1), Vector4D::new_vector(0.0, 1.0, 0.0));
   let xs =
       vec![Intersection { t: -0.9899, obj: Box::new(shape1.clone()) },
            Intersection { t: -0.4899, obj: Box::new(shape2.clone()) },
            Intersection { t: 0.4899, obj: Box::new(shape2.clone()) },
            Intersection { t: 0.9899, obj: Box::new(shape1.clone()) }];
   let sc = ray.prepare_computations(&xs[2], &xs);
   let color = world.refracted_color(&sc, 5);
   assert_color_eq!(color, Color::new(0.0, 0.99888, 0.04725));
}
#[test]
fn test_transparent_floor() {
    let mut world : World = Default::default();
    let mut floor = Plane::new();
    let mut transparent_floor_mat: Material = Default::default();
    transparent_floor_mat.transparency = 0.5;
    transparent_floor_mat.refractive_index = 1.5;
    floor.set_material(transparent_floor_mat);
    world.objects.push(Shape::Plane(floor.clone()));
    let mut ball = Sphere::new();
    let mut ball_mat : Material = Default::default();
    ball_mat.color = Color::new(1.0, 0.0, 0.0);
    ball_mat.ambient = 0.5;
    ball.set_material(ball_mat);
    ball.set_transform(Matrix4x4::translation(0.0, -3.5, -0.5));
    world.objects.push(Shape::Sphere(ball));
    let ray = Ray::new(Vector4D::new_point(0.0, 0.0, -3.0), Vector4D::new_vector(0.0, -2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0));
    let xs = vec![Intersection { t: 2.0f64.sqrt(), obj: Box::new(Shape::Plane(floor.clone()))}];
    let sc = ray.prepare_computations(&xs[0], &xs);
    let color = shade_hit(&world, &sc, 5);
    assert_color_eq!(color, Color::new(0.93642, 0.6864, 0.68842));
}

#[test]
fn test_render_transparent_floor() {
    const WIDTH_PX: usize = 480;
    const HEIGHT_PX: usize = 320;
    let mut canvas = Canvas::new(WIDTH_PX, HEIGHT_PX);
    let mut world : World = Default::default();
    world.objects.clear();
    let mut floor = Plane::new();
    let mut transparent_floor_mat: Material = Default::default();
    transparent_floor_mat.transparency = 0.7;
    transparent_floor_mat.reflexivity = 0.8;
    transparent_floor_mat.refractive_index = 1.5;
    floor.set_material(transparent_floor_mat);
    floor.set_transform(Matrix4x4::translation(0.0, -2.0, 0.0));
    world.objects.push(Shape::Plane(floor.clone()));
    let mut ball = Sphere::new();
    let mut ball_mat : Material = Default::default();
    ball_mat.color = Color::new(1.0, 0.0, 0.0);
    ball_mat.ambient = 0.5;
    ball_mat.transparency = 0.8;
    ball_mat.refractive_index = 1.3;
    ball.set_material(ball_mat);
    ball.set_transform(Matrix4x4::translation(0.0, 1.0, 0.0));
    world.objects.push(Shape::Sphere(ball));
    let ray = Ray::new(Vector4D::new_point(0.0, 0.0, -3.0), Vector4D::new_vector(0.0, -2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0));
    let xs = vec![Intersection { t: 2.0f64.sqrt(), obj: Box::new(Shape::Plane(floor.clone()))}];
    let sc = ray.prepare_computations(&xs[0], &xs);
    let color = shade_hit(&world, &sc, 5);
    let mut c = Camera::new(WIDTH_PX, HEIGHT_PX, PI/3.0);
    let from = Vector4D::new_point(0.0, 1.5, -5.0);
    let to = Vector4D::new_point(0.0, 1.0, 0.0);
    let up = Vector4D::new_vector(0.0, 1.0, 0.0);
    c.transform = view_transformation(from, to, up); 
    render_with_reflection(&c, &world, &mut canvas);
    canvas.write_ppm("ch11_transparent_floor_sphere.ppm").unwrap();
}

#[test]
fn test_render_checked_sphere_transparent(){
    const WIDTH_PX: usize = 800;
    const HEIGHT_PX: usize = 600;
    let mut canvas = Canvas::new(WIDTH_PX, HEIGHT_PX);
    let mut world: World = Default::default();
    world.objects.clear();

    let mut floor = Plane::new();
    let mut floor_mat : Material = Default::default();
    floor_mat.pattern = Some(Box::new(Pattern::CheckeredPattern(Default::default())));
    floor_mat.transparency = 0.8;
    floor_mat.reflexivity = 0.7;
    floor_mat.refractive_index = 0.4;
    floor.set_transform(Matrix4x4::translation(0.0, -1.0, 0.0));
    floor.set_material(floor_mat);
    world.objects.push(Shape::Plane(floor.clone()));

    let mut left_wall_obj = Sphere::new();
    left_wall_obj.transform = MatrixChainer::new()
        .then(Matrix4x4::scaling(10.0, 0.01, 10.0))
        .then(Matrix4x4::rotate_x(PI/2.0))
        .then(Matrix4x4::rotate_y(-PI/4.0))
        .then(Matrix4x4::translation(0.0, 0.0, 5.0))
        .finish();
    left_wall_obj.material =  Default::default();
    world.objects.push(Shape::Sphere(left_wall_obj));

    let mut right_wall_obj = Sphere::new();
    right_wall_obj.transform = MatrixChainer::new()
        .then(Matrix4x4::scaling(10.0, 0.01, 10.0))
        .then(Matrix4x4::rotate_x(PI/2.0))
        .then(Matrix4x4::rotate_y(PI/4.0))
        .then(Matrix4x4::translation(0.0, 0.0, 5.0))
        .finish();
    right_wall_obj.material =  Default::default();
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

    let mut front_middle = Sphere::new();
    front_middle.transform = MatrixChainer::new()
        .then(Matrix4x4::rotate_x(PI/4.0))
        .then(Matrix4x4::translation(-0.5, 1.0, -2.0))
        .then(Matrix4x4::scaling(0.5, 0.5, 0.5))
        .finish();
    front_middle.material.color = Color::new(0.3, 0.2, 0.5);
    front_middle.material.transparency = 1.0;
    front_middle.material.refractive_index = 1.33;
    front_middle.material.reflexivity = 0.1;
    world.objects.push(Shape::Sphere(front_middle));

    let mut right_sphere = Sphere::new();
    right_sphere.transform = MatrixChainer::new()
        .then(Matrix4x4::scaling(0.6, 0.6, 0.6))
        .then(Matrix4x4::translation(1.5, 0.5, -0.5))
        .finish();

    right_sphere.material.color = Color::new(0.85, 0.3, 0.85);
    right_sphere.material.diffuse = 0.7;
    right_sphere.material.specular = 0.3;
    right_sphere.material.reflexivity = 0.9;
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

    canvas.write_ppm("ch11_checkered_floor.ppm").unwrap();
}

#[test]
fn test_shade_hit_with_shlick() {
    let mut world : World = Default::default();
    let ray = Ray::new(Vector4D::new_point(0.0, 0.0, -3.0), Vector4D::new_vector(0.0, -2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0));
    let mut floor = Plane::new();
    let mut mat : Material = Default::default();
    mat.reflexivity = 0.5;
    mat.transparency = 0.5;
    mat.refractive_index = 1.5;
    floor.set_transform(Matrix4x4::translation(0.0, -1.0, 0.0));
    floor.set_material(mat);
    world.objects.push(Shape::Plane(floor.clone()));
    let mut ball = Sphere::new();
    let mut ball_mat : Material = Default::default();
    ball_mat.color = Color::new(1.0, 0.0, 0.0);
    ball_mat.ambient = 0.5;
    ball.set_transform(Matrix4x4::translation(0.0, -3.5, -0.5));
    ball.set_material(ball_mat);
    world.objects.push(Shape::Sphere(ball));
    let xs = vec![Intersection { t: 2.0f64.sqrt(), obj: Box::new(Shape::Plane(floor.clone()))}];
    let sc = ray.prepare_computations(&xs[0], &xs);
    let color = shade_hit(&world, &sc, 5);
    assert_color_eq!(color, Color::new(0.93391, 0.69643, 0.69243));
}
