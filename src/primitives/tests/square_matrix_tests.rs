use std::f64::consts::PI;

use crate::primitives::{Point, Vector};

use super::SquareMatrix;

#[test]
fn get_works_correctly() {
    let data = vec![
        vec![1., 2., 3., 4.],
        vec![5.5, 6.5, 7.5, 8.5],
        vec![9., 10., 11., 12.],
        vec![13.5, 14.5, 15.5, 16.5],
    ];

    let matrix = SquareMatrix::new(data.clone());

    for row in 0..data.len() {
        for col in 0..data[0].len() {
            assert_eq!(data[row][col], matrix.get(row, col));
        }
    }
}

#[test]
fn matrix_multiplication_computes_correctly() {
    let matrix1 = vec![
        vec![1., 2., 3., 4.],
        vec![5., 6., 7., 8.],
        vec![9., 8., 7., 6.],
        vec![5., 4., 3., 2.],
    ];
    let matrix2 = vec![
        vec![-2., 1., 2., 3.],
        vec![3., 2., 1., -1.],
        vec![4., 3., 6., 5.],
        vec![1., 2., 7., 8.],
    ];
    let result = vec![
        vec![20., 22., 50., 48.],
        vec![44., 54., 114., 108.],
        vec![40., 58., 110., 102.],
        vec![16., 26., 46., 42.],
    ];

    let matrix1 = &SquareMatrix::new(matrix1);
    let matrix2 = &SquareMatrix::new(matrix2);
    let result = SquareMatrix::new(result);

    assert_eq!(matrix1 * matrix2, result);
}

#[test]
fn multiplying_matrix_with_point_computes_correctly() {
    let data = vec![
        vec![1., 2., 3., 4.],
        vec![2., 4., 4., 2.],
        vec![8., 6., 4., 1.],
        vec![0., 0., 0., 1.],
    ];
    let point = &Point::new(1., 2., 3.);
    let matrix = &SquareMatrix::new(data);
    let result = Point::new(18., 24., 33.);

    assert_eq!(result, matrix * point);
}

#[test]
fn multiplying_matrix_with_vector_computes_correctly() {
    let data = vec![
        vec![1., 2., 3., 4.],
        vec![2., 4., 4., 2.],
        vec![8., 6., 4., 1.],
        vec![0., 0., 0., 1.],
    ];
    let point = &Vector::new(1., 2., 3.);
    let matrix = &SquareMatrix::new(data);
    let result = Vector::new(14., 22., 32.);

    assert_eq!(result, matrix * point);
}

#[test]
fn determinant_calculates_correctly_for3x3() {
    let data = vec![vec![1., 2., 6.], vec![-5., 8., -4.], vec![2., 6., 4.]];
    let matrix = SquareMatrix::new(data);

    assert_eq!(-196., matrix.determinant());
}

#[test]
fn determinant_calculates_correctly_for4x4() {
    let data = vec![
        vec![-2., -8., 3., 5.],
        vec![-3., 1., 7., 3.],
        vec![1., 2., -9., 6.],
        vec![-6., 7., 7., -9.],
    ];

    let matrix = SquareMatrix::new(data);
    assert_eq!(-4071., matrix.determinant());
}

#[test]
fn matrix_times_inverse_equals_identity() {
    let data = vec![
        vec![-2., -8., 3., 5.],
        vec![-3., 1., 7., 3.],
        vec![1., 2., -9., 6.],
        vec![-6., 7., 7., -9.],
    ];

    let matrix = SquareMatrix::new(data);
    let inverse = matrix.invert();
    let identity = &matrix * &inverse;
    assert_eq!(identity, SquareMatrix::identity(4));
}

#[test]
fn translation_correctly_translates_point() {
    let point = Point::new(0., 0., 0.);

    let x = 1.5;
    let y = -2.;
    let z = 5.;

    let translate_matrix = SquareMatrix::translation(x, y, z);
    let translated_point = &translate_matrix * &point;
    let expected = Point::new(x, y, z);
    assert_eq!(expected, translated_point)
}

