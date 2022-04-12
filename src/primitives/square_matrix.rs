use std::ops::Mul;

use super::{Point, Vector};

#[derive(Debug, Clone)]
pub struct SquareMatrix {
    elements: Vec<Vec<f64>>,
}

impl SquareMatrix {
    pub fn new(elements: Vec<Vec<f64>>) -> Self {
        SquareMatrix { elements }
    }
    pub fn zeroes(size: usize) -> Self {
        let elements = vec![vec![0.0; size]; size];
        SquareMatrix { elements }
    }

    pub fn identity(size: usize) -> Self {
        let mut result = SquareMatrix::zeroes(size);
        for i in 0..size {
            result.set(i, i, 1.0);
        }
        result
    }

    pub fn transpose(&self) -> Self {
        let len = self.elements.len();
        let mut result = SquareMatrix::zeroes(len);
        for row in 0..len {
            for col in 0..len {
                result.set(row, col, self.elements[col][row]);
            }
        }
        result
    }

    pub fn get(&self, row: usize, col: usize) -> f64 {
        self.elements[row][col]
    }

    pub fn set(&mut self, row: usize, col: usize, value: f64) {
        self.elements[row][col] = value;
    }

    pub fn determinant(&self) -> f64 {
        if self.elements.len() == 2 {
            return self.elements[0][0] * self.elements[1][1] - self.elements[1][0] * self.elements[0][1]
        } 
        let mut result = 0.0;
        for i in 0..self.elements.len() {
            result += self.elements[0][i] * self.cofactor(0, i);
        }
        result
    }

    pub fn is_invertable(&self) -> bool {
        self.determinant() != 0.
    }

    pub fn invert(&self) -> Self {
        let determinant = self.determinant();
        let mut result = SquareMatrix::zeroes(self.elements.len());
        for row in 0..self.elements.len() {
            for col in 0..self.elements.len() {
                result.set(col, row, self.cofactor(row, col) / determinant)
            }
        }
        result
    }

    fn submatrix(&self, row: usize, col: usize) -> Self {
        let mut rows = Vec::with_capacity(self.elements.len() - 1);
        for curr_row in 0..self.elements.len() {
            if row == curr_row {
                continue;
            }
            let mut row_vec = Vec::with_capacity(self.elements.len() - 1);
            for curr_col in 0..self.elements.len() {
                if col == curr_col { continue;}
                row_vec.push(self.elements[curr_row][curr_col]);
            }
            rows.push(row_vec)
        }
        SquareMatrix { elements: rows }
    }

    fn minor(&self, row: usize, col: usize) -> f64 {
        self.submatrix(row, col).determinant()
    }

    fn cofactor(&self, row: usize, col: usize) -> f64 {
        let mut minor = self.minor(row, col);
        if (row + col) % 2  == 1 {
            minor *= -1.;
        }
        minor
    }
}

impl Mul<&Point> for &SquareMatrix {
    type Output = Point;

    fn mul(self, rhs: &Point) -> Self::Output {
        let len = self.elements.len();
        let mut data = [0.0; 4];
        for row in 0..len {
            for col in 0..len {
                data[row] += self.elements[row][col] * rhs[col];
            }
        }
        Point::new(data[0], data[1], data[2])
    }
}

impl Mul<&Vector> for &SquareMatrix {
    type Output = Vector;

    fn mul(self, rhs: &Vector) -> Self::Output {
        let len = self.elements.len();
        let mut data = [0.0; 4];
        for row in 0..len {
            for col in 0..len {
                data[row] += self.elements[row][col] * rhs[col];
            }
        }
        Vector::new(data[0], data[1], data[2])
    }
}

impl PartialEq for SquareMatrix {
    fn eq(&self, other: &Self) -> bool {
        if self.elements.len() != other.elements.len()
            || self.elements[0].len() != other.elements[0].len()
        {
            return false;
        }
        for (self_row, other_row) in self.elements.iter().zip(other.elements.iter()) {
            for (self_item, other_item) in self_row.iter().zip(other_row.iter()) {
                if !approx_equal(*self_item, *other_item) {
                    return false;
                }
            }
        }
        true
    }
}

fn approx_equal(one: f64, two: f64) -> bool {
    const EPSILON: f64 = 1e-6;
    (one - two).abs() < EPSILON
}

impl Mul for &SquareMatrix {
    type Output = SquareMatrix;

    fn mul(self, rhs: Self) -> Self::Output {
        let width = self.elements.len();
        let mut result = SquareMatrix::zeroes(width);

        for row in 0..width {
            for col in 0..width {
                let mut value = 0.0;
                for index in 0..width {
                    value += self.elements[row][index] * rhs.elements[index][col];
                }
                result.set(row, col, value)
            }
        }
        result
    }
}

#[cfg(test)]
#[path = "tests/square_matrix_tests.rs"]
mod tests;
