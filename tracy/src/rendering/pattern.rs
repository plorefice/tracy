use crate::math::Point;

use super::Color;

/// Nestable colored patterns.
#[derive(Debug, Clone, PartialEq)]
pub enum Pattern {
    /// A single solid color.
    Solid(Color),
    /// Two repeating, equally spaced pattern stripes.
    ///
    /// The pattern is constant in the `y` and `z` coordinates, and alternates at each integer unit
    /// of the `x` coordinate.
    Stripes {
        /// The pattern assigned when `x` is even.
        a: Box<Pattern>,
        /// The pattern assigned when `x` is odd.
        b: Box<Pattern>,
    },
    /// Two repeating, equally spaced pattern rings.
    ///
    /// The pattern is constant in the `y` coordinate, and alternates at each integer concentric
    /// ring on the `xz` plane.
    Rings {
        /// The pattern assigned on even rings.
        a: Box<Pattern>,
        /// The pattern assigned on odd rings.
        b: Box<Pattern>,
    },
    /// Alternating cubes in two patterns.
    Checkers {
        /// The first alternating color.
        a: Box<Pattern>,
        /// The second alternating color.
        b: Box<Pattern>,
    },
    /// Linear gradient between two colors.
    ///
    /// The pattern is constant in the `y` and `z` coordinates, with gradient stops at each integer
    /// unit of the `x` coordinate.
    LinearGradient {
        /// The first gradient stop.
        a: Color,
        /// The second gradient stop.
        b: Color,
    },
    /// Radial gradient between two colors.
    ///
    /// The pattern is constant in the `y` coordinate, with gradient stops at each integer
    /// concentric ring on the `xz` plane.
    RadialGradient {
        /// The first gradient stop.
        a: Color,
        /// The second gradient stop.
        b: Color,
    },
}

impl From<Color> for Pattern {
    fn from(c: Color) -> Self {
        Self::Solid(c)
    }
}

impl Pattern {
    /// Returns the color of `self` at `p`.
    pub fn color_at(&self, p: &Point) -> Color {
        match self {
            &Self::Solid(c) => c,
            Self::Stripes { a, b } => {
                if (p.x.floor() as i32) % 2 == 0 {
                    a.color_at(p)
                } else {
                    b.color_at(p)
                }
            }
            Self::Rings { a, b } => {
                if (p.x.powi(2) + p.z.powi(2)).sqrt().floor() as i32 % 2 == 0 {
                    a.color_at(p)
                } else {
                    b.color_at(p)
                }
            }
            Self::Checkers { a, b } => {
                if (p.x.floor() + p.y.floor() + p.z.floor()) as i32 % 2 == 0 {
                    a.color_at(p)
                } else {
                    b.color_at(p)
                }
            }
            Self::LinearGradient { a, b } => a + (b - a) * (p.x - p.x.floor()),
            Self::RadialGradient { a, b } => {
                let dist = (p.x.powi(2) + p.z.powi(2)).sqrt();
                a + (b - a) * (dist - dist.floor())
            }
        }
    }
}
