use std::{
    slice::{Iter, IterMut},
    vec::IntoIter,
};

use itertools::Itertools;

use crate::{
    math::{Matrix, Point3, Vec3, EPSILON},
    rendering::{self, Color, Material, Pattern, PointLight},
    shape::Sphere,
};

use super::{Object, Ray, RayIntersection};

/// A handle to an object in a world.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct ObjectHandle(u32);

/// A container of collidable objects.
#[derive(Debug)]
pub struct World {
    objects: Vec<Object>,
    lights: Vec<PointLight>,
}

impl Default for World {
    fn default() -> Self {
        let mat = Material {
            pattern: Pattern::new(Color::new(0.8, 1.0, 0.6).into()),
            diffuse: 0.7,
            specular: 0.2,
            ..Default::default()
        };

        Self {
            objects: vec![
                Object::new_with_material(Sphere, Matrix::identity(4), mat),
                Object::new(Sphere, Matrix::from_scale(0.5, 0.5, 0.5)),
            ],
            lights: vec![PointLight {
                position: (-10., 10., -10.).into(),
                color: Color::WHITE,
                intensity: 1.,
                casts_shadows: true,
            }],
        }
    }
}

impl World {
    /// Creates an empty world.
    pub fn new() -> Self {
        Self {
            objects: Vec::new(),
            lights: Vec::new(),
        }
    }

    /// Adds an object to this world.
    pub fn add(&mut self, object: Object) -> ObjectHandle {
        self.objects.push(object);
        ObjectHandle(self.objects.len() as u32 - 1)
    }

    /// Returns a reference to the object identified by this handle.
    pub fn get(&self, handle: ObjectHandle) -> Option<&Object> {
        self.objects.get(handle.0 as usize)
    }

    /// Returns a mutable reference to the object identified by this handle.
    pub fn get_mut(&mut self, handle: ObjectHandle) -> Option<&mut Object> {
        self.objects.get_mut(handle.0 as usize)
    }

    /// Returns an iterator over this world's objects.
    pub fn objects(&self) -> Iter<Object> {
        self.objects.iter()
    }

    /// Returns a mutable iterator over this world's objects.
    pub fn objects_mut(&mut self) -> IterMut<Object> {
        self.objects.iter_mut()
    }

    /// Adds a new light source to this world.
    pub fn add_light(&mut self, light: PointLight) {
        self.lights.push(light);
    }

    /// Removes the first occurrence of `light` from this world.
    pub fn remove_light(&mut self, light: &PointLight) {
        if let Some((pos, _)) = self.lights.iter_mut().find_position(|l| l == &light) {
            self.lights.remove(pos);
        }
    }

    /// Returns an iterator over this world's lights.
    pub fn lights(&self) -> Iter<PointLight> {
        self.lights.iter()
    }

    /// Returns a mutable iterator over this world's lights.
    pub fn lights_mut(&mut self) -> IterMut<PointLight> {
        self.lights.iter_mut()
    }

    /// Computes the intersections between all the object in this world and a ray.
    ///
    /// The intersections returned by this method are sorted by time of impact in ascending order.
    pub fn interferences_with_ray<'a>(&'a self, ray: &'a Ray) -> InterferencesWithRay {
        InterferencesWithRay {
            ray,
            world: self,
            inner: self
                .handles()
                .map(move |hnd| {
                    let obj = self.get(hnd).unwrap();
                    (
                        hnd,
                        obj.shape()
                            .intersections_in_world_space(obj.transform(), ray),
                    )
                })
                .flat_map(|(obj, intersections)| intersections.map(move |i| (obj, i)))
                .sorted_unstable_by(|(_, x1), (_, x2)| x1.toi.partial_cmp(&x2.toi).unwrap()),
            containers: Vec::with_capacity(8),
        }
    }

    /// Recursively computes the color at the specified interference point.
    ///
    /// The recursion will be at most `remaining` deep. Returns `None` if the recursion limit is
    /// reached.
    pub fn shade_hit(&self, interference: &Interference, remaining: u32) -> Color {
        let obj = self
            .get(interference.handle)
            .expect("invalid object handle in interference");

        let surface = self.lights().fold(Color::BLACK, |surface, light| {
            surface
                + rendering::phong_lighting(
                    obj,
                    light,
                    &interference.over_point,
                    &interference.eye,
                    &interference.normal,
                    light.casts_shadows && self.is_in_shadow(&interference.over_point, light),
                )
        });

        let reflected = self.reflected_color(interference, remaining);
        let refracted = self.refracted_color(interference, remaining);

        let m = obj.material();
        if m.reflective > 0.0 && m.transparency > 0.0 {
            let reflectance = interference.schlick();
            surface + reflected * reflectance + refracted * (1.0 - reflectance)
        } else {
            surface + reflected + refracted
        }
    }

    /// Recursively computes the reflected color at the specified interference point.
    ///
    /// The recursion will be at most `remaining` deep. Returns `None` if the recursion limit is
    /// reached.
    pub fn reflected_color(&self, interference: &Interference, remaining: u32) -> Color {
        let obj = self
            .get(interference.handle)
            .expect("invalid object handle in interference");

        let reflective = obj.material().reflective;

        if remaining == 0 || reflective == 0.0 {
            Color::BLACK
        } else {
            let r = Ray::new(interference.over_point, interference.reflect);
            let c = self.color_at(&r, remaining - 1);
            c * reflective
        }
    }

    /// Recursively computes the refracted color at the specified interference point.
    ///
    /// The recursion will be at most `remaining` deep. Returns `None` if the recursion limit is
    /// reached.
    pub fn refracted_color(&self, interference: &Interference, remaining: u32) -> Color {
        let obj = self
            .get(interference.handle)
            .expect("invalid object handle in interference");

        let transparency = obj.material().transparency;

        if remaining == 0 || transparency == 0.0 {
            Color::BLACK
        } else {
            let n_ratio = interference.n1 / interference.n2;
            let cos_i = interference.eye.dot(&interference.normal);
            let sin2_t = n_ratio.powi(2) * (1.0 - cos_i.powi(2));

            if sin2_t > 1.0 {
                Color::BLACK
            } else {
                let cos_t = (1.0 - sin2_t).sqrt();
                let direction =
                    interference.normal * (n_ratio * cos_i - cos_t) - interference.eye * n_ratio;

                let r = Ray::new(interference.under_point, direction);
                let c = self.color_at(&r, remaining - 1);
                c * transparency
            }
        }
    }

    /// Recursively computes the color at the intersection between an object and a ray.
    ///
    /// The recursion will be at most `remaining` deep. Returns `None` if the recursion limit is
    /// reached.
    pub fn color_at(&self, ray: &Ray, remaining: u32) -> Color {
        if let Some(hit) = self.interferences_with_ray(ray).hit() {
            self.shade_hit(&hit, remaining)
        } else {
            Color::BLACK
        }
    }

    /// Checks whether the given point lies in shadow of the specified light source.
    pub fn is_in_shadow(&self, point: &Point3, light: &PointLight) -> bool {
        let v = light.position - point;
        let distance = v.length();
        let direction = v.normalize();

        let r = Ray::new(*point, direction);
        if let Some(hit) = self.interferences_with_ray(&r).hit_with_shadow() {
            hit.toi < distance
        } else {
            false
        }
    }

    fn handles(&self) -> impl Iterator<Item = ObjectHandle> {
        (0..self.objects.len()).map(|i| ObjectHandle(i as u32))
    }
}

