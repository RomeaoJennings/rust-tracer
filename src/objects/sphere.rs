use crate::primitives::{Ray, Point};

pub struct Sphere;

impl Sphere {
    pub fn get_hits(&self, ray: &Ray) -> Vec<f64> {
        let mut result = Vec::new();

        let sphere_to_ray = &(ray.get_origin() - &Point::new(0.,0.,0.));
        let mut a = ray.get_direction() * ray.get_direction();
        let mut b = (ray.get_direction() * sphere_to_ray) * 2.0;
        let c = sphere_to_ray * sphere_to_ray - 1.0;

        let mut discriminant = b * b - 4.0 * a * c;
        if discriminant < 0. {
            return result;
        }
        discriminant = discriminant.sqrt();
        a *= 2.0;
        b *= -1.0;
        result.push((b - discriminant) / a); 
        if discriminant > 0.0 {
            result.push((b + discriminant) / a);
        }
        result
    }
}

#[cfg(test)]
#[path = "tests/sphere_tests.rs"]
mod tests;
