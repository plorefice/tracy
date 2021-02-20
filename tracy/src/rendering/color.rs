use std::ops::{Add, AddAssign, Div, DivAssign, Mul, MulAssign, Sub, SubAssign};

/// A color in RGB format.
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize),
    serde(from = "[f32; 3]")
)]
#[derive(Debug, Default, Clone, Copy, PartialEq, PartialOrd)]
pub struct Color {
    /// The red component of this color.
    pub r: f32,
    /// The green component of this color.
    pub g: f32,
    /// The blue component of this color.
    pub b: f32,
}

impl From<(f32, f32, f32)> for Color {
    fn from((r, g, b): (f32, f32, f32)) -> Self {
        Self::new(r, g, b)
    }
}

impl From<[f32; 3]> for Color {
    fn from([r, g, b]: [f32; 3]) -> Self {
        Self::new(r, g, b)
    }
}

impl Color {
    /// The black color.
    pub const BLACK: Color = Color::new(0.0, 0.0, 0.0);

    /// The white color.
    pub const WHITE: Color = Color::new(1.0, 1.0, 1.0);

    /// Creates a new color from its components.
    pub const fn new(r: f32, g: f32, b: f32) -> Self {
        Self { r, g, b }
    }

    /// Returns true if the absolute difference of all components between `self` and `other`
    /// is less than or equal to `max_abs_diff`.
    pub fn abs_diff_eq(&self, other: &Self, max_abs_diff: f32) -> bool {
        (self.r - other.r).abs() < max_abs_diff
            && (self.g - other.g).abs() < max_abs_diff
            && (self.b - other.b).abs() < max_abs_diff
    }

    /// Returns the RGB888 representation of `self`.
    pub fn to_rgb888(self) -> (u8, u8, u8) {
        (
            (self.r * 255.).max(0.).min(255.).round() as u8,
            (self.g * 255.).max(0.).min(255.).round() as u8,
            (self.b * 255.).max(0.).min(255.).round() as u8,
        )
    }
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
                    r: self.r + rhs.r,
                    g: self.g + rhs.g,
                    b: self.b + rhs.b,
                }
            }
        }

        impl_ref_bin_op!(impl Add, add for $t, $t);

        impl Sub for $t {
            type Output = Self;

            fn sub(self, rhs: Self) -> Self::Output {
                Self::Output {
                    r: self.r - rhs.r,
                    g: self.g - rhs.g,
                    b: self.b - rhs.b,
                }
            }
        }

        impl_ref_bin_op!(impl Sub, sub for $t, $t);

        impl Mul for $t {
            type Output = $t;

            fn mul(self, rhs: Self) -> Self::Output {
                Self::Output {
                    r: self.r * rhs.r,
                    g: self.g * rhs.g,
                    b: self.b * rhs.b,
                }
            }
        }

        impl_ref_bin_op!(impl Mul, mul for $t, $t);

        impl Div for $t {
            type Output = $t;

            fn div(self, rhs: Self) -> Self::Output {
                Self::Output {
                    r: self.r / rhs.r,
                    g: self.g / rhs.g,
                    b: self.b / rhs.b,
                }
            }
        }

        impl_ref_bin_op!(impl Div, div for $t, $t);

        impl Mul<f32> for $t {
            type Output = $t;

            fn mul(self, rhs: f32) -> Self::Output {
                Self::Output {
                    r: rhs * self.r,
                    g: rhs * self.g,
                    b: rhs * self.b,
                }
            }
        }

        impl_ref_bin_op!(impl Mul, mul for $t, f32);

        impl Div<f32> for $t {
            type Output = $t;

            fn div(self, rhs: f32) -> Self::Output {
                Self::Output {
                    r: self.r / rhs,
                    g: self.g / rhs,
                    b: self.b / rhs,
                }
            }
        }

        impl_ref_bin_op!(impl Div, div for $t, f32);
    )*)
}

impl_ops!(Color);

impl AddAssign for Color {
    fn add_assign(&mut self, rhs: Self) {
        self.r += rhs.r;
        self.g += rhs.g;
        self.b += rhs.b;
    }
}

impl SubAssign for Color {
    fn sub_assign(&mut self, rhs: Self) {
        self.r -= rhs.r;
        self.g -= rhs.g;
        self.b -= rhs.b;
    }
}

impl MulAssign<f32> for Color {
    fn mul_assign(&mut self, rhs: f32) {
        self.r *= rhs;
        self.g *= rhs;
        self.b *= rhs;
    }
}

impl DivAssign<f32> for Color {
    fn div_assign(&mut self, rhs: f32) {
        self.r /= rhs;
        self.g /= rhs;
        self.b /= rhs;
    }
}

#[cfg(all(feature = "serde-support", test))]
mod tests {
    use serde_test::{assert_de_tokens, Token};

    use super::*;

    #[test]
    fn deserialize_from_vec() {
        let c = Color::new(0.2, 0.3, 0.4);

        assert_de_tokens(
            &c,
            &[
                Token::Seq { len: Some(3) },
                Token::F32(0.2),
                Token::F32(0.3),
                Token::F32(0.4),
                Token::SeqEnd,
            ],
        );
    }
}
