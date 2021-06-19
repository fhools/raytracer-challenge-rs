use utils::*;
use types::*;
extern crate raytracer_challenge_rs;
use raytracer_challenge_rs::Canvas;
use std::f64::consts::PI;
#[test]
fn sphere_normal_at() {
    let s = Sphere::new();
    let mut n = s.normal_at(Vector4D::new_point(1.0, 0.0, 0.0));
    assert!(n.eq(&Vector4D::new_vector(1.0, 0.0, 0.0)));
    n = s.normal_at(Vector4D::new_point(0.0, 1.0, 0.0));
    assert!(n.eq(&Vector4D::new_vector(0.0, 1.0, 0.0)));
    n = s.normal_at(Vector4D::new_point(0.0, 0.0, 1.0));
    assert!(n.eq(&Vector4D::new_vector(0.0, 0.0, 1.0)));
}

#[test]
fn sohere_translated_normal_at() {
    let mut s = Sphere::new();
    s.set_transform(Matrix4x4::translation(0.0, 1.0, 0.0));
    let n = s.normal_at(Vector4D::new_point(0.0, 1.70711, -0.70711));
    assert!(n.eq(&Vector4D::new_vector(0.0, 0.7071067811, -0.7071067811)));

}

#[test]
fn sphere_tranformed_normal_at() {
    let mut s = Sphere::new();
    s.set_transform(MatrixChainer::new()
                    .then(Matrix4x4::rotate_z(PI/5.0))
                    .then(Matrix4x4::scaling(1.0, 0.5, 1.0))
                    .finish());
    let n = s.normal_at(Vector4D::new_point(0.0, 2.0f64.sqrt() / 2.0, - 2.0f64.sqrt() / 2.0));
    assert!(n.eq(&Vector4D::new_vector(0.0, 0.970142500, -0.2425356250)));
}

#[test]
fn reflect_45degree() {
    let v = Vector4D::new_vector(1.0, -1.0, 0.0);
    let n = Vector4D::new_vector(0.0, 1.0, 0.0);
    let r = reflect(v, n);
    assert!(r.eq(&Vector4D::new_vector(1.0, 1.0, 0.0)));
}


#[test]
fn reflect_obtuse_anglee() {
    let v = Vector4D::new_vector(0.0, -1.0, 0.0);
    let n = Vector4D::new_vector(2.0f64.sqrt()/2.0, 2.0f64.sqrt()/2.0, 0.0);
    let r = reflect(v, n);
    assert_vector4d_eq!(r, Vector4D::new_vector(1.0, 0.0, 0.0));
}

#[test]
fn lightpoint_new() {
    let l = LightSource::new(Color::new(1.0, 1.0, 1.0),
                            Vector4D::new_point(1.0, 0.0, 0.0));
    assert_vector4d_eq!(l.position, Vector4D::new_point(1.0, 0.0, 0.0));
    assert_f64_eq!(l.intensity.red, 1.0);
    assert_f64_eq!(l.intensity.green, 1.0);
    assert_f64_eq!(l.intensity.blue, 1.0);
}


#[test]
fn test_sphere_has_default_material() {
    let s = Sphere::new();
    let m = s.get_material();
    assert_f64_eq!(m.color.red, 1.0);
    assert_f64_eq!(m.color.green, 1.0);
    assert_f64_eq!(m.color.blue, 1.0);
    assert_f64_eq!(m.ambient, 0.1);
    assert_f64_eq!(m.diffuse, 0.9);
    assert_f64_eq!(m.specular, 0.9);
    assert_f64_eq!(m.shininess, 200.0);
}

#[test]
fn lighting_eye_between_light_and_point() {
    let p = Vector4D::new_point(0.0, 0.0, 0.0);
    let m: Material = Default::default(); 
    let eyev = Vector4D::new_vector(0.0, 0.0, -1.0);
    let normalv = Vector4D::new_vector(0.0, 0.0, -1.0);
    let light = LightSource::new(Color::new(1.0, 1.0, 1.0),Vector4D::new_point(0.0, 0.0, -10.0));
    let result = lighting(m, &Sphere::new(), light, p, eyev, normalv, false);
    println!("light color: {:?}", result);
    assert_eq!(result, Color::new(1.9, 1.9, 1.9));
}

#[test]
fn lighting_eye_between_light_and_point_offset_45degrees() {
    let p = Vector4D::new_point(0.0, 0.0, 0.0);
    let m: Material = Default::default(); 
    let eyev = Vector4D::new_vector(0.0, 2.0f64.sqrt() / 2.0, -2.0f64.sqrt() / 2.0);
    let normalv = Vector4D::new_vector(0.0, 0.0, -1.0);
    let light = LightSource::new(Color::new(1.0, 1.0, 1.0),Vector4D::new_point(0.0, 0.0, -10.0));
    let result = lighting(m, &Sphere::new(), light, p, eyev, normalv, false);
    println!("light color: {:?}", result);
    assert_eq!(result, Color::new(1.0, 1.0, 1.0));
}

