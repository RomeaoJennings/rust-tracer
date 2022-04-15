use crate::objects::{Hittable, Sphere};

use super::{Hit, Point, Ray, Vector};

pub struct HitRecord<'a> {
    t: f64,
    object: &'a Sphere,
    hit_point: Point,
    eye_vector: Vector,
    normal_vector: Vector,
}

impl<'a> HitRecord<'a> {
    pub fn new(hit: &'a Hit, ray: &Ray) -> Self {
        let t = hit.get_t();
        let hit_point = ray.at(t);
        HitRecord {
            t,
            object: hit.get_object(),
            normal_vector: hit.get_object().get_normal(&hit_point),
            hit_point,
            eye_vector: -ray.get_direction(),
        }
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }

    pub fn get_object(&self) -> &'a Sphere {
        self.object
    }

    pub fn get_hit_point(&self) -> &Point {
        &self.hit_point
    }

    pub fn get_eye_vector(&self) -> &Vector {
        &self.eye_vector
    }

    pub fn get_normal(&self) -> &Vector {
        &self.normal_vector
    }
}
