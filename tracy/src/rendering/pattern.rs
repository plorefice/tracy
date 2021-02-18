use crate::math::Point;

use super::Color;

/// Colored patterns.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Pattern {
    /// A single solid color.
    Solid(Color),
    /// Two repeating, equally spaced color stripes.
    ///
    /// The pattern is constant in the `y` and `z` coordinates, and alternates at each integer unit
    /// of the `x` coordinate.
    Stripes {
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
            Self::Solid(c) => c,
            Self::Stripes { ca, cb } => {
                if (p.x.floor() as i32) % 2 == 0 {
                    ca
                } else {
                    cb
                }
            }
        }
    }
}
