use crate::{
    objects::Hittable,
    primitives::{HitRecord, Point, RgbColor},
};

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

    pub fn shade(&self, record: &HitRecord) -> RgbColor {
        let material = record.get_object().get_material();
        let effective_color = material.get_color() * &self.intensity;
        let lightv = (&self.location - record.get_hit_point()).get_normal();
        let ambient = &effective_color * material.get_ambient();
        let light_dot_normal = &lightv * record.get_normal();
        let diffuse: RgbColor;
        let specular: RgbColor;
        if light_dot_normal < 0. {
            diffuse = RgbColor::new(0., 0., 0.);
            specular = RgbColor::new(0., 0., 0.);
        } else {
            diffuse = &effective_color * (material.get_diffuse() * light_dot_normal);
            let reflectv = (-&lightv).reflect(record.get_normal());
            let reflect_dot_eye = &reflectv * record.get_eye_vector();
            if reflect_dot_eye <= 0. {
                specular = RgbColor::new(0., 0., 0.);
            } else {
                let factor = reflect_dot_eye.powf(material.get_shininess());
                specular = &self.intensity * (material.get_specular() * factor);
            }
        }

        &(&ambient + &diffuse) + &specular
    }
}

#[cfg(test)]
#[path = "tests/point_light_tests.rs"]
mod tests;
