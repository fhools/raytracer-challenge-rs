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
    Plane(Plane),
}

impl Shape {
    pub fn eq(&self, other: &Shape) -> bool {
        match *self {
            Shape::Sphere(ref s) => {
                s.eq(&other)
            },
            Shape::TestShape(ref t) => {
               t.eq(&other) 
            },
            Shape::Plane(ref p) => {
                p.eq(&other)
            }
        }
    }

    pub fn get_material(&self) -> Material {
        match *self {
            Shape::Sphere(ref s) => {
                s.get_material()
            },
            Shape::TestShape(ref t) => {
                t.get_material()
            },
            Shape::Plane(ref p) => {
                p.get_material()
            }
        }
    }

    pub fn set_material(&mut self, material: &Material) {
        match *self {
            Shape::Sphere(ref mut s) => {
                s.set_material(material.clone())
            },
            Shape::TestShape(ref mut t) => {
                t.set_material(material.clone())
            },
            Shape::Plane(ref mut p) => {
                p.set_material(material.clone())
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
    fn normal_at(&self, world_p: Vector4D) -> Vector4D {
        let obj_point = self.get_transform().inverse().mul_vector4d(&world_p);
        let obj_normal = self.normal_at_local(obj_point);

        // The tranpose of the inverse will rescale the normal vectors to the correct proportions.
        // Inverse then transpose of rotation component is a no-op, because transpose is the same
        // as inverting for an orthogonal matrix.
        // Transpose of the pure-scale is a no-op since pure-scale is on the diagnoals of a matrix.
        // So we are left with just the inverting, which does the rescaling of the normal like we
        // want.
        let world_normal = self.get_transform().inverse().transpose().mul_vector4d(&obj_normal);
        let mut n = world_normal.normalized();
        n.w = 0.0;
        n
    }
    fn normal_at_local(&self, obj_point: Vector4D) -> Vector4D;
    fn get_material(&self) -> Material;
    fn set_material(&mut self, material: Material);


    fn saved_ray(&self) -> Option<Ray>;
}

#[derive(Debug, Clone)]
pub struct TestShape {
   pub transform: Matrix4x4,
   pub material: Material,
   saved_ray: Cell<Option<Ray>>
}

impl Intersectable for TestShape {
    fn intersect(&self, ray: &Ray) -> Intersections {
        let ray = ray.transform(&self.get_transform().inverse());
        self.saved_ray.set(Some(ray));
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

    fn normal_at_local(&self, obj_point: Vector4D) -> Vector4D {
        let obj_normal = obj_point - Vector4D::new_point(0.0, 0.0, 0.0);
        obj_normal
    }
    fn get_material(&self) -> Material {
        self.material.clone()
    }
    fn set_material(&mut self, material: Material) {
        self.material = material.clone();
    }

    fn saved_ray(&self) -> Option<Ray> {
        self.saved_ray.get()
    }
}

impl TestShape {
    pub fn new() -> TestShape {
        TestShape {
            material: Default::default(),
            transform: Matrix4x4::new(),
            saved_ray: Cell::new(None)
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


    fn saved_ray(&self) -> Option<Ray> {
        self.saved_ray.get()
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


    fn normal_at_local(&self, obj_point: Vector4D) -> Vector4D {
        let obj_normal = obj_point - Vector4D::new_point(0.0, 0.0, 0.0);
        obj_normal
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

#[derive(Debug, Clone)]
pub struct Plane {
    pub transform: Matrix4x4,
    pub material: Material,
    saved_ray: Cell<Option<Ray>>
}

impl Intersectable for Plane {
    fn intersect(&self, ray: &Ray) -> Intersections {
        let ray = ray.transform(&self.get_transform().inverse());
        self.saved_ray.set(Some(ray));
        if ray.direction.y.abs() < 0.00001 {
            vec![]
        } else {
            let t = -ray.origin.y / ray.direction.y;
            vec![Intersection {
                obj: Box::new(Shape::Plane((*self).clone())),
                t: t }]
        }
    }
    fn eq(&self, other: &Shape) -> bool {
        match other {
            Shape::Plane(ref plane) => {
                self.get_transform().eq(&plane.get_transform())
            },
            _ => { false }
        }
    }

    fn set_transform(&mut self, m: Matrix4x4) {
        self.transform = m
    }
    fn get_transform(&self) -> Matrix4x4 {
        self.transform
    }
    fn normal_at_local(&self, _obj_point: Vector4D) -> Vector4D {
        Vector4D::new_vector(0.0, 1.0, 0.0)
    }
    fn get_material(&self) -> Material {
        self.material.clone()
    }
    fn set_material(&mut self, material: Material) {
        self.material = material.clone();
    }

    fn saved_ray(&self) -> Option<Ray> {
        self.saved_ray.get()
    }
}

impl Plane {
    pub fn new() -> Plane {
        Plane {
            material: Default::default(),
            transform: Matrix4x4::new(),
            saved_ray: Cell::new(None),
        }
    }
}
