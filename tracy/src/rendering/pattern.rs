use crate::math::{Matrix, Point3};

use super::Color;

/// A nestable, colored pattern.
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize)
)]
#[derive(Debug, Clone, PartialEq)]
pub struct Pattern {
    kind: PatternKind,
    #[cfg_attr(feature = "serde-support", serde(default))]
    transform: Matrix,
}

/// Different kinds of patterns.
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize),
    serde(rename_all = "snake_case")
)]
#[derive(Debug, Clone, PartialEq)]
pub enum PatternKind {
    /// A single solid color.
    Solid(Color),
    /// Two repeating, equally spaced pattern stripes.
    ///
    /// The pattern is constant in the `y` and `z` coordinates, and alternates at each integer unit
    /// of the `x` coordinate.
    Stripes(Box<Pattern>, Box<Pattern>),
    /// Two repeating, equally spaced pattern rings.
    ///
    /// The pattern is constant in the `y` coordinate, and alternates at each integer concentric
    /// ring on the `xz` plane.
    Rings(Box<Pattern>, Box<Pattern>),
    /// Alternating cubes in two patterns.
    Checkers(Box<Pattern>, Box<Pattern>),
    /// Average of two patterns.
    Blended(Box<Pattern>, Box<Pattern>),
    /// Linear gradient between two colors.
    ///
    /// The pattern is constant in the `y` and `z` coordinates, with gradient stops at each integer
    /// unit of the `x` coordinate.
    LinearGradient(Color, Color),
    /// Radial gradient between two colors.
    ///
    /// The pattern is constant in the `y` coordinate, with gradient stops at each integer
    /// concentric ring on the `xz` plane.
    RadialGradient(Color, Color),
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
            PatternKind::Stripes(a, b) => {
                if (p.x.floor() as i32) % 2 == 0 {
                    a.color_at(&p)
                } else {
                    b.color_at(&p)
                }
            }
            PatternKind::Rings(a, b) => {
                if (p.x.powi(2) + p.z.powi(2)).sqrt().floor() as i32 % 2 == 0 {
                    a.color_at(&p)
                } else {
                    b.color_at(&p)
                }
            }
            PatternKind::Checkers(a, b) => {
                if (p.x.floor() + p.y.floor() + p.z.floor()) as i32 % 2 == 0 {
                    a.color_at(&p)
                } else {
                    b.color_at(&p)
                }
            }
            PatternKind::Blended(a, b) => (a.color_at(&p) + b.color_at(&p)) / 2.0,
            PatternKind::LinearGradient(a, b) => a + (b - a) * (p.x - p.x.floor()),
            PatternKind::RadialGradient(a, b) => {
                let dist = (p.x.powi(2) + p.z.powi(2)).sqrt();
                a + (b - a) * (dist - dist.floor())
            }
            PatternKind::Test => Color::new(p.x, p.y, p.z),
        }
    }
}

#[cfg(all(feature = "serde-support", test))]
mod tests {
    use serde_test::{assert_de_tokens, Token};

    use super::*;

    #[test]
    fn deserialize_complex_pattern() {
        let p = Pattern::new(PatternKind::Blended(
            Box::new(Pattern::new(Color::WHITE.into())),
            Box::new(Pattern::new(Color::BLACK.into())),
        ));

        /*
          pattern:
            kind:
              blended:
                - kind:
                    solid: [1, 1, 1]
                - kind:
                    solid: [0, 0, 0]
        */

        assert_de_tokens(
            &p,
            &[
                // outer pattern
                Token::Struct {
                    name: "Pattern",
                    len: 1,
                },
                Token::Str("kind"),
                // outer blended pattern
                Token::Enum {
                    name: "PatternKind",
                },
                Token::Str("blended"),
                Token::Seq { len: Some(2) },
                // first Solid start
                Token::Struct {
                    name: "Pattern",
                    len: 1,
                },
                Token::Str("kind"),
                Token::Enum {
                    name: "PatternKind",
                },
                Token::Str("solid"),
                Token::Seq { len: Some(2) },
                Token::F32(1.0),
                Token::F32(1.0),
                Token::F32(1.0),
                Token::SeqEnd,
                Token::StructEnd,
                // first Solid end
                // second Solid start
                Token::Struct {
                    name: "Pattern",
                    len: 1,
                },
                Token::Str("kind"),
                Token::Enum {
                    name: "PatternKind",
                },
                Token::Str("solid"),
                Token::Seq { len: Some(2) },
                Token::F32(0.0),
                Token::F32(0.0),
                Token::F32(0.0),
                Token::SeqEnd,
                Token::StructEnd,
                // second Solid end
                Token::SeqEnd,
                // outer Blended end
                Token::StructEnd,
            ],
        );
    }
}
