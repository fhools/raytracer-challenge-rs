use types::*;
use utils::*;
extern crate raytracer_challenge_rs;

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
