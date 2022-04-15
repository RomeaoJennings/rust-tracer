use std::cmp::Ordering;

use crate::objects::Sphere;

#[derive(Debug, Clone, PartialEq)]
pub struct HitIntersection<'a> {
    t: f64,
    object: &'a Sphere,
}

impl<'a> HitIntersection<'a> {
    pub fn new(t: f64, object: &'a Sphere) -> Self {
        HitIntersection { t, object }
    }

    pub fn get_t(&self) -> f64 {
        self.t
    }

    pub fn get_object(&self) -> &Sphere {
        self.object
    }

    pub fn cmp_ignore_nan(&self, other: &Self) -> Ordering {
        if self.t == f64::NAN && other.t == f64::NAN {
            Ordering::Equal
        } else if self.t == f64::NAN {
            Ordering::Greater
        } else if other.t == f64::NAN {
            Ordering::Less
        }else {
            self.partial_cmp(&other).unwrap()
        }
    }
}

impl<'a> PartialOrd for HitIntersection<'a> {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.t.partial_cmp(&other.t)
    }
}