/// An intersection between a world object and a ray.
#[derive(Debug, Clone)]
pub struct Interference {
    /// A handle to the object that was hit by the ray.
    pub handle: ObjectHandle,
    /// The time of impact of the ray with the object.
    pub toi: f32,
    /// The coordinates of the intersection.
    pub point: Point3,
    /// The point slightly above the intersection point along its normal.
    pub over_point: Point3,
    /// The point slightly below the intersection point along its normal.
    pub under_point: Point3,
    /// The vector from the intersection point towards the camera.
    pub eye: Vec3,
    /// The normal vector to the intesection point.
    pub normal: Vec3,
    /// The reflected ray after this interference.
    pub reflect: Vec3,
    /// Whether this intersection occurred on the object's inside.
    pub inside: bool,
    /// Refractive index of the material being exited by this intersection.
    pub n1: f32,
    /// Refractive index of the material being entered by this intersection.
    pub n2: f32,
}

impl Interference {
    /// Computes the reflectance at this intersection.
    pub fn schlick(&self) -> f32 {
        let mut cos = self.eye.dot(&self.normal);

        if self.n1 > self.n2 {
            let n = self.n1 / self.n2;
            let sin2_t = n.powi(2) * (1.0 - cos.powi(2));

            if sin2_t > 1.0 {
                return 1.0;
            }

            cos = (1.0 - sin2_t).sqrt();
        }

        let r0 = ((self.n1 - self.n2) / (self.n1 + self.n2)).powi(2);
        r0 + (1.0 - r0) * (1.0 - cos).powi(5)
    }
}

/// Iterator over all the objects in the world that intersect a specific ray.
#[derive(Debug, Clone)]
pub struct InterferencesWithRay<'a, 'b> {
    ray: &'a Ray,
    world: &'b World,
    inner: IntoIter<(ObjectHandle, RayIntersection)>,
    containers: Vec<ObjectHandle>,
}

impl<'a> InterferencesWithRay<'a, '_> {
    /// Returns the first intersection to have hit an object in the world.
    pub fn hit(mut self) -> Option<Interference> {
        self.find(|i| i.toi >= 0.)
    }

    /// Returns the first intersection to have hit an object in the world which casts a shadow.
    pub fn hit_with_shadow(mut self) -> Option<Interference> {
        let world = self.world;

        self.find(|i| i.toi >= 0. && world.get(i.handle).unwrap().casts_shadow())
    }

    /// Returns the refractive index of the last entered object, or `None` if no objects have been
    /// entered by this iterator yet.
    fn get_current_refractive_index(&self) -> Option<f32> {
        let hnd = self.containers.last()?;
        Some(self.world.get(*hnd)?.material().refractive_index)
    }
}

impl Iterator for InterferencesWithRay<'_, '_> {
    type Item = Interference;

    fn next(&mut self) -> Option<Self::Item> {
        self.inner.next().map(|(handle, i)| {
            let eye = -self.ray.dir;
            let inside = i.normal.dot(&eye) < 0.;
            let normal = if inside { -i.normal } else { i.normal };
            let reflect = self.ray.dir.reflect(&normal);
            let point = self.ray.point_at(i.toi);

            let n1 = self.get_current_refractive_index().unwrap_or(1.0);

            if self.containers.contains(&handle) {
                self.containers.retain(|elem| elem != &handle);
            } else {
                self.containers.push(handle);
            }

            let n2 = self.get_current_refractive_index().unwrap_or(1.0);

            Interference {
                handle,
                toi: i.toi,
                point,
                over_point: point + normal * EPSILON,
                under_point: point - normal * EPSILON,
                eye,
                normal,
                reflect,
                inside,
                n1,
                n2,
            }
        })
    }
}
