use crate::math::Point;

use super::Color;

/// Colored patterns.
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// A pattern composed of two repeating, equally spaced color stripes.
    ///
    /// The pattern is constant in the `y` and `z` coordinates, and alternates at each integer unit
    /// of the `x` coordinate.
    StripePattern {
        /// The color assigned when `x` is even.
        ca: Color,
        /// The color assigned when `x` is odd.
        cb: Color,
    },
}

impl Pattern {
    /// Returns the color of `self` at `p`.
    pub fn color_at(&self, p: &Point) -> Color {
        match *self {
            Self::StripePattern { ca, cb } => {
                if (p.x.floor() as i32) % 2 == 0 {
                    ca
                } else {
                    cb
                }
            }
        }
    }
}
