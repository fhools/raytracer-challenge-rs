use std::f64::consts::PI;
use types::*;
use utils::*;
extern crate raytracer_challenge_rs;
use raytracer_challenge_rs::Canvas;
use raytracer_challenge_rs::render;
#[test]
fn test_default_world() {
    let world : World = Default::default();
}

#[test]
fn ray_intersect_world() {
    let world : World = Default::default();
    let ray = Ray::new(Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(0.0,0.0,1.0));
    let xs = ray.intersect_world(&world);
    assert_eq!(xs.len() , 4);
    assert_f64_eq!(xs[0].t, 4.0);
    assert_f64_eq!(xs[1].t, 4.5);
    assert_f64_eq!(xs[2].t, 5.5);
    assert_f64_eq!(xs[3].t, 6.0);
}

#[test]
fn test_prepare_computations() {
    let r = Ray::new(Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(0.0, 0.0, 1.0));
    let shape = Sphere::new();
    let i = Intersection {
                obj: Box::new(Shape::Sphere(shape)),
                t: 4.0
    };

    let c = r.prepare_computations(&i);
    assert_f64_eq!(c.t, i.t);
    assert!(i.obj.eq(*c.obj));
    assert_vector4d_eq!(c.eyev, Vector4D::new_vector(0.0, 0.0, -1.0));
    assert_vector4d_eq!(c.point, Vector4D::new_point(0.0, 0.0, -1.0));
    assert_vector4d_eq!(c.normalv, Vector4D::new_vector(0.0, 0.0, -1.0));
}

#[test]
fn test_prepare_computations_inside_hit_false() {
    let r = Ray::new(Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(0.0, 0.0, 1.0));
    let shape = Sphere::new();
    let i = Intersection {
                obj: Box::new(Shape::Sphere(shape)),
                t: 4.0
    };

    let c = r.prepare_computations(&i);
    assert_f64_eq!(c.t, i.t);
    assert!(i.obj.eq(*c.obj));
    assert_vector4d_eq!(c.eyev, Vector4D::new_vector(0.0, 0.0, -1.0));
    assert_vector4d_eq!(c.point, Vector4D::new_point(0.0, 0.0, -1.0));
    assert_vector4d_eq!(c.normalv, Vector4D::new_vector(0.0, 0.0, -1.0));
    assert_eq!(c.inside, false);
}

#[test]
fn test_prepare_computations_inside_hit_true() {
    let r = Ray::new(Vector4D::new_point(0.0, 0.0, 0.0), Vector4D::new_vector(0.0, 0.0, 1.0));
    let shape = Sphere::new();
    let i = Intersection {
                obj: Box::new(Shape::Sphere(shape)),
                t: 1.0
    };

    let c = r.prepare_computations(&i);
    assert_f64_eq!(c.t, i.t);
    assert!(i.obj.eq(*c.obj));
    assert_vector4d_eq!(c.eyev, Vector4D::new_vector(0.0, 0.0, -1.0));
    assert_vector4d_eq!(c.point, Vector4D::new_point(0.0, 0.0, 1.0));
    assert_vector4d_eq!(c.normalv, Vector4D::new_vector(0.0, 0.0, -1.0));
    assert_eq!(c.inside, true);
}

#[test]
fn shade_intersection() {
    let w : World = Default::default();
    let shape = w.objects[0]; 
    let r = Ray::new(Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(0.0, 0.0, 1.0));
    let i = Intersection {
                obj: Box::new(shape),
                t: 4.0
    };

    let c = r.prepare_computations(&i);
    let color = shade_hit(&w, &c);
    assert_f64_eq!(color.red, 0.380661193);
    assert_f64_eq!(color.green, 0.475826491);
    assert_f64_eq!(color.blue, 0.2854958948);
}

#[test]
fn shade_intersection_inside() {
    let mut w : World = Default::default();
    w.light_source = LightSource::new(Color::new(1.0, 1.0, 1.0), Vector4D::new_point(0.0, 0.25, 0.0));
    let shape = w.objects[1]; 
    let r = Ray::new(Vector4D::new_point(0.0, 0.0, 0.0), Vector4D::new_vector(0.0, 0.0, 1.0));
    let i = Intersection {
                obj: Box::new(shape),
                t: 0.5
    };

    let c = r.prepare_computations(&i);
    let color = shade_hit(&w, &c);
    assert_f64_eq!(color.red, 0.90498447208);
    assert_f64_eq!(color.green, 0.90498447208);
    assert_f64_eq!(color.blue, 0.90498447208);
}

