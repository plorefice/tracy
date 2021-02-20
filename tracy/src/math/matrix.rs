//! Matrix representations and operations defined on them.

use std::{
    ops::{Index, IndexMut, Mul},
    slice,
};

use super::{Point3, Vec3};

/// A NxN, column-major matrix.
#[cfg_attr(feature = "serde-support", derive(serde::Serialize))]
#[derive(Debug, Clone, PartialEq)]
pub struct Matrix {
    data: [f32; 16],
    order: usize,
}

impl Default for Matrix {
    fn default() -> Self {
        Self::identity(4)
    }
}

impl Matrix {
    /// Creates a matrix of order `n` filled with zeros.
    pub fn zeros(n: usize) -> Self {
        Self {
            data: [0.0; 16],
            order: n,
        }
    }

    /// Creates the identity matrix of order `n`.
    pub fn identity(n: usize) -> Self {
        let mut data = [0.0; 16];

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
        let cols = data.as_ref();
        assert_eq!(n * n, cols.len());

        let mut data: [f32; 16] = Default::default();
        data[..n * n].copy_from_slice(cols);

        Self { data, order: n }
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

    /// Creates a view transform matrix looking at `center` from `eye`.
    pub fn look_at(eye: Point3, center: Point3, up: Vec3) -> Matrix {
        let fwd = (center - eye).normalize();
        let up = up.normalize();
        let left = fwd.cross(&up);
        let up = left.cross(&fwd);

        let orientation = Matrix::from_column_slice(
            4,
            [
                left.x, up.x, -fwd.x, 0.0, left.y, up.y, -fwd.y, 0.0, left.z, up.z, -fwd.z, 0.0,
                0.0, 0.0, 0.0, 1.0,
            ],
        );

        orientation * Matrix::from_translation(-eye.x, -eye.y, -eye.z)
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
    pub fn inverse(&self) -> Option<Matrix> {
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
    pub fn submatrix(&self, row: usize, col: usize) -> Matrix {
        let mut out = Matrix::zeros(self.order() - 1);
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

impl Index<(usize, usize)> for Matrix {
    type Output = f32;

    fn index(&self, (i, j): (usize, usize)) -> &Self::Output {
        self.data.index(self.liner_index(i, j))
    }
}

impl IndexMut<(usize, usize)> for Matrix {
    fn index_mut(&mut self, (i, j): (usize, usize)) -> &mut Self::Output {
        self.data.index_mut(self.liner_index(i, j))
    }
}

impl Mul for Matrix {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self::Output {
        &self * &rhs
    }
}

impl Mul<&Matrix> for Matrix {
    type Output = Self;

    fn mul(self, rhs: &Self) -> Self::Output {
        &self * rhs
    }
}

impl Mul<Matrix> for &Matrix {
    type Output = Matrix;

    fn mul(self, rhs: Matrix) -> Self::Output {
        self * &rhs
    }
}

impl<'a, 'b> Mul<&'b Matrix> for &'a Matrix {
    type Output = Matrix;

    fn mul(self, rhs: &'b Matrix) -> Self::Output {
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

macro_rules! impl_mul {
    ($($t:ty)*) => {$(
        impl Mul<$t> for Matrix {
            type Output = $t;

            fn mul(self, rhs: $t) -> Self::Output {
                &self * &rhs
            }
        }

        impl Mul<$t> for &Matrix {
            type Output = $t;

            fn mul(self, rhs: $t) -> Self::Output {
                self * &rhs
            }
        }

        impl Mul<&$t> for Matrix {
            type Output = $t;

            fn mul(self, rhs: &$t) -> Self::Output {
                &self * rhs
            }
        }

        impl Mul<&$t> for &Matrix {
            type Output = $t;

            fn mul(self, rhs: &$t) -> Self::Output {
                let coords = self * <$t as Into<[f32; 4]>>::into(*rhs);
                Self::Output::new(coords[0], coords[1], coords[2])
            }
        }
    )*};
}

impl_mul!(Point3 Vec3);

impl Mul<[f32; 4]> for Matrix {
    type Output = [f32; 4];

    fn mul(self, rhs: [f32; 4]) -> Self::Output {
        &self * rhs
    }
}

impl Mul<[f32; 4]> for &Matrix {
    type Output = [f32; 4];

    #[allow(clippy::suspicious_arithmetic_impl)]
    fn mul(self, rhs: [f32; 4]) -> Self::Output {
        [
            (0..4).fold(0., |sum, i| sum + self[(0, i)] * rhs[i]),
            (0..4).fold(0., |sum, i| sum + self[(1, i)] * rhs[i]),
            (0..4).fold(0., |sum, i| sum + self[(2, i)] * rhs[i]),
            (0..4).fold(0., |sum, i| sum + self[(3, i)] * rhs[i]),
        ]
    }
}

// NOTE: this is an extremely efficient, loop-unrolled matrix inverse from MESA (MIT licensed).
fn do_inverse4(m: &Matrix, out: &mut Matrix) -> bool {
    let m = m.data;

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

#[cfg(feature = "serde-support")]
impl<'de> serde::Deserialize<'de> for Matrix {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        use std::fmt;

        use serde::de::{self, SeqAccess};

        enum Isometry {
            RotateX(f32),
            RotateY(f32),
            RotateZ(f32),
            Translate { x: f32, y: f32, z: f32 },
            Scale { x: f32, y: f32, z: f32 },
        }

        impl<'de> serde::Deserialize<'de> for Isometry {
            fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
            where
                D: serde::Deserializer<'de>,
            {
                #[derive(Debug, serde::Deserialize)]
                #[serde(rename_all = "lowercase")]
                enum IsometryKind {
                    #[serde(rename = "rotate-x")]
                    RotateX,
                    #[serde(rename = "rotate-y")]
                    RotateY,
                    #[serde(rename = "rotate-z")]
                    RotateZ,
                    Translate,
                    Scale,
                }

                struct IsometryVisitor;

                impl<'de> de::Visitor<'de> for IsometryVisitor {
                    type Value = Isometry;

                    fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                        fmt.write_str("Isometry")
                    }

                    fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
                    where
                        V: SeqAccess<'de>,
                    {
                        let kind: IsometryKind = seq
                            .next_element()?
                            .ok_or_else(|| de::Error::invalid_length(0, &self))?;

                        match kind {
                            IsometryKind::RotateX => {
                                let angle = seq
                                    .next_element()?
                                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                                Ok(Isometry::RotateX(angle))
                            }
                            IsometryKind::RotateY => {
                                let angle = seq
                                    .next_element()?
                                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                                Ok(Isometry::RotateY(angle))
                            }
                            IsometryKind::RotateZ => {
                                let angle = seq
                                    .next_element()?
                                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                                Ok(Isometry::RotateZ(angle))
                            }
                            IsometryKind::Translate => {
                                let x = seq
                                    .next_element()?
                                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                                let y = seq
                                    .next_element()?
                                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;

                                let z = seq
                                    .next_element()?
                                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;

                                Ok(Isometry::Translate { x, y, z })
                            }
                            IsometryKind::Scale => {
                                let x = seq
                                    .next_element()?
                                    .ok_or_else(|| de::Error::invalid_length(1, &self))?;

                                let y = seq
                                    .next_element()?
                                    .ok_or_else(|| de::Error::invalid_length(2, &self))?;

                                let z = seq
                                    .next_element()?
                                    .ok_or_else(|| de::Error::invalid_length(3, &self))?;

                                Ok(Isometry::Scale { x, y, z })
                            }
                        }
                    }
                }

                deserializer.deserialize_seq(IsometryVisitor)
            }
        }

        struct MatrixVisitor;

        impl<'de> de::Visitor<'de> for MatrixVisitor {
            type Value = Matrix;

            fn expecting(&self, fmt: &mut fmt::Formatter<'_>) -> fmt::Result {
                fmt.write_str("Matrix")
            }

            fn visit_seq<V>(self, mut seq: V) -> Result<Self::Value, V::Error>
            where
                V: SeqAccess<'de>,
            {
                let mut m = Matrix::identity(4);

                while let Some(isometry) = seq.next_element()? {
                    match isometry {
                        Isometry::RotateX(angle) => m = Matrix::from_rotation_x(angle) * m,
                        Isometry::RotateY(angle) => m = Matrix::from_rotation_y(angle) * m,
                        Isometry::RotateZ(angle) => m = Matrix::from_rotation_z(angle) * m,
                        Isometry::Translate { x, y, z } => {
                            m = Matrix::from_translation(x, y, z) * m
                        }
                        Isometry::Scale { x, y, z } => m = Matrix::from_scale(x, y, z) * m,
                    }
                }

                Ok(m)
            }
        }

        deserializer.deserialize_seq(MatrixVisitor)
    }
}

#[cfg(all(feature = "serde-support", test))]
mod tests {
    use std::f32::consts::PI;

    use serde::Deserialize;
    use serde_test::{assert_de_tokens, Deserializer, Token};

    use super::*;

    #[test]
    fn deserialize_scale() {
        let m = Matrix::from_scale(1.0, 2.0, 3.0);

        assert_de_tokens(
            &m,
            &[
                Token::Seq { len: Some(1) },
                Token::Seq { len: Some(4) },
                Token::Enum {
                    name: "IsometryKind",
                },
                Token::UnitVariant {
                    name: "IsometryKind",
                    variant: "scale",
                },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::F32(3.0),
                Token::SeqEnd,
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn deserialize_translation() {
        let m = Matrix::from_translation(1.0, 2.0, 3.0);

        assert_de_tokens(
            &m,
            &[
                Token::Seq { len: Some(1) },
                Token::Seq { len: Some(4) },
                Token::Enum {
                    name: "IsometryKind",
                },
                Token::UnitVariant {
                    name: "IsometryKind",
                    variant: "translate",
                },
                Token::F32(1.0),
                Token::F32(2.0),
                Token::F32(3.0),
                Token::SeqEnd,
                Token::SeqEnd,
            ],
        );
    }

    #[test]
    fn deserialize_rotations() {
        for (m, angle, kind) in &[
            (Matrix::from_rotation_x(PI), PI, "rotate-x"),
            (Matrix::from_rotation_y(PI), PI, "rotate-y"),
            (Matrix::from_rotation_z(PI), PI, "rotate-z"),
        ] {
            assert_de_tokens(
                m,
                &[
                    Token::Seq { len: Some(1) },
                    Token::Seq { len: Some(2) },
                    Token::Enum {
                        name: "IsometryKind",
                    },
                    Token::UnitVariant {
                        name: "IsometryKind",
                        variant: kind,
                    },
                    Token::F32(*angle),
                    Token::SeqEnd,
                    Token::SeqEnd,
                ],
            );
        }
    }

    #[test]
    fn deserialize_complex() {
        let exp = Matrix::from_rotation_x(PI)
            * Matrix::from_rotation_y(PI / 2.0)
            * Matrix::from_rotation_z(PI / 3.0)
            * Matrix::from_translation(2.0, 3.0, 4.0)
            * Matrix::from_scale(0.2, 0.3, 0.4);

        let mut de = Deserializer::new(&[
            Token::Seq { len: Some(5) },
            // #1: scale
            Token::Seq { len: Some(4) },
            Token::Enum {
                name: "IsometryKind",
            },
            Token::UnitVariant {
                name: "IsometryKind",
                variant: "scale",
            },
            Token::F32(0.2),
            Token::F32(0.3),
            Token::F32(0.4),
            Token::SeqEnd,
            // #2: translate
            Token::Seq { len: Some(4) },
            Token::Enum {
                name: "IsometryKind",
            },
            Token::UnitVariant {
                name: "IsometryKind",
                variant: "translate",
            },
            Token::F32(2.0),
            Token::F32(3.0),
            Token::F32(4.0),
            Token::SeqEnd,
            // #3: rotate-z
            Token::Seq { len: Some(2) },
            Token::Enum {
                name: "IsometryKind",
            },
            Token::UnitVariant {
                name: "IsometryKind",
                variant: "rotate-z",
            },
            Token::F32(PI / 3.0),
            Token::SeqEnd,
            // #4: rotate-y
            Token::Seq { len: Some(2) },
            Token::Enum {
                name: "IsometryKind",
            },
            Token::UnitVariant {
                name: "IsometryKind",
                variant: "rotate-y",
            },
            Token::F32(PI / 2.0),
            Token::SeqEnd,
            // #5: rotate-x
            Token::Seq { len: Some(2) },
            Token::Enum {
                name: "IsometryKind",
            },
            Token::UnitVariant {
                name: "IsometryKind",
                variant: "rotate-x",
            },
            Token::F32(PI),
            Token::SeqEnd,
            Token::SeqEnd,
        ]);

        let res = Matrix::deserialize(&mut de).unwrap();

        assert!(exp.abs_diff_eq(&res, crate::math::EPSILON));
    }
}
