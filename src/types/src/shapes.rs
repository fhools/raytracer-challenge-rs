use utils::*;
use crate::Vector4D;
use crate::Matrix4x4;
use crate::Ray;

#[derive(Copy, Clone, Debug)]
pub enum Shape {
    Sphere(Sphere)
}

impl Shape {
    pub fn eq(&self, other: Shape) -> bool {
        match *self {
            Shape::Sphere(ref s) => {
                (s as &dyn Intersectable).eq(other)
            }
        }
    }
}


#[derive(Debug,  Clone)]
pub struct Intersection {
    pub obj: Box<Shape>,
    pub t: f64
}


pub type Intersections = Vec<Intersection>;


pub fn hit(xs: &Intersections) -> Option<Intersection> {
    let mut s = xs.clone();
    s.sort_by(|a, b| a.t.partial_cmp(&b.t).unwrap());
    for i in &s {
        if i.t >= 0.0 {
            return Some(i.clone());
        }
    }
    None
}

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Intersections;
    fn eq(&self, other: Shape) -> bool;
    fn set_transform(&mut self, m: Matrix4x4);
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    pub origin: Vector4D,
    pub radius: f64,
    pub transform: Matrix4x4
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        // Transform the ray via the inverse of the objects transform, same as tranforming unit
        // sphere to in front of the camera.
        let ray = ray.transform(&self.transform.inverse());
        let sphere_to_ray = ray.origin() - self.origin;
        let a = ray.dir().dot(ray.dir());
        let b = 2.0 * ray.dir().dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        println!("a: {}, b: {}, c: {}", a, b, c);
        let discriminant = b*b - 4.0 * a * c;
        println!("discriminant: {}", discriminant);
        if discriminant < 0.0 {
            return vec![];
        }
        let mut intersections : Vec<Intersection> = vec![];
        intersections.push(Intersection {
            obj: Box::new(Shape::Sphere((*self).clone())),
            t: (-b - discriminant.sqrt()) / (2.0 * a) 
        });
        intersections.push(Intersection {
            obj: Box::new(Shape::Sphere((*self).clone())),
            t: (-b + discriminant.sqrt()) / (2.0 * a)
        });
        intersections
    }

    fn eq(&self, other: Shape) -> bool {
        match other { 
            Shape::Sphere(ref sphere) => {
                self.origin.eq(&sphere.origin) &&
                f64_eq(self.radius, sphere.radius) 
            },
            _ => {
                false
            }
        }
    }
    
    fn set_transform(&mut self, m: Matrix4x4) {
        self.transform = m;
    }
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            origin: Vector4D::new_point(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Matrix4x4::new()
        }
    }
}

