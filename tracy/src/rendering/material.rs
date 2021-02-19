//! Materials that can be applied to objects for rendering.

use crate::{math::Point, rendering::Color};

use super::Pattern;

/// A material with standard properties.
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
    /// Shininess constant, larger for surfaces that are smoother and more mirror-like.
    pub shininess: f32,
    /// Reflectivity constant, 0 for completely opaque materials, 1 for a perfect mirror.
    pub reflective: f32,
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
        }
    }
}

impl Material {
    /// Returns the color of `self` at local-space coordinates `p`.
    pub fn color_at(&self, p: &Point) -> Color {
        self.pattern.color_at(p)
    }
}
