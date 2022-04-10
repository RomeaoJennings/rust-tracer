use std::ops::{Add, Neg, Sub};

use super::Point;

#[derive(Debug, Clone, PartialEq)]
pub struct Vector {
    x: f64,
    y: f64,
    z: f64,
}

impl Vector {
    pub fn new(x: f64, y: f64, z: f64) -> Self {
        Vector { x, y, z }
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

impl Add for &Vector {
    type Output = Vector;

    fn add(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x + rhs.x,
            y: self.y + rhs.y,
            z: self.z + rhs.z,
        }
    }
}

impl Add<&Point> for &Vector {
    type Output = Vector;

    fn add(self, rhs: &Point) -> Self::Output {
        Vector {
            x: self.x + rhs.get_x(),
            y: self.y + rhs.get_y(),
            z: self.z + rhs.get_z(),
        }
    }
}

impl Sub for &Vector {
    type Output = Vector;

    fn sub(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.x - rhs.x,
            y: self.y - rhs.y,
            z: self.z - rhs.z,
        }
    }
}

impl Neg for &Vector {
    type Output = Vector;

    fn neg(self) -> Self::Output {
        Vector::new(-self.x, -self.y, -self.z)
    }
}

#[cfg(test)]
#[path = "tests/vector_tests.rs"]
mod tests;
