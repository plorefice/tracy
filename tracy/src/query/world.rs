use std::{
    slice::{Iter, IterMut},
    vec::IntoIter,
};

use itertools::Itertools;

use crate::{
    math::{MatrixN, Point, Vector, EPSILON},
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
    light: Option<PointLight>,
    objects: Vec<Object>,
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
            light: Some(PointLight {
                position: Point::from_point(-10., 10., -10.),
                color: Color::WHITE,
                intensity: 1.,
                casts_shadows: true,
            }),
            objects: vec![
                Object::new_with_material(Sphere, MatrixN::identity(4), mat),
                Object::new(Sphere, MatrixN::from_scale(0.5, 0.5, 0.5)),
            ],
        }
    }
}

impl World {
    /// Creates an empty world.
    pub fn new() -> Self {
        Self {
            light: None,
            objects: Vec::new(),
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

    /// Returns a reference to this world's light.
    pub fn light(&self) -> Option<&PointLight> {
        self.light.as_ref()
    }

    /// Updates this world's light.
    pub fn set_light(&mut self, light: PointLight) {
        self.light = Some(light);
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
    pub fn shade_hit(&self, interference: &Interference, remaining: u32) -> Option<Color> {
        let obj = self.get(interference.handle)?;
        let light = self.light()?;

        let surface = rendering::phong_lighting(
            obj,
            light,
            &interference.point,
            &interference.eye,
            &interference.normal,
            light.casts_shadows && self.is_in_shadow(&interference.over_point),
        );

        let reflected = self.reflected_color(interference, remaining)?;

        Some(surface + reflected)
    }

    /// Recursively computes the reflected color at the specified interference point.
    ///
    /// The recursion will be at most `remaining` deep. Returns `None` if the recursion limit is
    /// reached.
    pub fn reflected_color(&self, interference: &Interference, remaining: u32) -> Option<Color> {
        let obj = self.get(interference.handle)?;
        let reflective = obj.material().reflective;

        if remaining == 0 {
            None
        } else if reflective == 0.0 {
            Some(Color::BLACK)
        } else {
            let r = Ray::new(interference.over_point, interference.reflect);
            let c = self.color_at(&r, remaining - 1)?;
            Some(c * reflective)
        }
    }

    /// Recursively computes the color at the intersection between an object and a ray.
    ///
    /// The recursion will be at most `remaining` deep. Returns `None` if the recursion limit is
    /// reached.
    pub fn color_at(&self, ray: &Ray, remaining: u32) -> Option<Color> {
        self.shade_hit(&self.interferences_with_ray(ray).hit()?, remaining)
    }

    /// Checks whether the given point lies in shadow of the light source.
    pub fn is_in_shadow(&self, point: &Point) -> bool {
        if let Some(light) = self.light() {
            let v = light.position - point;
            let distance = v.length();
            let direction = v.normalize();

            let r = Ray::new(*point, direction);
            if let Some(hit) = self.interferences_with_ray(&r).hit() {
                return hit.toi < distance;
            }
        }
        false
    }

    fn handles(&self) -> impl Iterator<Item = ObjectHandle> {
        (0..self.objects.len()).map(|i| ObjectHandle(i as u32))
    }
}

/// An intersection between a world object and a ray.
#[derive(Debug)]
pub struct Interference {
    /// A handle to the object that was hit by the ray.
    pub handle: ObjectHandle,
    /// The time of impact of the ray with the object.
    pub toi: f32,
    /// The coordinates of the intersection.
    pub point: Point,
    /// The point slightly above the intersection point along its normal.
    pub over_point: Point,
    /// The point slightly below the intersection point along its normal.
    pub under_point: Point,
    /// The vector from the intersection point towards the camera.
    pub eye: Vector,
    /// The normal vector to the intesection point.
    pub normal: Vector,
    /// The reflected ray after this interference.
    pub reflect: Vector,
    /// Whether this intersection occurred on the object's inside.
    pub inside: bool,
    /// Refractive index of the material being exited by this intersection.
    pub n1: f32,
    /// Refractive index of the material being entered by this intersection.
    pub n2: f32,
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
