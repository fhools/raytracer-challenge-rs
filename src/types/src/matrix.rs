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


    pub fn transpose(&self) -> Matrix4x4 {
        let r1 = vec![self.m[0][0], self.m[1][0], self.m[2][0], self.m[3][0]];
        let r2 = vec![self.m[0][1], self.m[1][1], self.m[2][1], self.m[3][1]];
        let r3 = vec![self.m[0][2], self.m[1][2], self.m[2][2], self.m[3][2]];
        let r4 = vec![self.m[0][3], self.m[1][3], self.m[2][3], self.m[3][3]];
        Matrix4x4::from_vectors(&r1, &r2, &r3, &r4)
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
