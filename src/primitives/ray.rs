use super::{Point, SquareMatrix, Vector};

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

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Self {
        let matrix = &SquareMatrix::translation(x, y, z);
        Ray::new(matrix * self.get_origin(), matrix * self.get_direction())
    }

    pub fn rotate_x(&self, radians: f64) -> Self {
        let matrix = &SquareMatrix::rotation_x(radians);
        Ray::new(matrix * self.get_origin(), matrix * self.get_direction())
    }

    pub fn rotate_y(&self, radians: f64) -> Self {
        let matrix = &SquareMatrix::rotation_y(radians);
        Ray::new(matrix * self.get_origin(), matrix * self.get_direction())
    }

    pub fn rotate_z(&self, radians: f64) -> Self {
        let matrix = &SquareMatrix::rotation_z(radians);
        Ray::new(matrix * self.get_origin(), matrix * self.get_direction())
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Self {
        let matrix = &SquareMatrix::scaling(x, y, z);
        Ray::new(matrix * self.get_origin(), matrix * self.get_direction())
    }

    pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        let matrix = &SquareMatrix::shearing(xy, xz, yx, yz, zx, zy);
        Ray::new(matrix * self.get_origin(), matrix * self.get_direction())
    }
}

#[cfg(test)]
#[path = "tests/ray_tests.rs"]
mod tests;
