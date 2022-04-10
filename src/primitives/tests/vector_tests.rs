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
    let v2 = &Vector::new(x2,y2,z2);

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
    let point = &Point::new(x2,y2,z2);

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
    let vector2 = &Vector::new(x2,y2,z2);

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