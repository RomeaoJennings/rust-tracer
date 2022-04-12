use std::f64::consts::PI;

use crate::primitives::Point;

use super::Vector;

#[test]
fn new_correctly_assigns_values() {
    let expected_x = 1.23;
    let expected_y = -5.55;
    let expected_z = 16.;

    let point = Vector::new(expected_x, expected_y, expected_z);

    assert_eq!(expected_x, point.x);
    assert_eq!(expected_y, point.y);
    assert_eq!(expected_z, point.z);
}

#[test]
fn get_x_returns_correct_value() {
    let expected_x = 1.23;
    let expected_y = -5.55;
    let expected_z = 16.;

    let point = Vector::new(expected_x, expected_y, expected_z);

    assert_eq!(expected_x, point.get_x());
}

#[test]
fn get_y_returns_correct_value() {
    let expected_x = 1.23;
    let expected_y = -5.55;
    let expected_z = 16.;

    let point = Vector::new(expected_x, expected_y, expected_z);

    assert_eq!(expected_y, point.get_y());
}

#[test]
fn get_z_returns_correct_value() {
    let expected_x = 1.23;
    let expected_y = -5.55;
    let expected_z = 16.;

    let point = Vector::new(expected_x, expected_y, expected_z);

    assert_eq!(expected_z, point.get_z());
}

#[test]
fn adding_vectors_returns_summed_vector() {
    let x1 = 1.;
    let x2 = -2.;
    let y1 = 5.;
    let y2 = 16.;
    let z1 = -100.;
    let z2 = 54.;

    let v1 = &Vector::new(x1, y1, z1);
    let v2 = &Vector::new(x2, y2, z2);

    let result = v1 + v2;

    assert_eq!(x1 + x2, result.x);
    assert_eq!(y1 + y2, result.y);
    assert_eq!(z1 + z2, result.z);
}

#[test]
fn adding_vector_and_point_returns_summed_vector() {
    let x1 = 1.;
    let x2 = -2.;
    let y1 = 5.;
    let y2 = 16.;
    let z1 = -100.;
    let z2 = 54.;

    let vector = &Vector::new(x1, y1, z1);
    let point = &Point::new(x2, y2, z2);

    let result = vector + point;

    assert_eq!(x1 + x2, result.x);
    assert_eq!(y1 + y2, result.y);
    assert_eq!(z1 + z2, result.z);
}

#[test]
fn subtracting_two_vectors_yields_difference_vector() {
    let x1 = 1.;
    let x2 = -2.;
    let y1 = 5.;
    let y2 = 16.;
    let z1 = -100.;
    let z2 = 54.;

    let vector1 = &Vector::new(x1, y1, z1);
    let vector2 = &Vector::new(x2, y2, z2);

    let result = vector1 - vector2;

    assert_eq!(x1 - x2, result.get_x());
    assert_eq!(y1 - y2, result.get_y());
    assert_eq!(z1 - z2, result.get_z());
}

#[test]
fn negating_vector_yields_negative_vector() {
    let x = 1.;
    let y = -2.;
    let z = -200.;

    let vector = -&Vector::new(x, y, z);

    assert_eq!(-x, vector.x);
    assert_eq!(-y, vector.y);
    assert_eq!(-z, vector.z);
}

#[test]
fn multiplying_vector_by_scalar_yields_scaled_vector() {
    let x = 1.;
    let y = -2.;
    let z = 20.;

    let scalar = 5.;

    let vector = &Vector::new(x, y, z) * scalar;

    assert_eq!(x * scalar, vector.x);
    assert_eq!(y * scalar, vector.y);
    assert_eq!(z * scalar, vector.z)
}

#[test]
fn dividing_vector_by_scalar_yields_scaled_vector() {
    let x = 1.;
    let y = -2.;
    let z = 20.;

    let scalar = 2.;

    let vector = &Vector::new(x, y, z) / scalar;

    assert_eq!(x / scalar, vector.x);
    assert_eq!(y / scalar, vector.y);
    assert_eq!(z / scalar, vector.z)
}

#[test]
fn vector_magnitude_computes_correctly() {
    assert_eq!(1., Vector::new(1., 0., 0.).len());
    assert_eq!(1., Vector::new(0., 1., 0.).len());
    assert_eq!(1., Vector::new(0., 0., 1.).len());

    let actual = (5 * 5 + 3 * 3 + -2 * -2) as f64;
    assert_eq!(actual.sqrt(), Vector::new(5., 3., -2.).len())
}

#[test]
fn get_normal_returns_normalized_vector() {
    let x = 3.;
    let y = 4.;
    let z = 10.;

    let mut len: f64 = x * x + y * y + z * z;
    len = len.sqrt();

    let vector = Vector::new(x, y, z).get_normal();
    assert_eq!(x / len, vector.x);
    assert_eq!(y / len, vector.y);
    assert_eq!(z / len, vector.z);
}

#[test]
fn normalize_creates_normalized_vector() {
    let x = 3.;
    let y = 4.;
    let z = 10.;

    let mut len: f64 = x * x + y * y + z * z;
    len = len.sqrt();

    let mut vector = Vector::new(x, y, z);
    vector.normalize();
    assert_eq!(x / len, vector.x);
    assert_eq!(y / len, vector.y);
    assert_eq!(z / len, vector.z);
}

#[test]
fn dot_product_computes_correctly() {
    let v1 = &Vector::new(1., 2., 3.);
    let v2 = &Vector::new(2., 3., 4.);

    assert_eq!(20., v1 * v2);
}

#[test]
fn cross_product_computes_correctly() {
    let v1 = &Vector::new(1., 2., 3.);
    let v2 = &Vector::new(2., 3., 4.);

    let actual = Vector::new(-1., 2., -1.);

    assert_eq!(actual, v1 ^ v2);
}

#[test]
fn chained_transformations_compute_correctly() {
    let vector = Vector::new(1., 0., 1.);
    let result = vector
        .rotate_x(PI / 2.)
        .scale(5., 5., 5.)
        .translate(10., 5., 7.);
    assert_eq!(Vector::new(5., -5., 0.), result);
}