#[test]
fn lighting_with_eye_opposite_surface_lightsource_offset_45degrees() {
    let p = Vector4D::new_point(0.0, 0.0, 0.0);
    let m: Material = Default::default(); 
    let eyev = Vector4D::new_vector(0.0, 0.0, -1.0);
    let normalv = Vector4D::new_vector(0.0, 0.0, -1.0);
    let light = LightSource::new(Color::new(1.0, 1.0, 1.0),Vector4D::new_point(0.0, 10.0, -10.0));
    let result = lighting(m, &Sphere::new(), light, p, eyev, normalv, false);
    println!("light color: {:?}", result);
    assert_eq!(result, Color::new(0.736396103, 0.736396103, 0.736396103));
}

#[test]
fn lighting_with_eye_in_path_of_reflection_vector() {
    let p = Vector4D::new_point(0.0, 0.0, 0.0);
    let m: Material = Default::default(); 
    let eyev = Vector4D::new_vector(0.0, -2.0f64.sqrt()/2.0, -2.0f64.sqrt()/2.0);
    let normalv = Vector4D::new_vector(0.0, 0.0, -1.0);
    let light = LightSource::new(Color::new(1.0, 1.0, 1.0),Vector4D::new_point(0.0, 10.0, -10.0));
    let result = lighting(m, &Sphere::new(), light, p, eyev, normalv, false);
    println!("light color: {:?}", result);
    assert_eq!(result, Color::new(1.636396103, 1.636396103, 1.636396103));
}

#[test]
fn lighting_behind_the_surface() {
    let p = Vector4D::new_point(0.0, 0.0, 0.0);
    let m: Material = Default::default(); 
    let eyev = Vector4D::new_vector(0.0, 0.0, -1.0);
    let normalv = Vector4D::new_vector(0.0, 0.0, -1.0);
    let light = LightSource::new(Color::new(1.0, 1.0, 1.0),Vector4D::new_point(0.0, 0.0, 10.0));
    let result = lighting(m, &Sphere::new(), light, p, eyev, normalv, false);
    println!("light color: {:?}", result);
    assert_eq!(result, Color::new(0.1, 0.1, 0.1));
}

#[test]
#[ignore="slow test"]
fn test_raytrace_sphere_lighting() {
    let ray_origin = Vector4D::new_point(0.0, 0.0, -5.0);
    let wall_z = 10.0;
    let wall_size = 7.0;
    let canvas_width_pixels = 400.0;
    let pixel_size = wall_size / canvas_width_pixels;
    let half = wall_size / 2.0;
    let mut canvas = Canvas::new(canvas_width_pixels as usize, canvas_width_pixels as usize);
    let mut shape = Sphere::new();
    shape.material.color = Color::new(1.0, 0.2, 1.0);
    shape.set_transform(Matrix4x4::scaling(1.0, 0.5, 1.0));
    let lightsrc = LightSource::new(Color::new(1.0, 1.0, 1.0), Vector4D::new_point(-10.0, 10.0, -10.0));

    for y in 0..(canvas_width_pixels as usize  - 1) {
        let world_y = half - pixel_size * (y as f64);
        for x in 0..(canvas_width_pixels as usize - 1) {
            let world_x = -half + pixel_size * (x as f64);
            let pos = Vector4D::new_point(world_x, world_y, wall_z);
            let ray = Ray::new(ray_origin, (pos - ray_origin).normalized());
            let xs = ray.intersect(&shape);
            match hit(&xs) {
                Some(ht) => {
                    match *ht.obj {
                        Shape::Sphere(obj) => {
                            let hit_point = ray.at_t(ht.t);
                            let normal = obj.normal_at(hit_point);
                            let eye = -ray.direction;
                            let color = lighting(obj.get_material(), &obj, lightsrc, hit_point, eye, normal, false);
                            canvas.set_pixel(x, y, &color);
                        },
                        Shape::TestShape(_) => {
                        },
                        Shape::Plane(obj) => {
                            let hit_point = ray.at_t(ht.t);
                            let normal = obj.normal_at(hit_point);
                            let eye = -ray.direction;
                            let color = lighting(obj.get_material(), &obj, lightsrc, hit_point, eye, normal, false);
                            canvas.set_pixel(x, y, &color);
                        }
                    }
                },
                None => {}
            }
        }
    }

    canvas.write_ppm("ch6.ppm").unwrap();
}

