use crate::primitives::{Point, RgbColor};

pub struct PointLight {
    location: Point,
    intensity: RgbColor,
}

impl PointLight {
    pub fn new(location: Point, intensity: RgbColor) -> Self {
        PointLight {
            location,
            intensity,
        }
    }

    pub fn get_location(&self) -> &Point {
        &self.location
    }

    pub fn get_intensity(&self) -> &RgbColor {
        &self.intensity
    }
}

#[cfg(test)]
#[path = "tests/point_light_tests.rs"]
mod tests;
