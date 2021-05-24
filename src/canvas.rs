use utils::*;
use types::*;
pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub canvas: Vec<Vec<Color>>,
}


impl Canvas {
    pub fn new(width: usize, height: usize) -> Canvas {
        let mut canvas = Canvas {
            width,
            height,
            canvas: vec![]
        };

        for _ in 0..height {
            canvas.canvas.push(vec![Default::default(); width]);
        }
        canvas
    }
}