use std::ops::{Add, Sub, Mul, Neg, Div};
use utils::*;

#[derive(Debug, Copy, Clone)]
pub struct Vector4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector4D {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vector4D {
        assert!(f64_eq(w, 0.0) || f64_eq(w, 1.0));
        Vector4D {
            x, y, z, w
        }
    }

    pub fn new_vector(x: f64, y: f64, z: f64) -> Vector4D {
        Vector4D {
            x, y, z, w: 0.0
        }
    }

    pub fn new_point(x: f64, y: f64, z: f64) -> Vector4D {
        Vector4D {
            x, y, z, w: 1.0
        }
    }

    pub fn is_point(&self) -> bool {
       f64_eq(self.w, 1.0) 
    }
    
    pub fn is_vector(&self) -> bool {
        f64_eq(self.w, 0.0)
    }

    pub fn norm(&self) -> f64 {
        let norm_sqr = self.x * self.x + self.y * self.y + self.z * self.z;
        norm_sqr.sqrt()
    }

    pub fn eq(self, r: &Vector4D) -> bool {
        f64_eq(self.x, r.x) && f64_eq(self.y, r.y) && f64_eq(self.z, r.z) && f64_eq(self.w, r.w)
    }

    pub fn normalize(&mut self) {
        let magnitude= self.norm();
        self.x = self.x / magnitude; 
        self.y = self.y / magnitude; 
        self.z = self.z / magnitude; 
    }

    pub fn normalized(&self) -> Vector4D {
        let magnitude= self.norm();
        let mut n = *self;

        n.x = self.x / magnitude; 
        n.y = self.y / magnitude; 
        n.z = self.z / magnitude; 
        n
    }
    pub fn dot(&self, other: Vector4D) -> f64 {
        self.x * other.x + self.y * other.y + self.z * other.z + self.w * other.w
    }

    pub fn cross(&self, other: Vector4D) -> Vector4D {
        Vector4D {
            x: self.y * other.z - self.z * other.y,
            y: self.z * other.x - self.x * other.z,
            z: self.x * other.y - self.y * other.x,
            w: 0.0
        }
    }
}

// TODO: All vector arithmetic should have w = 0, should we force that, or rely on invariant that
// the user won't call operations on points?
/// scalar * vector
impl Mul<Vector4D> for f64 {
    type Output = Vector4D;
    fn mul(self, v: Vector4D) -> Vector4D {
        Vector4D {
            x: self * v.x,
            y: self * v.y,
            z: self * v.z,
            w: self * v.w, 
        }
    }
}

/// vector / scalar
impl Div<f64> for Vector4D {
    type Output = Vector4D;
    fn div(self, divisor: f64) -> Vector4D {
        Vector4D {
            x: self.x / divisor,
            y: self.y / divisor,
            z: self.z / divisor,
            w: self.w
        }
    }
}
/// vector addition
impl Add<Vector4D> for Vector4D {
    type Output = Vector4D;
    fn add(self, r: Vector4D) -> Vector4D {
        Vector4D {
            x: self.x + r.x,
            y: self.y + r.y,
            z: self.z + r.z,
            w: 0.0
        }
    }
}

/// vector subtraction 
impl Sub<Vector4D> for Vector4D {
    type Output = Vector4D;
    fn sub(self, r: Vector4D) -> Vector4D {
        Vector4D {
            x: self.x - r.x,
            y: self.y - r.y,
            z: self.z - r.z,
            w: 0.0 
        }
    }
}


/// vector negation 
impl Neg for Vector4D {
    type Output = Vector4D;
    fn neg(self) -> Vector4D {
        Vector4D {
            x: - self.x,
            y: - self.y,
            z: - self.z,
            w:  0.0
        }
    }
}

pub fn reflect(input: Vector4D, normal: Vector4D) -> Vector4D {
    input - 2.0 * input.dot(normal) * normal
}
