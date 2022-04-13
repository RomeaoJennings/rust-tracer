use crate::primitives::{HitRecord, Point, Ray, SquareMatrix};

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    transform: SquareMatrix,
}

impl Sphere {
    pub fn identity() -> Self {
        Sphere {
            transform: SquareMatrix::identity(4),
        }
    }

    pub fn new(transform: SquareMatrix) -> Self {
        Sphere {
            transform: transform.invert(),
        }
    }

    pub fn set_transform(&mut self, transform: SquareMatrix) {
        self.transform = transform.invert();
    }

    pub fn get_hits(&self, ray: &Ray) -> Vec<HitRecord> {
        let mut result = Vec::new();
        let ray = ray.transform(&self.transform);
        let sphere_to_ray = &(ray.get_origin() - &Point::new(0., 0., 0.));
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
        result.push(HitRecord::new((b - discriminant) / a, self));
        if discriminant > 0.0 {
            result.push(HitRecord::new((b + discriminant) / a, self));
        }
        result
    }
}

#[cfg(test)]
#[path = "tests/sphere_tests.rs"]
mod tests;
