use super::{Point, Vector};

#[derive(Clone, Debug, PartialEq)]
pub struct Ray {
    origin: Point,
    direction: Vector,
}

impl Ray {
    pub fn new(origin: Point, direction: Vector) -> Self {
        Ray { origin, direction }
    }

    pub fn get_origin(&self) -> &Point {
        &self.origin
    }

    pub fn get_direction(&self) -> &Vector {
        &self.direction
    }

    pub fn at(&self, t: f64) -> Point {
        self.get_origin() + &(self.get_direction() * t)
    }
}

#[cfg(test)]
#[path = "tests/ray_tests.rs"]
mod tests;
