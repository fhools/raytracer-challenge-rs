use types::*;
use utils::*;
//use raytracer_challenge_rs::lib::Canvas;
extern crate raytracer_challenge_rs;

use raytracer_challenge_rs::Canvas;
#[test]
fn color_new() {
    let c = Color::new(0.5, 0.2, 1.0);
    assert!(f64_eq(c.red, 0.5));
    assert!(f64_eq(c.green, 0.2));
    assert!(f64_eq(c.blue, 1.0));
}

#[test]
fn color_mul_scalar() {
    let c = Color::new(0.3, 0.2, 1.0);
    let d = 2.0 * c;
    let e = c * 2.0;
    assert!(f64_eq(d.red, 0.6));
    assert!(f64_eq(d.green, 0.4));
    assert!(f64_eq(d.blue, 2.0));

    assert!(f64_eq(e.red, 0.6));
    assert!(f64_eq(e.green, 0.4));
    assert!(f64_eq(e.blue, 2.0));
}


#[test]
fn color_add() {
    let c = Color::new(0.1, 0.2, 0.3);
    let d = c + c;
    assert!(f64_eq(d.red, 0.2));
    assert!(f64_eq(d.green, 0.4));
    assert!(f64_eq(d.blue, 0.6));
}

#[test]
fn color_sub() {
    let c = Color::new(0.1, 0.2, 0.3);
    let mut d = Color::new(0.2, 0.3, 0.4);
    d = c - d;
    assert!(f64_eq(d.red, -0.1));
    assert!(f64_eq(d.green, -0.1));
    assert!(f64_eq(d.blue, -0.1));
}

#[test]
fn canvas_new() {
    let mut canvas = Canvas::new(15,3);
    canvas.set_pixel(0,0, &Color::new(0.5, 0.2, 1.0));
    let ppm_str = canvas.write_ppm_str();
    println!("{}", ppm_str);
    assert!(ppm_str.len() != 0);
}

#[test]
fn canvas_write_ppm() {
    let c1 = Color::new(1.5, 0.0, 0.0);
    let c2 = Color::new(0.0, 0.5, 0.0);
    let c3 = Color::new(-0.5, 0.0, 1.0);
    let mut canvas = Canvas::new(5,3);
    canvas.set_pixel(0,0, &c1);
    canvas.set_pixel(2,1, &c2);
    canvas.set_pixel(4,2, &c3);
    canvas.write_ppm("my.ppm").expect("unable to write");
    let ppm_str = canvas.write_ppm_str();

    let lines : Vec<&str> = ppm_str.split("\n").collect();
    assert_eq!(lines[3], "255 0 0 0 0 0 0 0 0 0 0 0 0 0 0"); 
    assert_eq!(lines[4], "0 0 0 0 0 0 0 128 0 0 0 0 0 0 0"); 
    assert_eq!(lines[5], "0 0 0 0 0 0 0 0 0 0 0 0 0 0 255"); 

}
