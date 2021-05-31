use utils::*;
use crate::Vector4D;
use crate::Intersectable;
use crate::Intersection;
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
        self.origin + t * self.direction
    }


    pub fn intersect<S: Intersectable>(&self, shape: &S) -> Vec<Intersection> {
        shape.intersect(self)
    }
}



