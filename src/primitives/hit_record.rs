use crate::objects::{Hittable, Sphere};

use super::{Hit, Point, Ray, Vector};

pub struct HitRecord<'a> {
    t: f64,
    object: &'a Sphere,
    hit_point: Point,
    eye_vector: Vector,
    normal_vector: Vector,
    is_inside: bool,
}

impl<'a> HitRecord<'a> {
    pub fn new(hit: &'a Hit, ray: &Ray) -> Self {
        let t = hit.get_t();
        let hit_point = ray.at(t);
        let eye_vector = -ray.get_direction();
        let mut normal_vector = hit.get_object().get_normal(&hit_point);
        let is_inside = if &normal_vector * &eye_vector < 0.0 {
            true
        } else {
            false
        };
        if is_inside {
            normal_vector = -&normal_vector;
        }
        HitRecord {
            t,
            object: hit.get_object(),
            normal_vector,
            hit_point,
            eye_vector,
            is_inside,
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

    pub fn get_is_inside(&self) -> bool {
        self.is_inside
    }
}
