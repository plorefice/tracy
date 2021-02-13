//! Matrix representations and operations defined on them.

use std::{
    ops::{Index, IndexMut, Mul},
    slice,
};

use super::Coords;

/// A NxN, column-major matrix.
#[derive(Debug, Clone, PartialEq)]
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

    /// Creates a matrix that applies a translation of `(x,y,z)`.
    pub fn from_translation(x: f32, y: f32, z: f32) -> Self {
        let mut out = Self::identity(4);
        out[(0, 3)] = x;
        out[(1, 3)] = y;
        out[(2, 3)] = z;
        out
    }

    /// Creates a matrix that applies a non-uniform scaling of `(x,y,z)`.
    pub fn from_scale(x: f32, y: f32, z: f32) -> Self {
        let mut out = Self::identity(4);
        out[(0, 0)] = x;
        out[(1, 1)] = y;
        out[(2, 2)] = z;
        out
    }

    /// Creates a matrix that applies a rotation of `rad` radians around the `x` axis.
    pub fn from_rotation_x(rad: f32) -> Self {
        let mut out = Self::identity(4);
        out[(1, 1)] = rad.cos();
        out[(1, 2)] = -rad.sin();
        out[(2, 1)] = rad.sin();
        out[(2, 2)] = rad.cos();
        out
    }

    /// Creates a matrix that applies a rotation of `rad` radians around the `y` axis.
    pub fn from_rotation_y(rad: f32) -> Self {
        let mut out = Self::identity(4);
        out[(0, 0)] = rad.cos();
        out[(0, 2)] = rad.sin();
        out[(2, 0)] = -rad.sin();
        out[(2, 2)] = rad.cos();
        out
    }

    /// Creates a matrix that applies a rotation of `rad` radians around the `z` axis.
    pub fn from_rotation_z(rad: f32) -> Self {
        let mut out = Self::identity(4);
        out[(0, 0)] = rad.cos();
        out[(0, 1)] = -rad.sin();
        out[(1, 0)] = rad.sin();
        out[(1, 1)] = rad.cos();
        out
    }

    /// Creates a matrix that applies the specified shear.
    pub fn from_shear(xy: f32, xz: f32, yx: f32, yz: f32, zx: f32, zy: f32) -> Self {
        let mut out = Self::identity(4);
        out[(0, 1)] = xy;
        out[(0, 2)] = xz;
        out[(1, 0)] = yx;
        out[(1, 2)] = yz;
        out[(2, 0)] = zx;
        out[(2, 1)] = zy;
        out
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

    /// Returns the inverse of the matrix, or `None` if the matrix is not invertible.
    pub fn inverse(&self) -> Option<MatrixN> {
        match self.order() {
            0 => None,
            4 => {
                let mut out = Self::zeros(4);
                if do_inverse4(self, &mut out) {
                    Some(out)
                } else {
                    None
                }
            }
            n => {
                let mut out = Self::zeros(n);
                let det = self.det();
                if det != 0. {
                    for i in 0..n {
                        for j in 0..n {
                            let c = self.cofactor(i, j);
                            out[(j, i)] = c / det;
                        }
                    }
                    Some(out)
                } else {
                    None
                }
            }
        }
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

// NOTE: this is an extremely efficient, loop-unrolled matrix inverse from MESA (MIT licensed).
fn do_inverse4(m: &MatrixN, out: &mut MatrixN) -> bool {
    let m = m.data.as_slice();

    out[(0, 0)] = m[5] * m[10] * m[15] - m[5] * m[11] * m[14] - m[9] * m[6] * m[15]
        + m[9] * m[7] * m[14]
        + m[13] * m[6] * m[11]
        - m[13] * m[7] * m[10];

    out[(1, 0)] = -m[1] * m[10] * m[15] + m[1] * m[11] * m[14] + m[9] * m[2] * m[15]
        - m[9] * m[3] * m[14]
        - m[13] * m[2] * m[11]
        + m[13] * m[3] * m[10];

    out[(2, 0)] = m[1] * m[6] * m[15] - m[1] * m[7] * m[14] - m[5] * m[2] * m[15]
        + m[5] * m[3] * m[14]
        + m[13] * m[2] * m[7]
        - m[13] * m[3] * m[6];

    out[(3, 0)] = -m[1] * m[6] * m[11] + m[1] * m[7] * m[10] + m[5] * m[2] * m[11]
        - m[5] * m[3] * m[10]
        - m[9] * m[2] * m[7]
        + m[9] * m[3] * m[6];

    out[(0, 1)] = -m[4] * m[10] * m[15] + m[4] * m[11] * m[14] + m[8] * m[6] * m[15]
        - m[8] * m[7] * m[14]
        - m[12] * m[6] * m[11]
        + m[12] * m[7] * m[10];

    out[(1, 1)] = m[0] * m[10] * m[15] - m[0] * m[11] * m[14] - m[8] * m[2] * m[15]
        + m[8] * m[3] * m[14]
        + m[12] * m[2] * m[11]
        - m[12] * m[3] * m[10];

    out[(2, 1)] = -m[0] * m[6] * m[15] + m[0] * m[7] * m[14] + m[4] * m[2] * m[15]
        - m[4] * m[3] * m[14]
        - m[12] * m[2] * m[7]
        + m[12] * m[3] * m[6];

    out[(3, 1)] = m[0] * m[6] * m[11] - m[0] * m[7] * m[10] - m[4] * m[2] * m[11]
        + m[4] * m[3] * m[10]
        + m[8] * m[2] * m[7]
        - m[8] * m[3] * m[6];

    out[(0, 2)] = m[4] * m[9] * m[15] - m[4] * m[11] * m[13] - m[8] * m[5] * m[15]
        + m[8] * m[7] * m[13]
        + m[12] * m[5] * m[11]
        - m[12] * m[7] * m[9];

    out[(1, 2)] = -m[0] * m[9] * m[15] + m[0] * m[11] * m[13] + m[8] * m[1] * m[15]
        - m[8] * m[3] * m[13]
        - m[12] * m[1] * m[11]
        + m[12] * m[3] * m[9];

    out[(2, 2)] = m[0] * m[5] * m[15] - m[0] * m[7] * m[13] - m[4] * m[1] * m[15]
        + m[4] * m[3] * m[13]
        + m[12] * m[1] * m[7]
        - m[12] * m[3] * m[5];

    out[(0, 3)] = -m[4] * m[9] * m[14] + m[4] * m[10] * m[13] + m[8] * m[5] * m[14]
        - m[8] * m[6] * m[13]
        - m[12] * m[5] * m[10]
        + m[12] * m[6] * m[9];

    out[(3, 2)] = -m[0] * m[5] * m[11] + m[0] * m[7] * m[9] + m[4] * m[1] * m[11]
        - m[4] * m[3] * m[9]
        - m[8] * m[1] * m[7]
        + m[8] * m[3] * m[5];

    out[(1, 3)] = m[0] * m[9] * m[14] - m[0] * m[10] * m[13] - m[8] * m[1] * m[14]
        + m[8] * m[2] * m[13]
        + m[12] * m[1] * m[10]
        - m[12] * m[2] * m[9];

    out[(2, 3)] = -m[0] * m[5] * m[14] + m[0] * m[6] * m[13] + m[4] * m[1] * m[14]
        - m[4] * m[2] * m[13]
        - m[12] * m[1] * m[6]
        + m[12] * m[2] * m[5];

    out[(3, 3)] = m[0] * m[5] * m[10] - m[0] * m[6] * m[9] - m[4] * m[1] * m[10]
        + m[4] * m[2] * m[9]
        + m[8] * m[1] * m[6]
        - m[8] * m[2] * m[5];

    let det = m[0] * out[(0, 0)] + m[1] * out[(0, 1)] + m[2] * out[(0, 2)] + m[3] * out[(0, 3)];

    if det != 0. {
        let inv_det = 1. / det;

        for j in 0..4 {
            for i in 0..4 {
                out[(i, j)] *= inv_det;
            }
        }
        true
    } else {
        false
    }
}