//! Matrix representations and operations defined on them.

use std::{ops::Mul, slice};

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

    #[rustfmt::skip]
    fn mul(self, rhs: &'b MatrixN) -> Self::Output {
        assert_eq!(rhs.order, self.order);

        let mut out = Self::Output::zeros(self.order);

        for i in 0..out.order {
            for j in 0..out.order {
                unsafe {
                    *out.get_unchecked_mut((i, j)) =
                          self.get_unchecked((i, 0)) * rhs.get_unchecked((0, j))
                        + self.get_unchecked((i, 1)) * rhs.get_unchecked((1, j))
                        + self.get_unchecked((i, 2)) * rhs.get_unchecked((2, j))
                        + self.get_unchecked((i, 3)) * rhs.get_unchecked((3, j));
                }
            }
        }

        out
    }
}
