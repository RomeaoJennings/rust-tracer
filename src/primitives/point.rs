use std::ops::{Add, Div, Mul, Neg, Sub, Index};

use super::Vector;

const W:f64 = 1.0;

#[derive(Debug, Clone, PartialEq)]
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
}

impl Add<&Vector> for &Point {
    type Output = Vector;

    fn add(self, rhs: &Vector) -> Self::Output {
        Vector::new(
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

impl Index<usize> for Point{
    type Output = f64;

    fn index(&self, index: usize) -> &Self::Output {
        match index {
            0 => &self.x,
            1 => &self.y,
            2 => &self.z,
            _ => &W
        }
    }
}

#[cfg(test)]
#[path = "tests/point_tests.rs"]
mod tests;
