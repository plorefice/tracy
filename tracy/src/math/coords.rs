//! Coordinate system.

use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Neg, Sub, SubAssign};

/// A point in 3D space.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Point3 {
    /// The `x` component of this point.
    pub x: f32,
    /// The `y` component of this point.
    pub y: f32,
    /// The `z` component of this point.
    pub z: f32,
}

/// A vector in 3D space.
#[derive(Debug, Default, Clone, Copy, PartialEq)]
pub struct Vec3 {
    /// The `x` component of this vector.
    pub x: f32,
    /// The `y` component of this vector.
    pub y: f32,
    /// The `z` component of this vector.
    pub z: f32,
}

impl Point3 {
    /// Creates a new point from its coordinates.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Returns true if the absolute difference of all elements between `self` and `other`
    /// is less than or equal to `max_abs_diff`.
    pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        (self.x - other.x).abs() < max_abs_diff
            && (self.y - other.y).abs() < max_abs_diff
            && (self.z - other.z).abs() < max_abs_diff
    }
}

impl Vec3 {
    /// Creates a new vector from its coordinates.
    pub fn new(x: f32, y: f32, z: f32) -> Self {
        Self { x, y, z }
    }

    /// Creates the zero vector.
    pub fn zero() -> Self {
        Self::default()
    }

    /// Creates the unit `x` vector, ie. `(1, 0, 0)`.
    pub fn unit_x() -> Self {
        Self::new(1.0, 0.0, 0.0)
    }

    /// Creates the unit `y` vector, ie. `(0, 1, 0)`.
    pub fn unit_y() -> Self {
        Self::new(0.0, 1.0, 0.0)
    }

    /// Creates the unit `z` vector, ie. `(0, 0, 1)`.
    pub fn unit_z() -> Self {
        Self::new(0.0, 0.0, 1.0)
    }

    /// Computes the magnitude of `self`.
    pub fn length(&self) -> f32 {
        (self.x.powi(2) + self.y.powi(2) + self.z.powi(2)).sqrt()
    }

    /// Returns `self` normalized to length 1.0.
    pub fn normalize(&self) -> Self {
        Self {
            x: self.x / self.length(),
            y: self.y / self.length(),
            z: self.z / self.length(),
        }
    }

    /// Computes the dot product of `self` and `rhs`.
    pub fn dot(&self, rhs: &Self) -> f32 {
        self.x * rhs.x + self.y * rhs.y + self.z * rhs.z
    }

    /// Computes the cross product of `self` and `rhs`.
    pub fn cross(&self, rhs: &Self) -> Self {
        Self::new(
            self.y * rhs.z - self.z * rhs.y,
            self.z * rhs.x - self.x * rhs.z,
            self.x * rhs.y - self.y * rhs.x,
        )
    }

    /// Reflects `self` around `n`.
    pub fn reflect(&self, n: &Self) -> Self {
        self - n * 2.0 * self.dot(n)
    }