#[test]
fn color_at_hit() {
    let world: World = Default::default();
    let r = Ray::new(Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(0.0, 0.0, 1.0));
    let color = color_at(&world, r);
    assert_f64_eq!(color.red, 0.380661193);
    assert_f64_eq!(color.green, 0.475826491);
    assert_f64_eq!(color.blue, 0.2854958948);
}

#[test]
fn color_at_nohits() {
    let world: World = Default::default();
    let r = Ray::new(Vector4D::new_point(0.0, 0.0, -5.0), Vector4D::new_vector(0.0, 1.0, 0.0));
    let color = color_at(&world, r);
    assert_f64_eq!(color.red, 0.0);
    assert_f64_eq!(color.green, 0.0);
    assert_f64_eq!(color.blue, 0.0);
}

#[test]
fn color_at_behind_ray() {
    let mut world: World = Default::default();
    let Shape::Sphere(ref mut s1) = world.objects[0];
    s1.material.ambient = 1.0;
    let Shape::Sphere(ref mut s2) = world.objects[1];
    s2.material.ambient = 1.0;
    let innercolor = s2.material.color;
    let r = Ray::new(Vector4D::new_point(0.0, 0.0, 0.75), Vector4D::new_vector(0.0, 0.0, -1.0));
    let color = color_at(&world, r);
    println!("### color: {:?} inner color: {:?}", color, innercolor);
    assert_f64_eq!(color.red, innercolor.red);
    assert_f64_eq!(color.green, innercolor.green);
    assert_f64_eq!(color.blue, innercolor.blue);
}

#[test]
fn view_transformation_default() {
    let from = Vector4D::new_point(0.0, 0.0, 0.0);
    let to = Vector4D::new_point(0.0, 0.0, -1.0);
    let up= Vector4D::new_point(0.0, 1.0, 0.0);
    let vt = view_transformation(from, to, up);
    assert_f64_eq!(vt.m[0][0], 1.0);
    assert_f64_eq!(vt.m[1][1], 1.0);
    assert_f64_eq!(vt.m[2][2], 1.0);
}

#[test]
fn view_transformation_look_z() {
    let from = Vector4D::new_point(0.0, 0.0, 0.0);
    let to = Vector4D::new_point(0.0, 0.0, 1.0);
    let up= Vector4D::new_point(0.0, 1.0, 0.0);
    let vt = view_transformation(from, to, up);
    let scaling = Matrix4x4::scaling(-1.0, 1.0, -1.0);
    assert!(vt.eq(&scaling));
}

#[test]
fn view_transformation_moves_world() {
    let from = Vector4D::new_point(0.0, 0.0, 8.0);
    let to = Vector4D::new_point(0.0, 0.0, 1.0);
    let up= Vector4D::new_point(0.0, 1.0, 0.0);
    let vt = view_transformation(from, to, up);
    let translation = Matrix4x4::translation(0.0, 0.0, -8.0);
    assert!(vt.eq(&translation));
}

#[test]
fn view_transformation_arbitrary_transform() {
    let from = Vector4D::new_point(1.0, 3.0, 2.0);
    let to = Vector4D::new_point(4.0, -2.0, 8.0);
    let up= Vector4D::new_point(1.0, 1.0, 0.0);
    let vt = view_transformation(from, to, up);
    let mut mt = Matrix4x4::new();
    mt.m[0][0] = -0.50709;
    mt.m[0][1] = 0.50709; 
    mt.m[0][2] = 0.67612; 
    mt.m[0][3] = -2.36643; 
    mt.m[1][0] = 0.76772; 
    mt.m[1][1] = 0.60609; 
    mt.m[1][2] = 0.12122; 
    mt.m[1][3] = -2.82843; 
    mt.m[2][0] = -0.35857; 
    mt.m[2][1] = 0.59761;
    mt.m[2][2] = -0.71714; 
    mt.m[2][3] = 0.0;
    mt.m[3][0] = 0.0;
    mt.m[3][1] = 0.0;
    mt.m[3][2] = 0.0;
    mt.m[3][3] =  1.0;
    assert!(vt.eq(&mt));
}

#[test]
fn camera_new() {
    let hsize = 160;
    let vsize = 120;
    let field_of_view = PI/2.0;
    let c = Camera::new(hsize, vsize, field_of_view);
}
#[test]
fn camera_aspect_hsize_gt_vsize() {
    let hsize = 200;
    let vsize = 125;
    let field_of_view = PI/2.0;
    let c = Camera::new(hsize, vsize, field_of_view);
    assert_f64_eq!(c.pixel_size, 0.01);
}

#[test]
fn camera_aspect_vsize_gt_hsize() {
    let hsize = 125;
    let vsize = 200;
    let field_of_view = PI/2.0;
    let c = Camera::new(hsize, vsize, field_of_view);
    assert_f64_eq!(c.pixel_size, 0.01);
}

