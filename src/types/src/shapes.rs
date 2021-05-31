use utils::*;
use crate::Vector4D;
use crate::Ray;

#[derive(Copy, Clone, Debug)]
pub enum Shape {
    Sphere(Sphere)
}

pub struct Intersection {
    pub obj: Box<Shape>,
    pub t: f64
}


type Intersections = Vec<Intersection>;

pub trait Intersectable {
    fn intersect(&self, ray: &Ray) -> Intersections;
    fn eq(&self, other: Shape) -> bool;
}

#[derive(Copy, Clone, Debug)]
pub struct Sphere {
    origin: Vector4D,
    radius: f64
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
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
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            origin: Vector4D::new_point(0.0, 0.0, 0.0),
            radius: 1.0
        }
    }
}

