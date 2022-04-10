use std::ops::{Add, Mul, Sub};

#[derive(Clone, Debug, PartialEq)]
pub struct RgbColor {
    red: f64,
    green: f64,
    blue: f64,
}

impl RgbColor {
    pub fn new(red: f64, green: f64, blue: f64) -> Self {
        RgbColor { red, green, blue }
    }

    pub fn get_red(&self) -> f64 {
        self.red
    }

    pub fn get_green(&self) -> f64 {
        self.green
    }

    pub fn get_blue(&self) -> f64 {
        self.blue
    }
}

impl Add for &RgbColor {
    type Output = RgbColor;

    fn add(self, rhs: Self) -> Self::Output {
        RgbColor::new(
            self.red + rhs.red,
            self.green + rhs.green,
            self.blue + rhs.blue,
        )
    }
}

impl Sub for &RgbColor {
    type Output = RgbColor;
    fn sub(self, rhs: Self) -> Self::Output {
        RgbColor::new(
            self.red - rhs.red,
            self.green - rhs.green,
            self.blue - rhs.blue,
        )
    }
}

impl Mul<f64> for &RgbColor {
    type Output = RgbColor;
    fn mul(self, rhs: f64) -> Self::Output {
        RgbColor::new(self.red * rhs, self.green * rhs, self.blue * rhs)
    }
}

impl Mul for &RgbColor {
    type Output = RgbColor;

    fn mul(self, rhs: Self) -> Self::Output {
        RgbColor::new(
            self.red * rhs.red,
            self.green * rhs.green,
            self.blue * rhs.blue,
        )
    }
}

#[cfg(test)]
#[path = "tests/rgb_color_tests.rs"]
mod tests;
