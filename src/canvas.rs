use types::*;

use std::fs::OpenOptions;
use std::path::Path;
use std::io::prelude::*;
use std::io::BufWriter;
use std::error::Error;

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

    pub fn set_pixel(&mut self, x: usize, y: usize, c: &Color) {
        if x >= self.width  {
            panic!("x: {} >= width: {}", x, self.width);
        }
        if y >= self.height {
            panic!("y: {} >= height: {}", y, self.height);
        }
        self.canvas[y][x] = *c;
    }

    pub fn get_pixel(&self, x: usize, y: usize) -> Color {
        if x >= self.width  {
            panic!("x: {} >= width: {}", x, self.width);
        }
        if y >= self.height {
            panic!("y: {} >= height: {}", y, self.height);
        }
        self.canvas[y][x]
    }
    

    pub fn write_ppm<P: AsRef<Path>>(&self, filepath: P) -> Result<(), Box<dyn Error>> {
        let mut f = OpenOptions::new().write(true).create(true).open(filepath)?;
        f.write(&self.write_ppm_str().as_bytes())?;
        Ok(())
    }

    pub fn write_ppm_str(&self) -> String {
        const MAX_COL : usize = 70;
        let mut bw = BufWriter::new(Vec::new());

        // Write magic
        write!(bw, "P3\n").expect("write ppm failed");

        // width height
        write!(bw, "{} {}\n", self.width, self.height).expect("write ppm failed");

        // maximum color value
        write!(bw, "{}\n", 255).expect("write ppm failed");

        // pixels, maximum 70 characters in each line
        // try to put one line for each row of pixels, except if it'
        // greater than 70 chars continue to next line
        let pixels = self.canvas.iter().flatten()
            .map(|color| vec![color.red, color.green, color.blue])
            .flatten();
        let mut line_length = 0;
        let mut cur_col = 0; 
        for  px in pixels {
            cur_col += 1;
            let val = (255.0 * px).round().clamp(0.0, 255.0);
            let next_val = format!("{:.0}", val);
            if line_length + next_val.len() + 1 > MAX_COL {
                write!(bw, "\n").expect("write ppm failed");
                line_length = 0;
            } 
            // output space after previous entry for entries after first one
            if cur_col > 1  {
                write!(bw, " ").expect("write ppm failed");
                line_length += 1;
            }
            write!(bw, "{}", next_val).expect("write ppm failed");

            if cur_col >= self.width*3 {
                write!(bw, "\n").expect("write ppm failed");
                cur_col = 0;
                line_length = 0;
            }
            line_length += next_val.len();
        }

        write!(bw, "\n").expect("write ppm failed");

        String::from_utf8(bw.into_inner().unwrap()).unwrap()
    }
}
