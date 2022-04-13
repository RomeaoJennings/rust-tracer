use std::ops::{Add, BitXor, Div, Index, Mul, Neg, Sub};

use super::{Point, SquareMatrix};

const W: f64 = 0.0;

#[derive(Debug, Clone)]
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

    pub fn len(&self) -> f64 {
        (self.x * self.x + self.y * self.y + self.z * self.z).sqrt()
    }

    pub fn get_normal(&self) -> Self {
        self / self.len()
    }

    pub fn normalize(&mut self) {
        let len = self.len();
        self.x /= len;
        self.y /= len;
        self.z /= len;
    }

    pub fn reflect(&self, normal: &Vector) -> Vector {
        let right_side = normal * (self * normal * 2.0);
        self - &right_side
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

impl Mul<f64> for &Vector {
    type Output = Vector;

    fn mul(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x * rhs,
            y: self.y * rhs,
            z: self.z * rhs,
        }
    }
}

impl Div<f64> for &Vector {
    type Output = Vector;

    fn div(self, rhs: f64) -> Self::Output {
        Vector {
            x: self.x / rhs,
            y: self.y / rhs,
            z: self.z / rhs,
        }
    }
}

impl Mul for &Vector {
    type Output = f64;

    fn mul(self, rhs: Self) -> Self::Output {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }
}

impl BitXor for &Vector {
    type Output = Vector;

    fn bitxor(self, rhs: Self) -> Self::Output {
        Vector {
            x: self.y * rhs.z - self.z * rhs.y,
            y: self.z * rhs.x - self.x * rhs.z,
            z: self.x * rhs.y - self.y * rhs.x,
        }
    }
}

impl Index<usize> for Vector {
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

impl PartialEq for Vector {
    fn eq(&self, other: &Self) -> bool {
        approx_eq(self.x, other.x) && approx_eq(self.y, other.y) && approx_eq(self.z, other.z)
    }
}

fn approx_eq(one: f64, two: f64) -> bool {
    const EPSILON: f64 = 1e-6;
    (one - two).abs() < EPSILON
}

#[cfg(test)]
#[path = "tests/vector_tests.rs"]
mod tests;
