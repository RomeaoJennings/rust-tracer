use std::ops::{Add, Div, Index, Mul, Neg, Sub};

use super::{SquareMatrix, Vector};

const W: f64 = 1.0;

#[derive(Debug, Clone)]
pub struct Point {
    x: f64,
    y: f64,
    z: f64,
}

impl Point {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Point { x, y, z }
    }

    pub fn get_x(&self) -> f64 {
        self.x
    }

    pub fn get_y(&self) -> f64 {
        self.y
    }

    pub fn get_z(&self) -> f64 {
        self.z
    }

    pub fn translate(&self, x: f64, y: f64, z: f64) -> Self {
        &SquareMatrix::translation(x, y, z) * self
    }

    pub fn rotate_x(&self, radians: f64) -> Self {
        &SquareMatrix::rotation_x(radians) * self
    }

    pub fn rotate_y(&self, radians: f64) -> Self {
        &SquareMatrix::rotation_y(radians) * self
    }

    pub fn rotate_z(&self, radians: f64) -> Self {
        &SquareMatrix::rotation_z(radians) * self
    }

    pub fn scale(&self, x: f64, y: f64, z: f64) -> Self {
        &SquareMatrix::scaling(x, y, z) * self
    }

    pub fn shear(&self, xy: f64, xz: f64, yx: f64, yz: f64, zx: f64, zy: f64) -> Self {
        &SquareMatrix::shearing(xy, xz, yx, yz, zx, zy) * self
    }
}

impl Add<&Vector> for &Point {
    type Output = Point;

    fn add(self, rhs: &Vector) -> Self::Output {
        Point::new(
            self.x + rhs.get_x(),
            self.y + rhs.get_y(),
            self.z + rhs.get_z(),
        )
    }
}

impl Sub for &Point {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector::new(self.x - rhs.x, self.y - rhs.y, self.z - rhs.z)
    }
}

impl Sub<&Vector> for &Point {
    type Output = Point;

    fn sub(self, rhs: &Vector) -> Self::Output {
        Point {
            x: self.x - rhs.get_x(),
            y: self.y - rhs.get_y(),
            z: self.z - rhs.get_z(),
        }
    }
}

impl Neg for &Point {
    type Output = Point;

    fn neg(self) -> Self::Output {
        Point::new(-self.x, -self.y, -self.z)
    }
}

impl Mul<f64> for &Point {
    type Output = Point;

    fn mul(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for &Point {
    type Output = Point;

    fn div(self, rhs: f64) -> Self::Output {
        Point {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Index<usize> for Point {
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &W,
        }
    }
}

impl PartialEq for Point {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y) && approx_eq(self.z, other.z)
    }
}

fn approx_eq(one: f64, two: f64) -> bool {
    const EPSILON: f64 = 1e-6;
    (one - two).abs() < EPSILON
}

#[cfg(test)]
#[path = "tests/point_tests.rs"]
mod tests;
