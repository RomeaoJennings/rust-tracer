use crate::{lighting::PointLight, objects::Hittable};

use super::{Ray, HitRecord};

pub struct World {
    lights: Vec<PointLight>,
    objects: Vec<Box<dyn Hittable>>,
}

impl World {
    pub fn new() -> Self {
        World {
            lights: Vec::new(),
            objects: Vec::new(),
        }
    }

    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    pub fn add_object<T: Hittable + 'static>(&mut self, object: T) {
        self.objects.push(Box::new(object));
    }

    pub fn get_lights(&self) -> &Vec<PointLight> {
        &self.lights
    }

    pub fn get_objects(&self) -> &Vec<Box<dyn Hittable>> {
        &self.objects
    }

    pub fn get_hits(&self, ray: &Ray) -> Vec<HitRecord> {
        let mut result = Vec::new();
        for object in self.objects.iter() {
            result.append(&mut object.get_hits(&ray));
        }
        result.sort_unstable_by(|a, b| a.cmp_ignore_nan(b));
        result
    }

    pub fn get_first_visible_hit<'a>(hits: &'a Vec<HitRecord>) -> Option<&'a HitRecord<'a>> {
        for hit in hits.iter() {
            if hit.get_t() > 0.0 {
                return Some(hit);
            }
        }
        None
    }
}
