use utils::*;
pub struct Vector4D {
    pub x: f64,
    pub y: f64,
    pub z: f64,
    pub w: f64,
}

impl Vector4D {
    pub fn new(x: f64, y: f64, z: f64, w: f64) -> Vector4D {
        Vector4D {
            x, y, z, w
        }
    }
    pub fn is_point(&self) -> bool {
       f64_eq(self.w, 1.0) 
    }
    
    pub fn is_vector(&self) -> bool {
        f64_eq(self.w, 0.0)
    }
}
