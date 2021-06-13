use crate::Color;
use crate::Matrix4x4;
use crate::Vector4D;
use crate::Intersectable;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Pattern {
    StripePattern(StripePattern),
    RingPattern(RingPattern)
}

pub trait Patternable {
    fn set_transform(&mut self, transform: Matrix4x4);
    fn get_transform(&self) -> Matrix4x4;
    fn pattern_at(&self, point: Vector4D) -> Color;
    fn pattern_at_object(&self, obj: &dyn Intersectable, world_point: Vector4D) -> Color {
        let object_point = obj.get_transform().inverse().mul_vector4d(&world_point);
        let pattern_point = self.get_transform().inverse().mul_vector4d(&object_point);
        self.pattern_at(pattern_point)
    }
}

// A strip pattern creates a pattern that alternatives along the x-axis
#[derive(Debug, Clone, Copy)]
pub struct StripePattern {
    pub colors: [Color; 2],
    pub transform: Matrix4x4,
}

impl StripePattern {
    pub fn new(color_1: Color, color_2: Color) -> StripePattern {
        StripePattern {
            colors: [color_1, color_2],
            transform: Matrix4x4::new(),
        }
    }
}


//impl std::fmt::Debug for StripePattern {
//    fn  fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//        write!(fmt, "StripePattern (Colors: {:?}, Transform: {:?}", self.colors, self.transform)
//    }
//}


impl Patternable for StripePattern {
    fn get_transform(&self) -> Matrix4x4 {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix4x4) {
        self.transform = transform;
    }

    fn pattern_at(&self, point: Vector4D) -> Color {
        if point.x.floor() as i64 % 2 == 0 {
            self.colors[0]
        } else {
            self.colors[1]
        }
    }

}

impl Default for StripePattern {
    fn default() -> Self {
        StripePattern {
            colors: [Color::white(), Color::black()],
            transform: Matrix4x4::new()
        }
    }
}

// A strip pattern creates a pattern that alternatives along the x-axis
#[derive(Debug, Clone, Copy)]
pub struct RingPattern {
    pub colors: [Color; 2],
    pub transform: Matrix4x4,
}

impl RingPattern {
    pub fn new(color_1: Color, color_2: Color) -> RingPattern {
        RingPattern {
            colors: [color_1, color_2],
            transform: Matrix4x4::new(),
        }
    }
}


//impl std::fmt::Debug for StripePattern {
//    fn  fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//        write!(fmt, "StripePattern (Colors: {:?}, Transform: {:?}", self.colors, self.transform)
//    }
//}


impl Patternable for RingPattern {
    fn get_transform(&self) -> Matrix4x4 {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix4x4) {
        self.transform = transform;
    }

    fn pattern_at(&self, point: Vector4D) -> Color {
        if (point.x.powf(2.0) + point.z.powf(2.0)).sqrt().floor() as i64 % 2 == 0 {
            self.colors[0]
        } else {
            self.colors[1]
        }
    }

}

impl Default for RingPattern {
    fn default() -> Self {
        RingPattern {
            colors: [Color::white(), Color::black()],
            transform: Matrix4x4::new()
        }
    }
}
