use utils::*;
use types::*;
use std::f64::consts::PI;
extern crate raytracer_challenge_rs;

#[allow(unused_imports)]
use raytracer_challenge_rs::{Canvas, render};


#[test]
fn intersect_sphere_saved_ray() {
    let r = Ray::new(Vector4D::new_point(0.0, 0.0, -5.0),
                     Vector4D::new_vector(0.0, 0.0, 1.0));
    let mut o = Sphere::new();
    o.set_transform(Matrix4x4::scaling(2.0, 2.0, 2.0));
    let _xs = o.intersect(&r);
    
    assert_vector4d_eq!(o.saved_ray().unwrap().origin, Vector4D::new_point(0.0, 0.0, -2.5));
    assert_vector4d_eq!(o.saved_ray().unwrap().direction, Vector4D::new_vector(0.0, 0.0, 0.5));
}

#[test]
fn intersect_plane() {
    let r = Ray::new(Vector4D::new_point(0.0, 1.0, -1.0),
                     Vector4D::new_vector(0.0, -2.0f64.sqrt()/2.0, -2.0f64.sqrt()/2.0));
    let plane = Plane::new();
    let _xs = plane.intersect(&r);
}

#[test]
//#[ignore="rendering"]
fn test_raytrace_with_camera() {
    const WIDTH: usize = 200;
    const HEIGHT: usize = 180;
    let mut canvas = Canvas::new(WIDTH, HEIGHT);
    let mut world: World = Default::default();
    let mut plane = Plane::new();
    let mut mat_p = Material::new(Color::new(0.5, 0.3, 0.0));
    mat_p.diffuse = 0.7;
    mat_p.specular = 0.1;
    plane.set_material(mat_p);
    plane.set_transform(Matrix4x4::translation(0.0, -2.0, 0.0));
    world.objects.push(Shape::Plane(plane));
    let mut c = Camera::new(WIDTH, HEIGHT, PI/2.0);
    let from = Vector4D::new_point(0.0, 2.0, -5.0);
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
        canvas.write_ppm("ch9.ppm").unwrap();
    }
    render(&c, &world, &mut canvas);
}

