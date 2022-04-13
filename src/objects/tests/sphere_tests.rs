use crate::primitives::{HitRecord, Point, Ray, SquareMatrix, Vector};

use super::Sphere;

#[test]
fn get_hits_returns_two_hit_points() {
    let sphere = Sphere::identity();
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));

    let hits = sphere.get_hits(&ray);

    assert_eq!(2, hits.len());
    assert_eq!(hits[0], HitRecord::new(4.0, &sphere));
    assert_eq!(hits[1], HitRecord::new(6.0, &sphere));
}

#[test]
fn get_hits_at_tanget_returns_one_point() {
    let sphere = Sphere::identity();
    let ray = Ray::new(Point::new(0., 1., -5.), Vector::new(0., 0., 1.));

    let hits = sphere.get_hits(&ray);

    assert_eq!(1, hits.len());
    assert_eq!(hits[0], HitRecord::new(5.0, &sphere));
}

#[test]
fn get_hits_misses_sphere() {
    let sphere = Sphere::identity();
    let ray = Ray::new(Point::new(0., 1.25, -5.), Vector::new(0., 0., 1.));

    let hits = sphere.get_hits(&ray);

    assert_eq!(0, hits.len());
}

#[test]
fn get_hits_computes_on_transformed_sphere() {
    let sphere = Sphere::new(SquareMatrix::scaling(2., 2., 2.));
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));

    let hits = sphere.get_hits(&ray);

    assert_eq!(2, hits.len());
    assert_eq!(hits[0], HitRecord::new(3.0, &sphere));
    assert_eq!(hits[1], HitRecord::new(7.0, &sphere));
}
