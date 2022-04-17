use crate::{
    cameras::Camera,
    primitives::{Canvas, Ray, RgbColor, World},
};

pub struct Tracer;

impl Tracer {
    pub fn trace_world(world: &World, camera: &Camera, canvas: &mut Canvas) {
        for y in 0..canvas.get_height() {
            for x in 0..canvas.get_width() {
                let ray = camera.get_ray(x, y);
                let all_hits = world.get_hits(&ray);

                if let Some(hit) = World::get_first_visible_hit(&all_hits) {
                    let mut record = hit.get_hit_record(&ray);
                    let mut color = RgbColor::new(0., 0., 0.);
                    let origin = ray.at(record.get_t() - 1e-6);
                    for light in world.get_lights().iter() {
                        let vec = light.get_location() - &origin;
                        let ray_to_light = Ray::new(origin.clone(), vec);
                        record.set_is_in_shadow(Self::check_in_shadow(world, &ray_to_light));
                        color = &color + &light.shade(&record);
                    }
                    canvas.set_pixel(y, x, color);
                }
            }
        }
        canvas.correct_colors();
    }

    fn check_in_shadow(world: &World, ray: &Ray) -> bool {
        let hits = world.get_hits(ray);
        if let Some(hit) = World::get_first_visible_hit(&hits) {
            if hit.get_t() < 1.0 {
                return true;
            }
        }
        false
    }
}
