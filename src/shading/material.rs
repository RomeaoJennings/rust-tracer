use crate::primitives::RgbColor;

#[derive(Clone, Debug, PartialEq)]
pub struct Material {
    color: RgbColor,
    ambient: f64,
    diffuse: f64,
    specular: f64,
    shininess: f64,
}

impl Material {
    pub fn get_color(&self) -> &RgbColor {
        &self.color
    }

    pub fn set_color(&mut self, color: RgbColor) {
        self.color = color;
    }

    pub fn get_ambient(&self) -> f64 {
        self.ambient
    }

    pub fn set_ambient(&mut self, ambient: f64) {
        self.ambient = ambient;
    }

    pub fn get_diffuse(&self) -> f64 {
        self.diffuse
    }

    pub fn set_diffuse(&mut self, diffuse: f64) {
        self.diffuse = diffuse
    }
    pub fn get_specular(&self) -> f64 {
        self.specular
    }

    pub fn set_specular(&mut self, specular: f64) {
        self.specular = specular
    }

    pub fn get_shininess(&self) -> f64 {
        self.shininess
    }

    pub fn set_shininess(&mut self, shininess: f64) {
        self.shininess = shininess;
    }
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: RgbColor::new(1.0, 1.0, 1.0),
            ambient: 0.2,
            diffuse: 0.85,
            specular: 0.8,
            shininess: 300.0,
        }
    }
}

#[cfg(test)]
#[path = "tests/material_tests.rs"]
mod tests;
