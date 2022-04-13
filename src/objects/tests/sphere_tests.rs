use crate::primitives::{Ray, Point, Vector};

use super::Sphere;

#[test]
fn get_hits_returns_two_hit_points() {
    let sphere = Sphere;
    let ray = Ray::new(Point::new(0.,0.,-5.), Vector::new(0.,0.,1.));

    let hits = sphere.get_hits(&ray);

    assert_eq!(2, hits.len());
    assert_eq!(hits[0], 4.0);
    assert_eq!(hits[1], 6.0);
}

#[test] 
fn get_hits_at_tanget_returns_one_point() {
    let sphere = Sphere;
    let ray = Ray::new(Point::new(0.,1.,-5.), Vector::new(0.,0.,1.));

    let hits = sphere.get_hits(&ray);

    assert_eq!(1, hits.len());
    assert_eq!(hits[0], 5.0);
}

#[test] 
fn get_hits_misses_sphere() {
    let sphere = Sphere;
    let ray = Ray::new(Point::new(0.,1.25,-5.), Vector::new(0.,0.,1.));

    let hits = sphere.get_hits(&ray);

    assert_eq!(0, hits.len());
}