use crate::Color;
use crate::Pattern;

#[derive(Debug, Clone)]
pub struct Material {
    pub color: Color,
    pub ambient: f64,
    pub diffuse: f64,
    pub specular: f64,
    pub shininess: f64,
    pub pattern: Option<Box<Pattern>>,
}

impl Material {
    pub fn new(color: Color) -> Material {
        Material {
            color,
            ..Default::default()
        }
    }

}

impl Default for Material {
    fn default() -> Self {
        Material {
            color: Color::new(1.0, 1.0, 1.0),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            pattern:  None,
        }
    }
}
