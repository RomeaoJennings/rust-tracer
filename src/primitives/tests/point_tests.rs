use crate::primitives::Vector;

use super::Point;

#[test]
fn new_correctly_assigns_values() {
    let expected_x = 1.23;
    let expected_y = -5.55;
    let expected_z = 16.;

    let point = Point::new(expected_x, expected_y, expected_z);

    assert_eq!(expected_x, point.x);
    assert_eq!(expected_y, point.y);
    assert_eq!(expected_z, point.z);
}

#[test]
fn get_x_returns_correct_value() {
    let expected_x = 1.23;
    let expected_y = -5.55;
    let expected_z = 16.;

    let point = Point::new(expected_x, expected_y, expected_z);

    assert_eq!(expected_x, point.get_x());
}

#[test]
fn get_y_returns_correct_value() {
    let expected_x = 1.23;
    let expected_y = -5.55;
    let expected_z = 16.;

    let point = Point::new(expected_x, expected_y, expected_z);

    assert_eq!(expected_y, point.get_y());
}

#[test]
fn get_z_returns_correct_value() {
    let expected_x = 1.23;
    let expected_y = -5.55;
    let expected_z = 16.;

    let point = Point::new(expected_x, expected_y, expected_z);

    assert_eq!(expected_z, point.get_z());
}

#[test]
fn adding_point_and_vector_returns_summed_vector() {
    let x1 = 1.;
    let x2 = -2.;
    let y1 = 5.;
    let y2 = 16.;
    let z1 = -100.;
    let z2 = 54.;

    let point = &Point::new(x2, y2, z2);
    let vector = &Vector::new(x1, y1, z1);

    let result = vector + point;

    assert_eq!(x1 + x2, result.get_x());
    assert_eq!(y1 + y2, result.get_y());
    assert_eq!(z1 + z2, result.get_z());
}

#[test]
fn subtracting_two_points_yields_difference_vector() {
    let x1 = 1.;
    let x2 = -2.;
    let y1 = 5.;
    let y2 = 16.;
    let z1 = -100.;
    let z2 = 54.;

    let point1 = &Point::new(x1, y1, z1);
    let point2 = &Point::new(x2, y2, z2);

    let result = point1 - point2;

    assert_eq!(x1 - x2, result.get_x());
    assert_eq!(y1 - y2, result.get_y());
    assert_eq!(z1 - z2, result.get_z());
}

#[test]
fn subtracting_vector_from_point_yields_difference_point() {
    let x1 = 1.;
    let x2 = -2.;
    let y1 = 5.;
    let y2 = 16.;
    let z1 = -100.;
    let z2 = 54.;

    let point = &Point::new(x1, y1, z1);
    let vector = &Vector::new(x2, y2, z2);

    let result = point - vector;

    assert_eq!(x1 - x2, result.get_x());
    assert_eq!(y1 - y2, result.get_y());
    assert_eq!(z1 - z2, result.get_z());
}

#[test]
fn negating_point_yields_negative_point() {
    let x = 1.;
    let y = -2.;
    let z = -200.;

    let point = -&Point::new(x, y, z);

    assert_eq!(-x, point.x);
    assert_eq!(-y, point.y);
    assert_eq!(-z, point.z);
}

#[test]
fn multiplying_point_by_scalar_yields_scaled_point() {
    let x = 1.;
    let y = -2.;
    let z = 20.;

    let scalar = 5.;

    let point = &Point::new(x, y, z) * scalar;

    assert_eq!(x * scalar, point.x);
    assert_eq!(y * scalar, point.y);
    assert_eq!(z * scalar, point.z)
}

#[test]
fn dividing_point_by_scalar_yields_scaled_point() {
    let x = 1.;
    let y = -2.;
    let z = 20.;

    let scalar = 2.;

    let point = &Point::new(x, y, z) / scalar;

    assert_eq!(x / scalar, point.x);
    assert_eq!(y / scalar, point.y);
    assert_eq!(z / scalar, point.z)
}