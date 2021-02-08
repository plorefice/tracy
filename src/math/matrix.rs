//! Matrix representations and operations defined on them.

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
    pub fn from_column_slice(n: usize, data: &[f32]) -> Self {
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
    pub fn from_row_slice(n: usize, data: &[f32]) -> Self {
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

    /// Returns the linear index in the matrix storage corresponding to element `(irow,icol)`.
    fn liner_index(&self, irow: usize, icol: usize) -> usize {
        icol * self.order + irow
    }
}
