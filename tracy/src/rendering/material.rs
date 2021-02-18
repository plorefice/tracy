//! Materials that can be applied to objects for rendering.

use crate::canvas::Color;

/// A material with standard properties.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Material {
    /// Diffuse color.
    pub color: Color,
    /// Ratio of reflection of the ambient term present in all points in the scene rendered.
    pub ambient: f32,
    /// Ratio of reflection of the diffuse term of incoming light.
    pub diffuse: f32,
    /// Ratio of reflection of the specular term of incoming light.
    pub specular: f32,
    /// Shininess constant, larger for surfaces that are smoother and more mirror-like.
    pub shininess: f32,
}

impl Default for Material {
    fn default() -> Self {
        Self {
            color: Color::WHITE,
            ambient: 0.1,
            diffuse: 0.9,
            specular: 0.9,
            shininess: 200.,
        }
    }
}
