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