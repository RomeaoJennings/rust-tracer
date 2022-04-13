use std::f64::consts::PI;

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
    let sphere = Sphere::with_transform(SquareMatrix::scaling(2., 2., 2.));
    let ray = Ray::new(Point::new(0., 0., -5.), Vector::new(0., 0., 1.));

    let hits = sphere.get_hits(&ray);

    assert_eq!(2, hits.len());
    assert_eq!(hits[0], HitRecord::new(3.0, &sphere));
    assert_eq!(hits[1], HitRecord::new(7.0, &sphere));
}

#[test]
fn normal_on_x_axis_computes_correctly() {
    let sphere = Sphere::identity();
    let point = Point::new(1.0, 0.0, 0.0);
    let normal = sphere.get_normal(&point);

    assert_eq!(normal, Vector::new(1.0, 0.0, 0.0));
}

#[test]
fn normal_on_y_axis_computes_correctly() {
    let sphere = Sphere::identity();
    let point = Point::new(0.0, 1.0, 0.0);
    let normal = sphere.get_normal(&point);

    assert_eq!(normal, Vector::new(0.0, 1.0, 0.0));
}

#[test]
fn normal_on_z_axis_computes_correctly() {
    let sphere = Sphere::identity();
    let point = Point::new(0.0, 0.0, 1.0);
    let normal = sphere.get_normal(&point);

    assert_eq!(normal, Vector::new(0.0, 0.0, 1.0));
}

#[test]
fn normal_on_arbitrary_point_computes_correctly() {
    let sphere = Sphere::identity();
    let v = 3.0f64.sqrt() / 3.0;
    let point = Point::new(v, v, v);
    let normal = sphere.get_normal(&point);

    assert_eq!(normal, Vector::new(v, v, v));
}

#[test]
fn normal_on_translated_sphere_computes_correctly() {
    let sphere = Sphere::with_transform(SquareMatrix::translation(0.0, 1.0, 0.0));
    let normal = sphere.get_normal(&Point::new(0.0, 1.70711, -0.70711));

    assert_eq!(normal, Vector::new(0.0, 0.7071067, -0.7071067));
}

#[test]
fn normal_on_transformed_sphere_computes_correctly() {
    let transform = &SquareMatrix::scaling(1.0, 0.5, 1.0) * &SquareMatrix::rotation_z(PI / 5.0);
    let sphere = Sphere::with_transform(transform);
    let sqrt2over2 = 2.0f64.sqrt() / 2.0;
    let normal = sphere.get_normal(&Point::new(0.0, sqrt2over2, -sqrt2over2));
    assert_eq!(normal, Vector::new(0.0, 0.9701425, -0.242535));
}

#[test]
fn normal_on_transformed_sphere_is_normalized() {}
