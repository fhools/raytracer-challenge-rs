use types::*;
use crate::Canvas;
pub fn render(camera: &Camera, world: &World, canvas: &mut Canvas) {
    for y in 0..(camera.vsize_px - 1) {
        for x in 0..(camera.hsize_px - 1) {
            let ray = ray_for_pixel(&camera, x, y); 
            let color = color_at(&world, ray);
            canvas.set_pixel(x, y, &color);
        }
    }
}
