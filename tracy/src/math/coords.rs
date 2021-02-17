//! Coordinate system.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

use super::EPSILON;

/// A point in 3D space.
pub type Point = Coords;

/// A vector in 3D space.
pub type Vector = Coords;

/// A four-dimensional `(x,y,z,w)` tuple  that can represent a point or vector in 3D space.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Coords {
    /// The `x` component of this tuple.
    pub x: f32,
    /// The `y` component of this tuple.
    pub y: f32,
    /// The `z` component of this tuple.
    pub z: f32,
    /// The `w` component of this tuple.
    pub w: f32,
}

impl From<(f32, f32, f32, f32)> for Coords {
    fn from((x, y, z, w): (f32, f32, f32, f32)) -> Self {
        Self { x, y, z, w }
    }
}

impl From<Coords> for [f32; 4] {
    fn from(c: Coords) -> Self {
        [c.x, c.y, c.z, c.w]
    }
}

impl Coords {
    /// Creates a new tuple from the coordinates of a point in space.
    pub fn from_point(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 1.0 }
    }

    /// Creates a new tuple from the coordinates of a vector in space.
    pub fn from_vector(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z, w: 0.0 }
    }

    /// Checks whether this tuple represents a point, ie. its `w` component is equal to 1.
    pub fn is_point(&self) -> bool {
        (self.w - 1.0).abs() <= EPSILON
    }

    /// Checks whether this tuple represents a vector, ie. its `w` component is equal to 0.
    pub fn is_vector(&self) -> bool {
        self.w == 0.0
    }

    /// Converts `self` into a point.
    pub fn to_point(mut self) -> Self {
        self.w = 1.;
        self
    }

    /// Converts `self` into a vector.
    pub fn to_vector(mut self) -> Self {
        self.w = 0.;
        self
    }

    /// Returns true if the absolute difference of all elements between `self` and `other`
    /// is less than or equal to `max_abs_diff`.
    pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        (self.x - other.x).abs() < max_abs_diff
            && (self.y - other.y).abs() < max_abs_diff
            && (self.z - other.z).abs() < max_abs_diff
            && (self.w - other.w).abs() < max_abs_diff
    }

    /// Computes the magnitude of `self`.
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2) + self.w.powi(2)).sqrt()
    }

    /// Returns `self` normalized to length 1.0.
    pub fn normalize(&self) -> Self {
        Self {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length(),
            w: self.w / self.length(),
        }
    }

    /// Computes the dot product of `self` and `rhs`.
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z + self.w * rhs.w
    }

    /// Computes the cross product of `self` and `rhs`.
    ///
    /// This operation only makes sense on vectors.
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::from_vector(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    /// Reflects `self` around `n`.
    ///
    /// This operation only makes sense on vectors.
    pub fn reflect(&self, n: &Self) -> Self {
        self - n * 2. * self.dot(n)
    }
}

macro_rules! impl_ref_unary_op {
    (impl $imp:ident, $method:ident for $t:ty) => {
        impl<'a> $imp for &'a $t {
            type Output = <$t as $imp>::Output;

            #[inline]
            fn $method(self) -> <$t as $imp>::Output {
                $imp::$method(*self)
            }
        }
    };
}

macro_rules! impl_ref_bin_op {
    (impl $imp:ident, $method:ident for $t:ty, $u:ty) => {
        impl<'a> $imp<$u> for &'a $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, other)
            }
        }

        impl<'a> $imp<&'a $u> for $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(self, *other)
            }
        }

        impl<'a, 'b> $imp<&'a $u> for &'b $t {
            type Output = <$t as $imp<$u>>::Output;

            #[inline]
            fn $method(self, other: &'a $u) -> <$t as $imp<$u>>::Output {
                $imp::$method(*self, *other)
            }
        }
    };
}

macro_rules! impl_ops {
    ($($t:ty)*) => ($(
        impl Add for $t {
            type Output = Self;

            fn add(self, rhs: Self) -> Self::Output {
                Self::Output {
                    x: self.x + rhs.x,
                    y: self.y + rhs.y,
                    z: self.z + rhs.z,
                    w: self.w + rhs.w,
                }
            }
        }

        impl_ref_bin_op!(impl Add, add for $t, $t);

        impl Sub for $t {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::Output {
                    x: self.x - rhs.x,
                    y: self.y - rhs.y,
                    z: self.z - rhs.z,
                    w: self.w - rhs.w,
                }
            }
        }

        impl_ref_bin_op!(impl Sub, sub for $t, $t);

        impl Mul<f32> for $t {
            type Output = $t;

            fn mul(self, rhs: f32) -> Self::Output {
                Self::Output {
                    x: rhs * self.x,
                    y: rhs * self.y,
                    z: rhs * self.z,
                    w: rhs * self.w,
                }
            }
        }

        impl_ref_bin_op!(impl Mul, mul for $t, f32);

        impl Div<f32> for $t {
            type Output = $t;

            fn div(self, rhs: f32) -> Self::Output {
                Self::Output {
                    x: self.x / rhs,
                    y: self.y / rhs,
                    z: self.z / rhs,
                    w: self.w / rhs,
                }
            }
        }

        impl_ref_bin_op!(impl Div, div for $t, f32);

        impl Neg for $t {
            type Output = Self;

            fn neg(self) -> Self::Output {
                Self::Output {
                    x: -self.x,
                    y: -self.y,
                    z: -self.z,
                    w: -self.w,
                }
            }
        }

        impl_ref_unary_op!(impl Neg, neg for $t);
    )*)
}

impl_ops!(Coords);

impl AddAssign for Coords {
    fn add_assign(&mut self, rhs: Self) {
        self.x += rhs.x;
        self.y += rhs.y;
        self.z += rhs.z;
        self.w += rhs.w;
    }
}

impl SubAssign for Coords {
    fn sub_assign(&mut self, rhs: Self) {
        self.x -= rhs.x;
        self.y -= rhs.y;
        self.z -= rhs.z;
        self.w -= rhs.w;
    }
}

impl MulAssign<f32> for Coords {
    fn mul_assign(&mut self, rhs: f32) {
        self.x *= rhs;
        self.y *= rhs;
        self.z *= rhs;
        self.w *= rhs;
    }
}

impl DivAssign<f32> for Coords {
    fn div_assign(&mut self, rhs: f32) {
        self.x /= rhs;
        self.y /= rhs;
        self.z /= rhs;
        self.w /= rhs;
    }
}
