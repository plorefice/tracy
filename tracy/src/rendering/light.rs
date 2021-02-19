//! Light sources.

use crate::{
    math::{Point3, Vec3},
    query::Object,
    rendering::Color,
};

/// A point light source.
#[derive(Debug, Clone, PartialEq)]
pub struct PointLight {
    /// Position of the light source in the world.
    pub position: Point3,
    /// Color of the light source.
    pub color: Color,
    /// Brightness of the light source.
    pub intensity: f32,
    /// Whether or not this light should cast shadows.
    pub casts_shadows: bool,
}

impl Default for PointLight {
    fn default() -> Self {
        Self {
            position: Point3::from_point(0.0, 0.0, 0.0),
            color: Color::WHITE,
            intensity: 1.0,
            casts_shadows: true,
        }
    }
}

/// Computes the illumination of a surface point according to the Phong reflection model.
///
/// The `point` is given in world-space coordinates.
pub fn phong_lighting(
    object: &Object,
    light: &PointLight,
    point: &Point3,
    eye: &Vec3,
    normal: &Vec3,
    in_shadow: bool,
) -> Color {
    let material = object.material();

    // convert point to local-space coordinates
    let local_point = object.transform().inverse().unwrap() * point;

    // combine the surface color with the light's color/intensity
    let effective_color = material.color_at(&local_point) * light.color * light.intensity;

    // find the direction to the light source
    let lightv = (light.position - point).normalize();

    // compute the ambient contribution
    let ambient = effective_color * material.ambient;

    // early exit if the point is in shadow
    if in_shadow {
        return ambient;
    }

    let diffuse;
    let specular;

    // light_dot_normal is the cosine of the angle between the light and normal vectors.
    // A negative number means the light is on the other side of the surface.
    let light_dot_normal = lightv.dot(normal);

    if light_dot_normal < 0. {
        diffuse = Color::BLACK;
        specular = Color::BLACK;
    } else {
        // compute the diffuse contribution
        diffuse = effective_color * material.diffuse * light_dot_normal;

        // reflect_dot_eye is the cosine of the angle between the reflection and eye vectors.
        // A negative number means the light reflects away from the eye.
        let reflectv = (-lightv).reflect(normal);
        let reflect_dot_eye = reflectv.dot(eye);

        if reflect_dot_eye <= 0. {
            specular = Color::BLACK;
        } else {
            // compute the specular contribution
            let factor = reflect_dot_eye.powf(material.shininess);
            specular = light.color * light.intensity * material.specular * factor;
        }
    }
    // add the three contributions together to get the final shading
    ambient + diffuse + specular
}
