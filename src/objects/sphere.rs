use crate::primitives::{HitRecord, Point, Ray, SquareMatrix};

#[derive(Clone, Debug, PartialEq)]
pub struct Sphere {
    transform: SquareMatrix,
    inverted: SquareMatrix,
}

impl Sphere {
    pub fn identity() -> Self {
        Sphere {
            transform: SquareMatrix::identity(4),
            inverted: SquareMatrix::identity(4),
        }
    }

    pub fn new(center: Point, radius: f64) -> Self {
        let mut transform = SquareMatrix::translation(center.get_x(), center.get_y(), center.get_z());
        transform = &transform * &SquareMatrix::scaling(radius, radius, radius);
        Sphere::with_transform(transform)
    }

    pub fn with_transform(transform: SquareMatrix) -> Self {
        Sphere {
            inverted: transform.invert(),
            transform,
        }
    }

    pub fn set_transform(&mut self, transform: SquareMatrix) {
        self.inverted = transform.invert();
        self.transform = transform;
    }

    pub fn get_hits(&self, ray: &Ray) -> Vec<HitRecord> {
        let mut result = Vec::new();
        let ray = ray.transform(&self.inverted);
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
