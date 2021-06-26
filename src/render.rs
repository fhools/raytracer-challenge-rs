use types::*;
use crate::Canvas;
const REFLECT_RAYS: usize = 2;
pub fn render(camera: &Camera, world: &World, canvas: &mut Canvas) {
    for y in 0..(camera.vsize_px - 1) {
        for x in 0..(camera.hsize_px - 1) {
            let ray = ray_for_pixel(&camera, x, y); 
            let color = color_at(&world, ray, 0);
            canvas.set_pixel(x, y, &color);
        }
    }
}

// TODO: We need to setup a RenderConfigurationObject have render take this object 
// instead of what we're doing here making a copy of render with a non zero reflect rays
// argument passed to color_at
pub fn render_with_reflection(camera: &Camera, world: &World, canvas: &mut Canvas) {
    for y in 0..(camera.vsize_px - 1) {
        for x in 0..(camera.hsize_px - 1) {
            let ray = ray_for_pixel(&camera, x, y); 
            let color = color_at(&world, ray, REFLECT_RAYS);
            canvas.set_pixel(x, y, &color);
        }
    }
}
