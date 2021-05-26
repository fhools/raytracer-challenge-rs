use utils::*;

#[derive(Debug)]
pub struct Matrix4x4 {
    pub m: [[f64;4]; 4]
}

impl Matrix4x4 {
    pub fn new() -> Matrix4x4 {
        let mut m = Matrix4x4 {
            m: [[0.0; 4]; 4]
        };
        m.m[0][0] = 1.0;
        m.m[1][1] = 1.0;
        m.m[2][2] = 1.0;
        m.m[3][3] = 1.0;
        m
    }


    pub fn from_vectors(r1: &[f64], r2: &[f64], r3: &[f64], r4: &[f64]) -> Matrix4x4 {
        let mut m = Matrix4x4 {
            m: [[0.0; 4]; 4]
        };

        for i in 0..4 {
            m.m[0][i] = r1[i];
        }
        for i in 0..4 {
            m.m[1][i] = r2[i];
        }
        for i in 0..4 {
            m.m[2][i] = r3[i];
        }
        for i in 0..4 {
            m.m[3][i] = r4[i];
        }

        m
    }

    pub fn eq(&self, other: &Matrix4x4) -> bool {
        for i in 0..4 {
            for j in 0..4 {
                if !f64_eq(self.m[i][j], other.m[i][j]) {
                    return false;
                }
            }
        }
        return true;
    }
    
    pub fn mul(&self, other: &Matrix4x4) -> Matrix4x4 {
        let mut prod = Matrix4x4::new();
        for r in 0..4 {
            for c in 0..4 {
                let dot = self.m[r][0] * other.m[0][c] +
                    self.m[r][1] * other.m[1][c] +
                    self.m[r][2] * other.m[2][c] +
                    self.m[r][3] * other.m[3][c];
                prod.m[r][c] = dot;
            }
        }
        prod
    }
}

#[derive(Debug)]
pub struct Matrix3x3 {
    pub m: [[f64; 3]; 3]
}

impl Matrix3x3 {
    pub fn new() -> Matrix3x3 {
        let mut m = Matrix3x3 {
            m: [[0.0; 3]; 3]
        };
        m.m[0][0] = 1.0;
        m.m[1][1] = 1.0;
        m.m[2][2] = 1.0;
        m
    }

    pub fn from_vectors(r1: &[f64], r2: &[f64], r3: &[f64]) -> Matrix3x3 {
        let mut m = Matrix3x3 {
            m: [[0.0; 3]; 3]
        };

        for i in 0..3 {
            m.m[0][i] = r1[i];
        }
        for i in 0..3 {
            m.m[1][i] = r2[i];
        }
        for i in 0..3 {
            m.m[2][i] = r3[i];
        }
        m
    }

    pub fn eq(&self, other: &Matrix3x3) -> bool {
        for i in 0..3 {
            for j in 0..3 {
                if !f64_eq(self.m[i][j], other.m[i][j]) {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn mul(&self, other: &Matrix3x3) -> Matrix3x3 {
        let mut prod = Matrix3x3::new();
        for r in 0..3 {
            for c in 0..3 {
                let dot = self.m[r][0] * other.m[0][c] +
                    self.m[r][1] * other.m[1][c] +
                    self.m[r][2] * other.m[2][c];
                prod.m[r][c] = dot;
            }
        }
        prod
    }
}


#[derive(Debug)]
pub struct Matrix2x2 {
    pub m: [[f64;2]; 2]
}

impl Matrix2x2 {
    pub fn new() -> Matrix2x2 {
        let mut m = Matrix2x2 {
            m: [[0.0; 2]; 2]
        };
        m.m[0][0] = 1.0;
        m.m[1][1] = 1.0;
        m
    }

    pub fn from_vectors(r1: &[f64], r2: &[f64]) -> Matrix2x2 {
        let mut m = Matrix2x2 {
            m: [[0.0; 2]; 2]
        };

        for i in 0..2 {
            m.m[0][i] = r1[i];
        }
        for i in 0..2 {
            m.m[1][i] = r2[i];
        }
        m
    }

    pub fn eq(&self, other: &Matrix2x2) -> bool {
        for i in 0..2 {
            for j in 0..2 {
                if !f64_eq(self.m[i][j], other.m[i][j]) {
                    return false;
                }
            }
        }
        return true;
    }

    pub fn mul(&self, other: &Matrix2x2) -> Matrix2x2 {
        let mut prod = Matrix2x2::new();
        for r in 0..2 {
            for c in 0..2 {
                let dot = self.m[r][0] * other.m[0][c] +
                    self.m[r][1] * other.m[1][c];
                prod.m[r][c] = dot;
            }
        }
        prod
    }
}
