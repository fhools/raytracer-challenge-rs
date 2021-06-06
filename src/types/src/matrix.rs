use utils::*;
use super::Vector4D;
#[derive(Debug,Clone, Copy)]
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

    pub fn from_vector(v: &[f64]) -> Matrix4x4 {
        let mut m = Matrix4x4 {
            m: [[0.0; 4]; 4]
        };

        m.m[0][0] = v[0];
        m.m[0][1] = v[1];
        m.m[0][2] = v[2];
        m.m[0][3] = v[3];
        m.m[1][0] = v[4];
        m.m[1][1] = v[5];
        m.m[1][2] = v[6];
        m.m[1][3] = v[7];
        m.m[2][0] = v[8];
        m.m[2][1] = v[9];
        m.m[2][2] = v[10];
        m.m[2][3] = v[11];
        m.m[3][0] = v[12];
        m.m[3][1] = v[13];
        m.m[3][2] = v[14];
        m.m[3][3] = v[15];

        m
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix3x3 {
        let mut submat = Matrix3x3::new();
        let mut sr = 0;

        for i in 0..4 {
            if i != row {
                let mut sc = 0;
                for j in 0..4 {
                    if j != col {
                        submat.m[sr][sc] = self.m[i][j];
                        sc += 1;
                    }
                }
                sr += 1;
            }
        }
        submat
    }

    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let sm = self.submatrix(row,col);
        sm.det()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        self.minor(row,col) * if ((row + col) % 2) == 1 { -1.0 } else { 1.0 } 
    }

    pub fn det(&self) -> f64 {
        let mut sum_cofactors_row1 = 0.0;
        for i in 0..self.m[0].len() {
            sum_cofactors_row1 += self.m[0][i] * self.cofactor(0, i);

        }
        sum_cofactors_row1
    }

    pub fn is_invertible(&self) -> bool {
        return !f64_eq(self.det(), 0.0)
    }

    pub fn inverse(&self) -> Matrix4x4 {
        if !self.is_invertible() {
            panic!("matrix not invertible");
        }

        let mut vals = vec![];
        for i in 0..self.m[0].len() {
            for j in 0..self.m[0].len() {
               vals.push(self.cofactor(j, i)/ self.det());
            }
        }
        Matrix4x4::from_vector(&vals)
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

    pub fn mul_vector4d(&self, v: &Vector4D) -> Vector4D {
        let mut vprime = Vector4D::new_vector(0.0, 0.0, 0.0);

        vprime.x = self.m[0][0]*v.x + self.m[0][1]*v.y + self.m[0][2]*v.z + self.m[0][3]*v.w;
        vprime.y = self.m[1][0]*v.x + self.m[1][1]*v.y + self.m[1][2]*v.z + self.m[1][3]*v.w;
        vprime.z = self.m[2][0]*v.x + self.m[2][1]*v.y + self.m[2][2]*v.z + self.m[2][3]*v.w;
        vprime.w = self.m[3][0]*v.x + self.m[3][1]*v.y + self.m[3][2]*v.z + self.m[3][3]*v.w;
        vprime
    }

    pub fn transpose(&self) -> Matrix4x4 {
        let r1 = vec![self.m[0][0], self.m[1][0], self.m[2][0], self.m[3][0]];
        let r2 = vec![self.m[0][1], self.m[1][1], self.m[2][1], self.m[3][1]];
        let r3 = vec![self.m[0][2], self.m[1][2], self.m[2][2], self.m[3][2]];
        let r4 = vec![self.m[0][3], self.m[1][3], self.m[2][3], self.m[3][3]];
        Matrix4x4::from_vectors(&r1, &r2, &r3, &r4)
    }
    
    pub fn translation(xt: f64, yt: f64, zt: f64) -> Matrix4x4 {
        let mut t = Matrix4x4::new();
        t.m[0][3] = xt;
        t.m[1][3] = yt;
        t.m[2][3] = zt;
        t
    }

    pub fn scaling(xs: f64, ys: f64, zs: f64) -> Matrix4x4 {
        let mut s = Matrix4x4::new();
        s.m[0][0] = xs;
        s.m[1][1] = ys;
        s.m[2][2] = zs; 
        s
    }

    pub fn rotate_x(rad: f64) -> Matrix4x4 {
        let mut rx = Matrix4x4::new();
        rx.m[1][1] = rad.cos();
        rx.m[1][2] = -rad.sin();
        rx.m[2][1] = rad.sin();
        rx.m[2][2] = rad.cos();
        rx
    }

    pub fn rotate_y(rad: f64) -> Matrix4x4 {
        let mut ry = Matrix4x4::new();
        ry.m[0][0] = rad.cos();
        ry.m[0][2] = rad.sin();
        ry.m[2][0] = -rad.sin();
        ry.m[2][2] = rad.cos();
        ry
    }

    pub fn rotate_z(rad: f64) -> Matrix4x4 {
        let mut rz = Matrix4x4::new();
        rz.m[0][0] = rad.cos();
        rz.m[0][1] = -rad.sin();
        rz.m[1][0] = rad.sin();
        rz.m[1][1] = rad.cos();
        rz
    }

    pub fn shearing(xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Matrix4x4 {
        let mut sh = Matrix4x4::new();
        sh.m[0][1] = xy;
        sh.m[0][2] = xz;
        sh.m[1][0] = yx;
        sh.m[1][2] = yz;
        sh.m[3][0] = zx;
        sh.m[3][1] = zy;
        sh
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

    pub fn from_vector(v: &[f64]) -> Matrix3x3 {
        let mut m = Matrix3x3 {
            m: [[0.0; 3]; 3]
        };

        m.m[0][0] = v[0];
        m.m[0][1] = v[1];
        m.m[0][2] = v[2];
        m.m[1][0] = v[3];
        m.m[1][1] = v[4];
        m.m[1][2] = v[5];
        m.m[2][0] = v[6];
        m.m[2][1] = v[7];
        m.m[2][2] = v[8];

        m
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Matrix2x2 {
        let mut submat = Matrix2x2::new();
        let mut sr = 0;

        for i in 0..3 {
            if i != row {
                let mut sc = 0;
                for j in 0..3 {
                    if j != col {
                        submat.m[sr][sc] = self.m[i][j];
                        sc += 1;
                    }
                }
                sr += 1;
            }
        }
        submat
    }


    pub fn minor(&self, row: usize, col: usize) -> f64 {
        let sm = self.submatrix(row,col);
        sm.det()
    }

    pub fn cofactor(&self, row: usize, col: usize) -> f64 {
        self.minor(row,col) * if ((row + col) % 2) == 1 { -1.0 } else { 1.0 } 
    }

    pub fn det(&self) -> f64 {
        let mut sum_cofactors_row1 = 0.0;
        for i in 0..self.m[0].len() {
            sum_cofactors_row1 += self.m[0][i] * self.cofactor(0, i);

        }
        sum_cofactors_row1
    }

    pub fn is_invertible(&self) -> bool {
        return !f64_eq(self.det(), 0.0)
    }
    
    pub fn inverse(&self) -> Matrix3x3 {
        if !self.is_invertible() {
            panic!("matrix not invertible");
        }

        let mut vals = vec![];
        for i in 0..self.m[0].len() {
            for j in 0..self.m[0].len() {
               vals.push(self.cofactor(i, j)/ self.det());
            }
        }
        Matrix3x3::from_vector(&vals)
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

    pub fn transpose(&self) -> Matrix3x3 {
        let r1 = vec![self.m[0][0], self.m[1][0], self.m[2][0]];
        let r2 = vec![self.m[0][1], self.m[1][1], self.m[2][1]];
        let r3 = vec![self.m[0][2], self.m[1][2], self.m[2][2]];
        Matrix3x3::from_vectors(&r1, &r2, &r3)
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
    
    pub fn from_vector(v: &[f64]) -> Matrix2x2 {
        let mut m = Matrix2x2 {
            m: [[0.0; 2]; 2]
        };

        m.m[0][0] = v[0];
        m.m[0][1] = v[1];
        m.m[1][0] = v[2];
        m.m[1][1] = v[3];

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

    pub fn transpose(&self) -> Matrix2x2 {
        let r1 = vec![self.m[0][0], self.m[1][0]];
        let r2 = vec![self.m[0][1], self.m[1][1]];
        Matrix2x2::from_vectors(&r1, &r2)
    }

    pub fn det(&self) -> f64 {
        self.m[0][0] * self.m[1][1] - self.m[0][1]*self.m[1][0]
    }

}


pub struct MatrixChainer{
    operations: Vec<Matrix4x4>,
}



impl MatrixChainer {
    pub fn new() -> MatrixChainer {
        MatrixChainer {
            operations: vec![]
        }
    }
    pub fn then<'a>(&'a mut self, m: Matrix4x4) -> &'a mut MatrixChainer {
       self.operations.push(m);
       self
    }

    pub fn finish(&mut self) -> Matrix4x4 {
       let mut m : Matrix4x4;
       self.operations = self.operations.iter().rev().cloned().collect();
       if let Some(mm) = self.operations.pop() {
           m = mm;
           while let Some(n) = self.operations.pop() {
               m = n.mul(&m);
           }
           return m
       }
       panic!("no operations on MatrixChainer");
    }
}
   

pub fn view_transformation(from: Vector4D, to: Vector4D, up: Vector4D) -> Matrix4x4 {
    let forward = (to - from).normalized();
    let left = forward.cross(up.normalized());
    let true_up = left.cross(forward);
    let mut orientation_m = Matrix4x4::new();
    orientation_m.m[0][0] = left.x;
    orientation_m.m[0][1] = left.y;
    orientation_m.m[0][2] = left.z;
    orientation_m.m[1][0] = true_up.x;
    orientation_m.m[1][1] = true_up.y;
    orientation_m.m[1][2] = true_up.z;
    orientation_m.m[2][0] = -forward.x;
    orientation_m.m[2][1] = -forward.y;
    orientation_m.m[2][2] = -forward.z;
    orientation_m.mul(&Matrix4x4::translation(-from.x, -from.y, -from.z))
}
