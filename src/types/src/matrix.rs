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
}
