use crate::math::Point;

use super::Color;

/// Traits commont to all colored patterns.
pub trait Pattern {
    /// Returns the color of `self` at `p`.
    fn color_at(&self, p: &Point) -> Color;
}

/// A pattern composed of two repeating, equally spaced color stripes.
///
/// The pattern is constant in the `y` and `z` coordinates, and alternates at each integer unit
/// of the `x` coordinate.
#[derive(Debug, Clone, PartialEq)]
pub struct StripePattern {
    ca: Color,
    cb: Color,
}

impl StripePattern {
    /// Creates a stripe pattern given two colors.
    pub const fn new(ca: Color, cb: Color) -> StripePattern {
        Self { ca, cb }
    }

    /// Returns the colors of this pattern.
    pub const fn colors(&self) -> (Color, Color) {
        (self.ca, self.cb)
    }
}

impl Pattern for StripePattern {
    fn color_at(&self, p: &Point) -> Color {
        if (p.x.floor() as i32) % 2 == 0 {
            self.ca
        } else {
            self.cb
        }
    }
}
