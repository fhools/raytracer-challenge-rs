use crate::Color;

#[derive(Debug, Copy, Clone, Default)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64
}

impl Material {
    pub fn new(color: Color) -> Material {
        Material {
            color,
            ..Default::default()
        }
    }
}
