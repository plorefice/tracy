use crate::math::{Matrix, Point3};

use super::Color;

/// A nestable, colored pattern.
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    kind: PatternKind,
    transform: Matrix,
}

/// Different kinds of patterns.
#[derive(Debug, Clone, PartialEq)]
pub enum PatternKind {
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
    /// Average of two patterns.
    Blended {
        /// The first blended pattern.
        a: Box<Pattern>,
        /// The second blended pattern.
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
    /// Test pattern that returns a color with the same coordinate of the point hit.
    Test,
}

impl From<Color> for PatternKind {
    fn from(c: Color) -> Self {
        Self::Solid(c)
    }
}

impl Pattern {
    /// Create a new pattern with an identity trasformation applied.
    pub fn new(kind: PatternKind) -> Self {
        Self::new_with_transform(kind, Matrix::identity(4))
    }

    /// Creates a new pattern with an applied transformation.
    pub fn new_with_transform(kind: PatternKind, transform: Matrix) -> Self {
        Self { kind, transform }
    }

    /// Returns the pattern kind of `self`.
    pub fn kind(&self) -> &PatternKind {
        &self.kind
    }

    /// Returns the transformation applied to `self`.
    pub fn transform(&self) -> &Matrix {
        &self.transform
    }

    /// Returns the color of `self` at object-space coordinates `p`.
    pub fn color_at(&self, p: &Point3) -> Color {
        let p = self.transform.inverse().unwrap() * p;

        match &self.kind {
            &PatternKind::Solid(c) => c,
            PatternKind::Stripes { a, b } => {
                if (p.x.floor() as i32) % 2 == 0 {
                    a.color_at(&p)
                } else {
                    b.color_at(&p)
                }
            }
            PatternKind::Rings { a, b } => {
                if (p.x.powi(2) + p.z.powi(2)).sqrt().floor() as i32 % 2 == 0 {
                    a.color_at(&p)
                } else {
                    b.color_at(&p)
                }
            }
            PatternKind::Checkers { a, b } => {
                if (p.x.floor() + p.y.floor() + p.z.floor()) as i32 % 2 == 0 {
                    a.color_at(&p)
                } else {
                    b.color_at(&p)
                }
            }
            PatternKind::Blended { a, b } => (a.color_at(&p) + b.color_at(&p)) / 2.0,
            PatternKind::LinearGradient { a, b } => a + (b - a) * (p.x - p.x.floor()),
            PatternKind::RadialGradient { a, b } => {
                let dist = (p.x.powi(2) + p.z.powi(2)).sqrt();
                a + (b - a) * (dist - dist.floor())
            }
            PatternKind::Test => Color::new(p.x, p.y, p.z),
        }
    }
}
