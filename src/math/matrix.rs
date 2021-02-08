//! Matrix representations and operations defined on them.

use std::{
    ops::{Index, IndexMut, Mul},
    slice,
};

use super::Coords;

/// A NxN, column-major matrix.
#[derive(Debug, Clone)]
pub struct MatrixN {
    data: Vec<f32>,
    order: usize,
}

impl MatrixN {
    /// Creates a matrix of order `n` filled with zeros.
    pub fn zeros(n: usize) -> Self {
        Self {
            data: vec![0.; n * n],
            order: n,
        }
    }

    /// Creates the identity matrix of order `n`.
    pub fn identity(n: usize) -> Self {
        let mut data = vec![0.; n * n];

        for i in 0..n {
            data[i * n + i] = 1.;
        }

        Self { data, order: n }
    }

    /// Creates a matrix of order `n` with its elements filled with the components provided
    /// by a slice in column-major order.
    ///
    /// # Panics
    ///
    /// Panics if `data.len() != n * n`.
    pub fn from_column_slice<D: AsRef<[f32]>>(n: usize, data: D) -> Self {
        let data = data.as_ref();
        assert_eq!(n * n, data.len());

        Self {
            data: Vec::from(data),
            order: n,
        }
    }

    /// Creates a matrix of order `n` with its elements filled with the components provided
    /// by a slice in row-major order.
    ///
    /// # Panics
    ///
    /// Panics if `data.len() != n * n`.
    pub fn from_row_slice<D: AsRef<[f32]>>(n: usize, data: D) -> Self {
        Self::from_column_slice(n, data).transpose()
    }

    /// Returns the order of this matrix, ie. the number of its rows/columns.
    pub fn order(&self) -> usize {
        self.order
    }

    /// Returns a reference to the element at position `(i,j)`, or `None` if the index is
    /// out-of-bounds.
    pub fn get(&self, (i, j): (usize, usize)) -> Option<&f32> {
        self.data.get(self.liner_index(i, j))
    }

    /// Returns a mutable reference to the element at position `(i,j)`, or `None` if the index is
    /// out-of-bounds.
    pub fn get_mut(&mut self, (i, j): (usize, usize)) -> Option<&mut f32> {
        let idx = self.liner_index(i, j);
        self.data.get_mut(idx)
    }

    /// Returns a reference to the element at position `(i,j)` without bound-checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior even if the resulting
    /// reference is not used.
    pub unsafe fn get_unchecked(&self, (i, j): (usize, usize)) -> &f32 {
        self.data.get_unchecked(self.liner_index(i, j))
    }

    /// Returns a mutable reference to the element at position `(i,j)` without bound-checking.
    ///
    /// # Safety
    ///
    /// Calling this method with an out-of-bounds index is undefined behavior even if the resulting
    /// reference is not used.
    pub unsafe fn get_unchecked_mut(&mut self, (i, j): (usize, usize)) -> &mut f32 {
        let idx = self.liner_index(i, j);
        self.data.get_unchecked_mut(idx)
    }

    /// Iterates through this matrix coordinates in column-major order.
    pub fn iter(&self) -> slice::Iter<f32> {
        self.data.iter()
    }

    /// Mutably iterates through this matrix coordinates in column-major order.
    pub fn iter_mut(&mut self) -> slice::IterMut<f32> {
        self.data.iter_mut()
    }

    /// Transposes `self`.
    pub fn transpose(&self) -> Self {
        let mut out = Self::zeros(self.order);

        for i in 0..self.order {
            for j in 0..self.order {
                unsafe {
                    *out.get_unchecked_mut((j, i)) = *self.get_unchecked((i, j));
                }
            }
        }

        out
    }

    /// Computes the determinant of the matrix.
    pub fn det(&self) -> f32 {
        if self.order() == 2 {
            self[(0, 0)] * self[(1, 1)] - self[(0, 1)] * self[(1, 0)]
        } else {
            (0..self.order()).fold(0., |det, i| det + self[(0, i)] * self.cofactor(0, i))
        }
    }

    /// Returns the matrix of order `n-1` obtain by removing `row` and `col` from `self`.
    pub fn submatrix(&self, row: usize, col: usize) -> MatrixN {
        let mut out = MatrixN::zeros(self.order() - 1);
        let mut iter = out.iter_mut();

        for j in 0..self.order() {
            for i in 0..self.order() {
                if i != row && j != col {
                    *iter.next().unwrap() = self[(i, j)];
                }
            }
        }

        out
    }

    /// Computes the minor of element `(i,j)`, ie. the determinant of the submatrix `(i,j)`.
    pub fn minor(&self, i: usize, j: usize) -> f32 {
        self.submatrix(i, j).det()
    }

    /// Computes the cofactor of element `(i,j)`, ie. the possibly negated minor of `(i,j)`.
    pub fn cofactor(&self, i: usize, j: usize) -> f32 {
        let minor = self.minor(i, j);

        if (i + j) % 2 == 0 {
            minor
        } else {
            -minor
        }
    }

    /// Returns true if the two matrix have the same order and the absolute difference of all
    /// corresponding elements between `self` and `other` is less than or equal to `max_abs_diff`.
    pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        self.order == other.order
            && self
                .iter()
                .zip(other.iter())
                .all(|(a, b)| (a - b).abs() <= max_abs_diff)
    }

    /// Returns the linear index in the matrix storage corresponding to element `(irow,icol)`.
    fn liner_index(&self, irow: usize, icol: usize) -> usize {
        icol * self.order + irow
    }
}

impl Index<(usize, usize)> for MatrixN {
    type Output = f32;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        self.data.index(self.liner_index(i, j))
    }
}

impl IndexMut<(usize, usize)> for MatrixN {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        self.data.index_mut(self.liner_index(i, j))
    }
}

impl Mul for MatrixN {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&MatrixN> for MatrixN {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        &self * rhs
    }
}

impl Mul<MatrixN> for &MatrixN {
    type Output = MatrixN;

    fn mul(self, rhs: MatrixN) -> Self::Output {
        self * &rhs
    }
}

impl<'a, 'b> Mul<&'b MatrixN> for &'a MatrixN {
    type Output = MatrixN;

    fn mul(self, rhs: &'b MatrixN) -> Self::Output {
        assert_eq!(rhs.order, self.order);

        let mut out = Self::Output::zeros(self.order);

        for i in 0..out.order {
            for j in 0..out.order {
                unsafe {
                    *out.get_unchecked_mut((i, j)) = (0..out.order).fold(0., |sum, idx| {
                        sum + self.get_unchecked((i, idx)) * rhs.get_unchecked((idx, j))
                    });
                }
            }
        }

        out
    }
}

impl Mul<Coords> for MatrixN {
    type Output = Coords;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Coords) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Coords> for &MatrixN {
    type Output = Coords;

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: Coords) -> Self::Output {
        let data = <[f32; 4]>::from(rhs);

        Self::Output {
            x: (0..4).fold(0., |sum, i| sum + self[(0, i)] * data[i]),
            y: (0..4).fold(0., |sum, i| sum + self[(1, i)] * data[i]),
            z: (0..4).fold(0., |sum, i| sum + self[(2, i)] * data[i]),
            w: (0..4).fold(0., |sum, i| sum + self[(3, i)] * data[i]),
        }
    }
}