    /// Returns true if the absolute difference of all elements between `self` and `other`
    /// is less than or equal to `max_abs_diff`.
    pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        (self.x - other.x).abs() < max_abs_diff
            && (self.y - other.y).abs() < max_abs_diff
            && (self.z - other.z).abs() < max_abs_diff
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

macro_rules! impl_unary_op {
    (impl $imp:ident[$method:ident, $op:tt] for $t:ty) => {
        impl $imp for $t {
            type Output = $t;

            fn $method(self) -> Self::Output {
                Self::Output {
                    x: $op self.x,
                    y: $op self.y,
                    z: $op self.z,
                }
            }
        }

        impl_ref_unary_op!(impl $imp, $method for $t);
    };
}

macro_rules! impl_bin_op {
    (impl $imp:ident[$method:ident, $op:tt] for $t:ty : $u:ty => $out:ty) => {
        impl $imp<$u> for $t {
            type Output = $out;

            fn $method(self, rhs: $u) -> Self::Output {
                Self::Output {
                    x: self.x $op rhs.x,
                    y: self.y $op rhs.y,
                    z: self.z $op rhs.z,
                }
            }
        }

        impl_ref_bin_op!(impl $imp, $method for $t, $u);
    };
}

macro_rules! impl_op_scalar {
    (impl $imp:ident[$method:ident, $op:tt] for $t:ty : $u:ty => $out:ty) => {
        impl $imp<$u> for $t {
            type Output = $out;

            fn $method(self, rhs: $u) -> Self::Output {
                Self::Output {
                    x: self.x $op rhs,
                    y: self.y $op rhs,
                    z: self.z $op rhs,
                }
            }
        }

        impl_ref_bin_op!(impl $imp, $method for $t, $u);
    };
}

macro_rules! impl_assign_ops {
    ($($t:ty)*) => {
        $(
            impl AddAssign for $t {
                fn add_assign(&mut self, rhs: Self) {
                    self.x += rhs.x;
                    self.y += rhs.y;
                    self.z += rhs.z;
                }
            }

            impl SubAssign for $t {
                fn sub_assign(&mut self, rhs: Self) {
                    self.x -= rhs.x;
                    self.y -= rhs.y;
                    self.z -= rhs.z;
                }
            }

            impl MulAssign<f32> for $t {
                fn mul_assign(&mut self, rhs: f32) {
                    self.x *= rhs;
                    self.y *= rhs;
                    self.z *= rhs;
                }
            }

            impl DivAssign<f32> for $t {
                fn div_assign(&mut self, rhs: f32) {
                    self.x /= rhs;
                    self.y /= rhs;
                    self.z /= rhs;
                }
            }
        )*
    };
}

macro_rules! impl_conversions {
    ($t:ty, $w:expr) => {
        impl From<(f32, f32, f32)> for $t {
            fn from((x, y, z): (f32, f32, f32)) -> Self {
                Self { x, y, z }
            }
        }

        impl From<[f32; 3]> for $t {
            fn from([x, y, z]: [f32; 3]) -> Self {
                Self { x, y, z }
            }
        }

        impl From<$t> for (f32, f32, f32) {
            fn from(p: $t) -> Self {
                (p.x, p.y, p.z)
            }
        }

        impl From<$t> for (f32, f32, f32, f32) {
            fn from(p: $t) -> Self {
                (p.x, p.y, p.z, $w)
            }
        }

        impl From<$t> for [f32; 3] {
            fn from(p: $t) -> Self {
                [p.x, p.y, p.z]
            }
        }

        impl From<$t> for [f32; 4] {
            fn from(p: $t) -> Self {
                [p.x, p.y, p.z, $w]
            }
        }
    };
}

impl From<Point3> for Vec3 {
    fn from(p: Point3) -> Self {
        Self {
            x: p.x,
            y: p.y,
            z: p.z,
        }
    }
}

impl From<Vec3> for Point3 {
    fn from(v: Vec3) -> Self {
        Self {
            x: v.x,
            y: v.y,
            z: v.z,
        }
    }
}

impl_bin_op!(impl Add[add, +] for Point3 : Vec3 => Point3);
impl_bin_op!(impl Sub[sub, -] for Point3 : Vec3 => Point3);
impl_bin_op!(impl Sub[sub, -] for Point3 : Point3 => Vec3);

impl_bin_op!(impl Add[add, +] for Vec3 : Vec3 => Vec3);
impl_bin_op!(impl Sub[sub, -] for Vec3 : Vec3 => Vec3);

impl_op_scalar!(impl Mul[mul, *] for Point3 : f32 => Point3);
impl_op_scalar!(impl Div[div, /] for Point3 : f32 => Point3);
impl_op_scalar!(impl Mul[mul, *] for Vec3 : f32 => Vec3);
impl_op_scalar!(impl Div[div, /] for Vec3 : f32 => Vec3);

impl_unary_op!(impl Neg[neg, -] for Point3);
impl_unary_op!(impl Neg[neg, -] for Vec3);

impl_assign_ops!(Point3 Vec3);

impl_conversions!(Point3, 1.0);
impl_conversions!(Vec3, 0.0);
