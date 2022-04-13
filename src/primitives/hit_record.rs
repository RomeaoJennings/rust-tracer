use crate::objects::Sphere;

#[derive(Debug, Clone, PartialEq)]
pub struct HitRecord<'a> {
    t: f64,
    object: &'a Sphere,
}

impl<'a> HitRecord<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Self {
        HitRecord { t, object }
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }

    pub fn get_object(&self) -> &Sphere {
        self.object
    }
}
