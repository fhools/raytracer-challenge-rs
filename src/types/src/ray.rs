use crate::Vector4D;
use crate::Matrix4x4;
use crate::Intersectable;
use crate::Intersection;
use crate::Intersections;
use crate::World;
use crate::Shape;
#[derive(Debug, Copy, Clone)]
pub struct Ray {
    pub origin: Vector4D,
    pub direction: Vector4D
}

impl Ray {
    pub fn new(origin: Vector4D, direction: Vector4D) -> Ray {
        Ray {
            origin,
            direction,
        }
    }

    pub fn origin(&self) -> Vector4D {
        self.origin
    }

    pub fn dir(&self) -> Vector4D {
        self.direction
    }

    pub fn at_t(&self, t: f64) -> Vector4D {
        let mut p =  self.origin + t * self.direction;
        p
    }


    pub fn intersect<S: Intersectable>(&self, shape: &S) -> Vec<Intersection> {
        shape.intersect(self)
    }

    pub fn intersect_world(&self, world: &World) -> Intersections {
        world.intersect(self)
    }

    pub fn prepare_computations(&self, intersection: &Intersection) -> Computation {
        let p = self.at_t(intersection.t);
        let eyev = -self.dir();
        let normalv;
        let obj;
        match *intersection.obj {
            Shape::Sphere(sph) => {
                normalv = sph.normal_at(p);
                obj = Shape::Sphere(sph);
            }
        }
        Computation {
            t: intersection.t,
            obj: Box::new(obj),
            point: p,
            eyev: eyev,
            normalv: normalv
        }
    }

    pub fn transform(&self, m: &Matrix4x4) -> Ray {
        Ray {
            origin: m.mul_vector4d(&self.origin),
            direction: m.mul_vector4d(&self.direction)
        }
    }
}

pub struct Computation {
    pub t: f64,
    pub obj: Box<Shape>,
    pub point: Vector4D,
    pub eyev: Vector4D,
    pub normalv: Vector4D
}

