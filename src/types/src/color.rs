use std::default::Default;
use std::ops::{Add, Mul, Sub};

// Values are in the range 0.0 - 1.0
#[derive(Debug, Copy, Clone)]
pub struct Color {
    pub red: f64,
    pub green: f64,
    pub blue: f64,
}

impl Color {
    pub fn new(red: f64, green: f64, blue: f64) -> Color {
        Color {
            red,
            green,
            blue
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color::new(0.0, 0.0, 0.0)
    }
}
// color addition
impl Add<Color> for Color {
    type Output = Color;
    fn add(self, other: Color) -> Color {
        Color {
            red: self.red + other.red,
            green: self.green + other.green,
            blue: self.blue + other.blue,
        }
    }
}

// color addition
impl Sub<Color> for Color {
    type Output = Color;
    fn sub(self, other: Color) -> Color {
        Color {
            red: self.red - other.red,
            green: self.green - other.green,
            blue: self.blue - other.blue,
        }
    }
}

// color mul
impl Mul<Color> for Color {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color {
            red: self.red * other.red,
            green: self.green * other.green,
            blue: self.blue * other.blue,
        }
    }
}

// scalar * color
impl Mul<Color> for f64 {
    type Output = Color;
    fn mul(self, other: Color) -> Color {
        Color {
            red: self * other.red,
            green: self * other.green,
            blue: self * other.blue,
        }
    }
}

// color * scalar 
impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, other: f64) -> Color {
        other * self
    }
}