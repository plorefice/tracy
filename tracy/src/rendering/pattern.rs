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
    /// Linear gradient between two colors.
    ///
    /// The pattern is constant in the `y` and `z` coordinates, with gradient stops at each integer
    /// unit of the `x` coordinate.
    Gradient {
        /// The first gradient stop.
        ca: Color,
        /// The second gradient stop.
        cb: Color,
    },
    /// Two repeating, equally spaced color rings.
    ///
    /// The pattern is constant in the `y` coordinate, and alternates at each integer contentric
    /// ring on the `xz` plane.
    Rings {
        /// The color assigned on even rings.
        ca: Color,
        /// The color assigned on odd rings.
        cb: Color,
    },
}

impl Pattern {
    /// Returns the color of `self` at `p`.
    pub fn color_at(&self, p: &Point) -> Color {
        match *self {
            Self::Solid(c) => c,
            Self::Gradient { ca, cb } => ca + (cb - ca) * (p.x - p.x.floor()),
            Self::Stripes { ca, cb } => {
                if (p.x.floor() as i32) % 2 == 0 {
                    ca
                } else {
                    cb
                }
            }
            Self::Rings { ca, cb } => {
                if (p.x.powi(2) + p.z.powi(2)).sqrt().floor() as i32 % 2 == 0 {
                    ca
                } else {
                    cb
                }
            }
        }
    }
}
