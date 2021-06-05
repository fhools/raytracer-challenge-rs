use std::default::Default;
use std::ops::{Add, Mul, Sub};
use std::cmp::PartialEq;
use utils::f64_eq;

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

    pub fn black() -> Color {
        Color {
            red: 0.0,
            green: 0.0,
            blue: 0.0
        }
    }
}

impl Default for Color {
    fn default() -> Self {
        Color {
            red: Default::default(),
            green: Default::default(),
            blue: Default::default()
        }
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


impl PartialEq<Color> for Color {
    fn eq(&self, other: &Color) -> bool {
        f64_eq(self.red, other.red) &&
        f64_eq(self.green, other.green) && 
        f64_eq(self.blue, other.blue) 
    }
}

// color * scalar 
impl Mul<f64> for Color {
    type Output = Color;
    fn mul(self, other: f64) -> Color {
        other * self
    }
}
