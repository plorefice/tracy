//! Materials that can be applied to objects for rendering.

use crate::{math::Point3, rendering::Color};

use super::Pattern;

/// A material with standard properties.
#[cfg_attr(
    feature = "serde-support",
    derive(serde::Serialize, serde::Deserialize),
    serde(default)
)]
#[derive(Debug, Clone, PartialEq)]
pub struct Material {
    /// Diffuse color pattern.
    pub pattern: Pattern,
    /// Ratio of reflection of the ambient term present in all points in the scene rendered.
    pub ambient: f32,
    /// Ratio of reflection of the diffuse term of incoming light.
    pub diffuse: f32,
    /// Ratio of reflection of the specular term of incoming light.
    pub specular: f32,
    /// Larger for surfaces that are smoother and more mirror-like.
    pub shininess: f32,
    /// 0 for completely opaque materials, 1 for a perfect mirror.
    pub reflective: f32,
    /// Larger for materials that let more light through.
    pub transparency: f32,
    /// Degree to which light will bend when entering or exiting the material.
    pub refractive_index: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            pattern: Pattern::new(Color::WHITE.into()),
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.0,
            reflective: 0.0,
            transparency: 0.0,
            refractive_index: 1.0,
        }
    }
}

impl Material {
    /// Returns the color of `self` at local-space coordinates `p`.
    pub fn color_at(&self, p: &Point3) -> Color {
        self.pattern.color_at(p)
    }
}

#[cfg(all(feature = "serde-support", test))]
mod tests {
    use serde_test::{assert_de_tokens, Token};

    use super::*;

    #[test]
    fn deserialize_default() {
        let m = Material {
            pattern: Pattern::new(Color::WHITE.into()),
            ambient: 1.0,
            specular: 0.6,
            ..Default::default()
        };

        assert_de_tokens(
            &m,
            &[
                Token::Struct {
                    name: "Material",
                    len: 3,
                },
                Token::Str("pattern"),
                // -> pattern
                Token::Struct {
                    name: "Pattern",
                    len: 1,
                },
                Token::Str("kind"),
                Token::Enum {
                    name: "PatternKind",
                },
                Token::Str("solid"),
                Token::Seq { len: Some(3) },
                Token::F32(1.0),
                Token::F32(1.0),
                Token::F32(1.0),
                Token::SeqEnd,
                Token::StructEnd,
                // <- pattern
                Token::Str("ambient"),
                Token::F32(1.0),
                Token::Str("specular"),
                Token::F32(0.6),
                Token::StructEnd,
            ],
        );
    }
}
