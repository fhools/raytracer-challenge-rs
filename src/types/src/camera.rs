use crate::Vector4D;
use crate::Matrix4x4;
use crate::Ray;

pub struct Camera {
    pub hsize_px: usize,
    pub vsize_px: usize,
    pub fov: f64,
    pub transform: Matrix4x4,
    pub pixel_size: f64,
    pub half_width: f64,
    pub half_height: f64,
}

impl Camera {
    pub fn new(hsize_px: usize, vsize_px: usize, fov: f64) -> Camera {
        let half_view = (fov/2.0).tan();
        let aspect = hsize_px as f64 / vsize_px as f64;
        let half_width;
        let half_height;
        if aspect >= 1.0 {
            half_width = half_view;
            half_height = half_view / aspect;
        } else {
            half_width = half_view * aspect;
            half_height = half_view;
        }
        let pixel_size = (half_width * 2.0) / (hsize_px as f64);
        Camera {
            hsize_px, 
            vsize_px,
            fov,
            transform: Matrix4x4::new(),
            pixel_size,
            half_width,
            half_height
        }
    }
}

pub fn ray_for_pixel(camera: &Camera, px: usize, py: usize) -> Ray {
    // Offset in the canvas in world_space
    // We are adding .5 to go to pixels center, when I left this off, the test was off by
    // .004999
    let xoffset = (px as f64 + 0.5) * camera.pixel_size;
    let yoffset = (py as f64 + 0.5) * camera.pixel_size;

    // x,y coordinate of the pixel in world space, (camera looks down at -z, so +x is to the left)
    let world_x = camera.half_width - xoffset;
    let world_y = camera.half_height - yoffset;

    let world_pixel = camera.transform
        .inverse()
        .mul_vector4d(&Vector4D::new_point(world_x, world_y, -1.0));
    // We are multplying the origin vector to get the translation component. Kinda looks
    // weird since multiplying by a vector that is 0,0,0 would usually just return 0 vector.
    // This is where the 4D part comes in I guess.
    let origin = camera.transform.inverse().mul_vector4d(&Vector4D::new_point(0.0, 0.0, 0.0)); 
    let dir = (world_pixel - origin).normalized();
    Ray::new(origin, dir)
}
