use crate::Color;
use crate::Matrix4x4;
use crate::Vector4D;
use crate::Intersectable;
use std::fmt::Debug;

#[derive(Debug, Clone)]
pub enum Pattern {
    StripePattern(StripePattern),
    RingPattern(RingPattern),
    GradientPattern(GradientPattern),
    CheckeredPattern(CheckeredPattern),
    TestPattern(TestPattern),
}

pub trait Patternable {
    fn set_transform(&mut self, transform: Matrix4x4);
    fn get_transform(&self) -> Matrix4x4;
    fn pattern_at(&self, point: Vector4D) -> Color;
    fn pattern_at_object(&self, obj: &dyn Intersectable, world_point: Vector4D) -> Color {
        let object_point = obj.world_to_object(world_point);
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

    pub fn test() -> StripePattern {
        StripePattern {
            colors: [Color::new(1.0, 1.5, 2.0), Color::new(0.75, 0.5, 0.25)],
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
            colors: [Color::white(), Color::BLACK],
            transform: Matrix4x4::new()
        }
    }
}

// A ring pattern pattern creates a pattern that alternatives colors in concentric rings 
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
        if (point.x.powi(2) + point.z.powi(2)).sqrt().floor() as i64 % 2 == 0 {
            self.colors[0]
        } else {
            self.colors[1]
        }
    }

}

impl Default for RingPattern {
    fn default() -> Self {
        RingPattern {
            colors: [Color::white(), Color::BLACK],
            transform: Matrix4x4::new()
        }
    }
}

// A gradient pattern pattern creates a pattern that interpolates between 2 colores along the
// x-axis 

#[derive(Debug, Clone, Copy)]
pub struct GradientPattern {
    pub colors: [Color; 2],
    pub transform: Matrix4x4,
}

impl GradientPattern {
    pub fn new(color_1: Color, color_2: Color) -> GradientPattern {
        GradientPattern {
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


impl Patternable for GradientPattern {
    fn get_transform(&self) -> Matrix4x4 {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix4x4) {
        self.transform = transform;
    }

    fn pattern_at(&self, point: Vector4D) -> Color {
        let distance = self.colors[1] - self.colors[0];
        let fraction = point.x - point.x.floor();
        return self.colors[0] + distance * fraction;
    }

}

impl Default for GradientPattern {
    fn default() -> Self {
        GradientPattern {
            colors: [Color::white(), Color::BLACK],
            transform: Matrix4x4::new()
        }
    }
}

// A checkered pattern pattern creates a pattern that interpolates between 2 colores along the
// x-axis 

#[derive(Debug, Clone, Copy)]
pub struct CheckeredPattern {
    pub colors: [Color; 2],
    pub transform: Matrix4x4,
}

impl CheckeredPattern {
    pub fn new(color_1: Color, color_2: Color) -> CheckeredPattern {
        CheckeredPattern {
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


impl Patternable for CheckeredPattern {
    fn get_transform(&self) -> Matrix4x4 {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix4x4) {
        self.transform = transform;
    }

    fn pattern_at(&self, point: Vector4D) -> Color {
        let sum = point.x.floor() + point.y.floor() + point.z.floor();
        if sum as i64 % 2 == 0 {
            self.colors[0]
        } else {
            self.colors[1]
        }
    }

}

impl Default for CheckeredPattern {
    fn default() -> Self {
        CheckeredPattern {
            colors: [Color::white(), Color::BLACK],
            transform: Matrix4x4::new()
        }
    }
}

// A checkered pattern pattern creates a pattern that interpolates between 2 colores along the
// x-axis 

#[derive(Debug, Clone, Copy)]
pub struct TestPattern {
    pub transform: Matrix4x4,
}

impl TestPattern {
    pub fn new() -> TestPattern {
        TestPattern {
            transform: Matrix4x4::new(),
        }
    }
}


//impl std::fmt::Debug for StripePattern {
//    fn  fmt(&self, fmt: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
//        write!(fmt, "StripePattern (Colors: {:?}, Transform: {:?}", self.colors, self.transform)
//    }
//}


impl Patternable for TestPattern {
    fn get_transform(&self) -> Matrix4x4 {
        self.transform
    }

    fn set_transform(&mut self, transform: Matrix4x4) {
        self.transform = transform;
    }

    fn pattern_at(&self, point: Vector4D) -> Color {
        println!("Requested TestPattern at: {:?}", point);
        Color::new(point.x, point.y, point.z)
    }

}

impl Default for TestPattern {
    fn default() -> Self {
        TestPattern {
            transform: Matrix4x4::new()
        }
    }
}
