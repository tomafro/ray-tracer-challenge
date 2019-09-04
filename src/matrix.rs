use crate::{float_eq, translation, Float, Tuple};
use std::ops::{Index, Mul};
use std::fmt;

#[derive(Clone, Copy, Default)]

pub struct Matrix {
    size: usize,
    values: [[Float; 4]; 4],
}

impl Matrix {
    fn build(size: usize, v: [Float; 4 * 4]) -> Matrix {
        let mut values = [[0.0; 4]; 4];
        for r in 0..size {
            for c in 0..size {
                values[r][c] = v[(r * size) + c];
            }
        }
        Matrix { size, values }
    }

    fn at(&self, row: usize, col: usize) -> Float {
        self.values[row][col]
    }

    pub fn transpose(&self) -> Self {
        let mut values = self.values.clone();

        for r in 1..self.size {
            for c in 0..r {
                values[r][c] = self.values[c][r];
                values[c][r] = self.values[r][c];
            }
        }

        Self {
            size: self.size,
            values,
        }
    }

    pub fn submatrix(&self, row: usize, col: usize) -> Self {
        let mut values = [[0.0; 4]; 4];
        let size = self.size - 1;

        for r in 0..size {
            for c in 0..size {
                let rx = match r < row {
                    true => r,
                    false => r + 1
                };

                let cx = match c < col {
                    true => c,
                    false => c + 1
                };

                values[r][c] = self.values[rx][cx];
            }
        }

        Self {
            size: self.size - 1,
            values,
        }
    }

    pub fn determinant(&self) -> Float {
        match self.size {
            2 => self.at(0, 0) * self.at(1, 1) - self.at(0, 1) * self.at(1, 0),
            _ => (0..self.size).fold(0.0, |result, c| {
                result + self.at(0, c) * self.cofactor(0, c)
            }),
        }
    }

    pub fn cofactor(&self, row: usize, col: usize) -> Float {
        let minor = self.minor(row, col);

        match (row + col) % 2 {
            0 => minor,
            _ => -minor,
        }
    }

    pub fn minor(&self, r: usize, c: usize) -> Float {
        self.submatrix(r, c).determinant()
    }

    pub fn is_invertible(&self) -> bool {
        self.determinant() != 0.0
    }


    pub fn inverse(&self) -> Matrix {
        let determinant = self.determinant();
        let mut values = [[0.0; 4]; 4];
        for r in 0..self.size {
            for c in 0..self.size {
                values[c][r] = self.cofactor(r, c) / determinant;
            }
        }
        Self {
            size: self.size,
            values,
        }
    }
}

impl fmt::Debug for Matrix {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        let mut result = String::new();

        result.push_str("Matrix (");

        let mut rows: Vec<String> = vec![];
        for r in 0..self.size {
            let mut row = String::new();
            row.push_str("(");
            let row_values: Vec<_> = self.values[r][0..self.size].iter().map(|v| format!("{:.1}", v)).collect();
            row.push_str(&row_values.join(", "));
            row.push_str(")");
            rows.push(row);
        }
        result.push_str(&rows.join(", "));
        result.push_str(")");
        write!(f, "{}", result)
    }
}

impl PartialEq for Matrix {
    fn eq(&self, other: &Self) -> bool {
        self.size == other.size &&
        self.values.iter().enumerate().fold(true, |result, (rx, row)| {
            result && row.iter().enumerate().fold(true, |result, (cx, value)| {
                result && float_eq(other.values[rx][cx], *value)
            })
        })
    }
}

impl Index<usize> for Matrix {
    type Output = [Float];

    fn index(&self, r: usize) -> &[Float] {
        // let start = self.size * r;
        // &self.values[start..(start + self.size)]
        &self.values[r][0..self.size]
    }
}

impl<'a> Mul<&'a Matrix> for Matrix {
    type Output = Self;

    fn mul(self, other: &Self) -> Self {
        let mut values = self.values.clone();

        for r in 0..self.size {
            for c in 0..self.size {
                values[r][c] =
                    (0..self.size).fold(0.0, |t, i| t + self.at(r, i) * other.at(i, c));
            }
        }

        Matrix {
            size: self.size,
            values,
        }
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        self * &other
    }
}

impl Mul<Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: Tuple) -> Tuple {
        self * &other
    }
}

impl<'a> Mul<&'a Tuple> for Matrix {
    type Output = Tuple;

    fn mul(self, other: &Tuple) -> Tuple {
        let mut values = [0.0; 4];
        let Tuple { x, y, z, w } = other;

        for n in 0..self.size {
            values[n] =
                self.at(n, 0) * x + self.at(n, 1) * y + self.at(n, 2) * z + self.at(n, 3) * w;
        }

        Tuple {
            x: values[0],
            y: values[1],
            z: values[2],
            w: values[3],
        }
    }
}

impl From<[[Float; 4]; 4]> for Matrix {
    fn from(t: [[Float; 4]; 4]) -> Matrix {
        let mut values = [0.0; 16];
        for r in 0..4 {
            for c in 0..4 {
                values[r * 4 + c] = t[r][c]
            }
        }
        Matrix::build(4, values)
    }
}

impl From<[[Float; 3]; 3]> for Matrix {
    fn from(t: [[Float; 3]; 3]) -> Matrix {
        let mut values = [0.0; 16];
        for r in 0..3 {
            for c in 0..3 {
                values[r * 3 + c] = t[r][c]
            }
        }
        Matrix::build(3, values)
    }
}

impl From<[[Float; 2]; 2]> for Matrix {
    fn from(t: [[Float; 2]; 2]) -> Matrix {
        let mut values = [0.0; 16];
        for r in 0..2 {
            for c in 0..2 {
                values[r * 2 + c] = t[r][c]
            }
        }
        Matrix::build(2, values)
    }
}

pub fn matrix<T: Into<Matrix>>(v: T) -> Matrix {
    v.into()
}

pub fn view_transform(from: Tuple, to: Tuple, up: Tuple) -> Matrix {
    let forward = (to - from).normalize();
    let left = forward.cross(up.normalize());
    let true_up = left.cross(forward);

    let orientation = matrix([
        [left.x, left.y, left.z, 0.0],
        [true_up.x, true_up.y, true_up.z, 0.0],
        [-forward.x, -forward.y, -forward.z, 0.0],
        [0.0, 0.0, 0.0, 1.0],
    ]);

    orientation * translation(-from.x, -from.y, -from.z)
}