#[test]
fn translation_does_not_affect_vectors() {
    let vector = Vector::new(0., 0., 0.);

    let x = 1.5;
    let y = -2.;
    let z = 5.;

    let translate_matrix = SquareMatrix::translation(x, y, z);
    let translated_vector = &translate_matrix * &vector;
    assert_eq!(vector, translated_vector)
}

#[test]
fn rotation_x_computes_correctly() {
    let point = &Point::new(0.0, 1.0, 0.0);
    let one_eighth = &SquareMatrix::rotation_x(PI / 4.0);
    let one_fourth = &SquareMatrix::rotation_x(PI / 2.0);

    assert_eq!(
        one_eighth * point,
        Point::new(0., 2f64.sqrt() / 2.0, 2f64.sqrt() / 2.)
    );
    assert_eq!(one_fourth * point, Point::new(0., 0., 1.));
}

#[test]
fn rotation_y_computes_correctly() {
    let point = &Point::new(0., 0., 1.);

    let one_eighth = &SquareMatrix::rotation_y(PI / 4.0);
    let one_fourth = &SquareMatrix::rotation_y(PI / 2.0);

    assert_eq!(
        one_eighth * point,
        Point::new(2f64.sqrt() / 2.0, 0.0, 2f64.sqrt() / 2.)
    );
    assert_eq!(one_fourth * point, Point::new(1., 0., 0.));
}

#[test]
fn rotation_z_computes_correctly() {
    let point = &Point::new(0.0, 1.0, 0.0);
    let one_eighth = &SquareMatrix::rotation_z(PI / 4.0);
    let one_fourth = &SquareMatrix::rotation_z(PI / 2.0);

    assert_eq!(
        one_eighth * point,
        Point::new(-2f64.sqrt() / 2.0, 2f64.sqrt() / 2., 0.0)
    );
    assert_eq!(one_fourth * point, Point::new(-1., 0., 0.));
}

#[test]
fn shearing_computes_correctly_x_in_proportion_to_y() {
    let point = &Point::new(2., 3., 4.);
    let matrix = &SquareMatrix::shearing(1., 0., 0., 0., 0., 0.);

    assert_eq!(matrix * point, Point::new(5., 3., 4.));
}

#[test]
fn shearing_computes_correctly_x_in_proportion_to_z() {
    let point = &Point::new(2., 3., 4.);
    let matrix = &SquareMatrix::shearing(0., 1., 0., 0., 0., 0.);

    assert_eq!(matrix * point, Point::new(6., 3., 4.));
}

#[test]
fn shearing_computes_correctly_y_in_proportion_to_x() {
    let point = &Point::new(2., 3., 4.);
    let matrix = &SquareMatrix::shearing(0., 0., 1., 0., 0., 0.);

    assert_eq!(matrix * point, Point::new(2., 5., 4.));
}

#[test]
fn shearing_computes_correctly_y_in_proportion_to_z() {
    let point = &Point::new(2., 3., 4.);
    let matrix = &SquareMatrix::shearing(0., 0., 0., 1., 0., 0.);

    assert_eq!(matrix * point, Point::new(2., 7., 4.));
}

#[test]
fn shearing_computes_correctly_z_in_proportion_to_x() {
    let point = &Point::new(2., 3., 4.);
    let matrix = &SquareMatrix::shearing(0., 0., 0., 0., 1., 0.);

    assert_eq!(matrix * point, Point::new(2., 3., 6.));
}

#[test]
fn shearing_computes_correctly_z_in_proportion_to_y() {
    let point = &Point::new(2., 3., 4.);
    let matrix = &SquareMatrix::shearing(0., 0., 0., 0., 0., 1.);

    assert_eq!(matrix * point, Point::new(2., 3., 7.));
}

#[test]
fn scaling_computes_correctly_for_point() {
    let point = &Point::new(-4., 6., 8.);
    let matrix = &SquareMatrix::scaling(2., 3., 4.);
    assert_eq!(matrix * point, Point::new(-8., 18., 32.));
}

#[test]
fn scaling_computes_correctly_for_vector() {
    let point = &Vector::new(-4., 6., 8.);
    let matrix = &SquareMatrix::scaling(2., 3., 4.);
    assert_eq!(matrix * point, Vector::new(-8., 18., 32.));
}