#[test]
fn ray_for_pixel__center_canvas() {
    let c = Camera::new(201, 101, PI/2.0);
    let r = ray_for_pixel(&c, 100, 50);
    assert_vector4d_eq!(r.origin, Vector4D::new_point(0.0, 0.0, 0.0));
    assert_vector4d_eq!(r.dir(), Vector4D::new_vector(0.0, 0.0, -1.0));
}

#[test]
fn ray_for_pixel_top_corner_canvas() {
    let c = Camera::new(201, 101, PI/2.0);
    let r = ray_for_pixel(&c, 0, 0);
    assert_vector4d_eq!(r.origin, Vector4D::new_point(0.0, 0.0, 0.0));
    assert_vector4d_eq!(r.dir(), Vector4D::new_vector(0.66519, 0.33259, -0.66851));
}

#[test]
fn ray_for_pixel_camera_transformed() {
    let mut c = Camera::new(201, 101, PI/2.0);
    c.transform = MatrixChainer::new()
        .then(Matrix4x4::translation(0.0, -2.0, 5.0))
        .then(Matrix4x4::rotate_y(PI/4.0))
        .finish();
    let r = ray_for_pixel(&c, 100, 50);
    assert_vector4d_eq!(r.origin, Vector4D::new_point(0.0, 2.0, -5.0));
    assert_vector4d_eq!(r.dir(), Vector4D::new_vector(2.0f64.sqrt()/2.0, 0.0, -2.0f64.sqrt()/2.0));
}

#[test]
#[ignore="render image"]
fn test_raytrace_with_camera() {
    let mut canvas = Canvas::new(100, 80);
    let world: World = Default::default();
    let mut c = Camera::new(100, 80, PI/2.0);
    let from = Vector4D::new_point(0.0, 0.0, -5.0);
    let to = Vector4D::new_point(0.0, 0.0, 0.0);
    let up = Vector4D::new_vector(0.0, 1.0, 0.0);
    c.transform = view_transformation(from, to, up); 
    fn render(camera: &Camera, world: &World, canvas: &mut Canvas) {
        for y in 0..(camera.vsize_px - 1) {
            for x in 0..(camera.hsize_px - 1) {
                let ray = ray_for_pixel(&camera, x, y); 
                let color = color_at(&world, ray);
                canvas.set_pixel(x, y, &color);
            }
        }
        canvas.write_ppm("ch7.ppm").unwrap();
    }
    render(&c, &world, &mut canvas);
}

#[test]
//#[ignore="render"]
fn test_raytrace_with_camera_multiple_spheres() {
    const WIDTH_PX: usize = 100;
    const HEIGHT_PX: usize = 80;;
    let mut canvas = Canvas::new(WIDTH_PX, HEIGHT_PX);
    let mut world: World = Default::default();
    world.objects.clear();

    let mut floor_obj = Sphere::new();
    floor_obj.transform = Matrix4x4::scaling(10.0, -0.01, 10.0);
    floor_obj.material.color = Color::new(1.0, 0.9, 0.9);
    floor_obj.material.specular = 0.0; 

    world.objects.push(Shape::Sphere(floor_obj));

    let mut left_wall_obj = Sphere::new();
    left_wall_obj.transform = MatrixChainer::new()
        .then(Matrix4x4::scaling(10.0, 0.01, 10.0))
        .then(Matrix4x4::rotate_x(PI/2.0))
        .then(Matrix4x4::rotate_y(-PI/4.0))
        .then(Matrix4x4::translation(0.0, 0.0, 5.0))
        .finish();
    left_wall_obj.material = floor_obj.material;
    world.objects.push(Shape::Sphere(left_wall_obj));

    let mut right_wall_obj = Sphere::new();
    right_wall_obj.transform = MatrixChainer::new()
        .then(Matrix4x4::scaling(10.0, 0.01, 10.0))
        .then(Matrix4x4::rotate_x(PI/2.0))
        .then(Matrix4x4::rotate_y(PI/4.0))
        .then(Matrix4x4::translation(0.0, 0.0, 5.0))
        .finish();
    right_wall_obj.material = floor_obj.material;
    world.objects.push(Shape::Sphere(right_wall_obj));
    

    let mut middle = Sphere::new();
    middle.transform = MatrixChainer::new()
        .then(Matrix4x4::translation(-0.5, 1.0, 0.5))
        .finish();
    middle.material.color = Color::new(0.1, 1.0, 0.5);
    middle.material.diffuse = 0.7;
    middle.material.specular = 0.3;
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
}
