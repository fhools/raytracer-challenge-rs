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
    pub no_cast_shadow: bool,
    pub reflective: f64,
    pub refractive_index: f64,
    pub transparency: f64,
    
}

pub mod refractive_indices {
    pub const AIR: f64 = 1.00029;
    pub const GLASS: f64 = 1.52;
    pub const DIAMOND: f64 = 2.417;
    pub const VACUUM: f64 = 1.0;
    pub const DEFAULT: f64 = VACUUM;
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
            no_cast_shadow: false,
            reflective: 0.0,
            refractive_index: refractive_indices::DEFAULT,
            transparency: 0.0,
        }
    }
}
