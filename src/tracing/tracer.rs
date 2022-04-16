use crate::{
    cameras::Camera,
    primitives::{Canvas, World, RgbColor},
};

pub struct Tracer;

impl Tracer {
    pub fn trace_world(world: &World, camera: &Camera, canvas: &mut Canvas) {
        for y in 0..canvas.get_height() {
            for x in 0..canvas.get_width() {
                let ray = camera.get_ray(x, y);
                let all_hits = world.get_hits(&ray);
    
                if let Some(hit) = World::get_first_visible_hit(&all_hits) {
                    let record = hit.get_hit_record(&ray);
                    let mut color = RgbColor::new(0., 0., 0.);
                    for light in world.get_lights().iter() {
                        color = &color + &light.shade(&record);
                    }
                    canvas.set_pixel(y, x, color);
                }
            }
        }
        canvas.correct_colors();
    }
}
