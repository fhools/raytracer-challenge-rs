use utils::*;
use crate::Vector4D;
use crate::Matrix4x4;
use crate::Ray;
use crate::Material;
use std::cell::Cell;

#[derive(Clone, Debug)]
pub enum Shape {
    Sphere(Sphere),
    TestShape(TestShape),
}

impl Shape {
    pub fn eq(&self, other: &Shape) -> bool {
        match *self {
            Shape::Sphere(ref s) => {
                s.eq(&other)
            },
            Shape::TestShape(t) => {
               t.eq(&other) 
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
    fn eq(&self, other: &Shape) -> bool;
    fn set_transform(&mut self, m: Matrix4x4);
    fn get_transform(&self) -> Matrix4x4;
    fn normal_at(&self, p: Vector4D) -> Vector4D;
    fn get_material(&self) -> Material;
    fn set_material(&mut self, material: Material);
}

#[derive(Debug, Clone, Copy)]
pub struct TestShape {
   pub transform: Matrix4x4,
   pub material: Material,
}

impl Intersectable for TestShape {
    fn intersect(&self, ray: &Ray) -> Intersections {
        vec![]
    }
    fn eq(&self, other: &Shape) -> bool {
        match *other {
            Shape::TestShape(ref _t) => { true },
            _ => { false }
        }
    }

    fn set_transform(&mut self, m: Matrix4x4) {
        self.transform = m;
    }

    fn get_transform(&self) -> Matrix4x4 {
        self.transform
    }

    fn normal_at(&self, p: Vector4D) -> Vector4D {
        let obj_point = self.get_transform().inverse().mul_vector4d(&p);
        let obj_normal = obj_point - Vector4D::new_point(0.0, 0.0, 0.0);
        let world_normal = self.get_transform().inverse().transpose().mul_vector4d(&obj_normal);
        let mut n = world_normal.normalized();
        n.w = 0.0;
        n
    }
    fn get_material(&self) -> Material {
        self.material
    }
    fn set_material(&mut self, material: Material) {
        self.material = material;
    }
}

impl TestShape {
    pub fn new() -> TestShape {
        TestShape {
            material: Default::default(),
            transform: Matrix4x4::new()
        }
    }
}


#[derive(Clone, Debug)]
pub struct Sphere {
    pub origin: Vector4D,
    pub radius: f64,
    pub transform: Matrix4x4,
    pub material: Material,
    saved_ray: Cell<Option<Ray>>
}

impl Intersectable for Sphere {
    fn intersect(&self, ray: &Ray) -> Vec<Intersection> {
        // Transform the ray via the inverse of the objects transform, same as tranforming unit
        // sphere to in front of the camera.
        let ray = ray.transform(&self.get_transform().inverse());
        self.saved_ray.set(Some(ray));
        let sphere_to_ray = ray.origin() - self.origin;
        let a = ray.dir().dot(ray.dir());
        let b = 2.0 * ray.dir().dot(sphere_to_ray);
        let c = sphere_to_ray.dot(sphere_to_ray) - 1.0;

        let discriminant = b*b - 4.0 * a * c;
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

    fn eq(&self, other: &Shape) -> bool {
        match other { 
            Shape::Sphere(ref sphere) => {
                self.origin.eq(&sphere.origin) &&
                f64_eq(self.radius, sphere.radius) 
            },
            _ => { false }
        }
    }
    
    fn set_transform(&mut self, m: Matrix4x4) {
        self.transform = m;
    }

    fn get_transform(&self) -> Matrix4x4 {
        self.transform
    }

    fn normal_at(&self, world_p: Vector4D) -> Vector4D {
        let obj_point = self.transform.inverse().mul_vector4d(&world_p);
        let obj_normal = obj_point - Vector4D::new_point(0.0, 0.0, 0.0);
        let world_normal = self.transform.inverse().transpose().mul_vector4d(&obj_normal);
        let mut n = world_normal.normalized();
        n.w = 0.0;
        n
    }
   
    fn get_material(&self) -> Material {
        self.material.clone()
    }

    fn set_material(&mut self, mat: Material) {
        self.material = mat;
    }
}

impl Sphere {
    pub fn new() -> Sphere {
        Sphere {
            origin: Vector4D::new_point(0.0, 0.0, 0.0),
            radius: 1.0,
            transform: Matrix4x4::new(),
            material: Default::default(),
            saved_ray: Cell::new(None)
        }
    }
}

