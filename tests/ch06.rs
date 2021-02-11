use std::{convert::Infallible, f32};

use cucumber_rust::{async_trait, given, then, when, WorldInit};
use trtc::{
    canvas::Color,
    math::{MatrixN, Point, Vector},
    query::{CollisionObject, CollisionObjectHandle, Ray, RayCast, World},
    rendering::PointLight,
    shape::{ShapeHandle, Sphere},
};

const EPSILON: f32 = 1e-4;

#[derive(WorldInit)]
pub struct TestRunner {
    world: World,
    s: Option<CollisionObjectHandle>,
    m: MatrixN,
    v: Vector,
    n: Vector,
    r: Vector,
    ns: Vec<Vector>,

    light: PointLight,
    color: Color,
    position: Point,
}

#[async_trait(?Send)]
impl cucumber_rust::World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            world: World::new(),
            s: None,
            m: MatrixN::zeros(4),
            v: Vector::default(),
            n: Vector::default(),
            r: Vector::default(),
            ns: Vec::new(),

            light: PointLight::default(),
            color: Color::default(),
            position: Point::default(),
        })
    }
}

#[given("s ← sphere()")]
async fn given_a_sphere(tr: &mut TestRunner) {
    tr.s = Some(tr.world.add(CollisionObject::new(
        ShapeHandle::new(Sphere),
        MatrixN::identity(4),
    )));
}

#[given("set_transform(s, m)")]
async fn given_a_transformed_sphere(tr: &mut TestRunner) {
    let co = tr.world.get_mut(tr.s.unwrap()).unwrap();
    co.set_transform(tr.m.clone());
}

#[given(regex = r"set_transform\(s, translation\((.*), (.*), (.*)\)\)")]
async fn given_a_translated_sphere(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    let co = tr.world.get_mut(tr.s.unwrap()).unwrap();
    co.set_transform(MatrixN::from_translation(x, y, z));
}

#[given(regex = r"m ← scaling\((.*), (.*), (.*)\) \* rotation_z\((.*)\)")]
async fn given_a_scale_and_rotation(tr: &mut TestRunner, x: f32, y: f32, z: f32, rad: f32) {
    tr.m = MatrixN::from_scale(x, y, z) * MatrixN::from_rotation_z(rad);
}

#[given(regex = r"v ← vector\((.*), (.*), (.*)\)")]
async fn given_a_vector(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.v = Vector::from_vector(x, y, z);
}

#[given(regex = r"n ← vector\((.*), (.*), (.*)\)")]
async fn given_a_normal(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.n = Vector::from_vector(x, y, z);
}

#[given(regex = r"intensity ← color\((.*), (.*), (.*)\)")]
async fn given_light_intensity(tr: &mut TestRunner, r: f32, g: f32, b: f32) {
    tr.color = Color::new(r, g, b);
}

#[given(regex = r"position ← point\((.*), (.*), (.*)\)")]
async fn given_light_position(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.position = Point::from_point(x, y, z);
}

#[when(regex = r"n ← normal_at\(s, point\((.*), (.*), (.*)\)\)")]
async fn compute_normal(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    let co = tr.world.get(tr.s.unwrap()).unwrap();

    tr.ns = co
        .shape()
        .toi_and_normal_with_ray(
            co.transform(),
            &Ray::new(Point::default(), Vector::from_vector(x, y, z)),
        )
        .map(|xs| xs.map(|x| x.normal).collect())
        .unwrap_or_default();
}

#[when("r ← reflect(v, n)")]
async fn reflect_vector(tr: &mut TestRunner) {
    tr.r = tr.v.reflect(&tr.n);
}

#[when("light ← point_light(position, intensity)")]
async fn create_point_light(tr: &mut TestRunner) {
    tr.light = PointLight {
        position: tr.position,
        color: tr.color,
        intensity: 1.,
    };
}

#[then(regex = r"n = vector\((.*), (.*), (.*)\)")]
async fn check_normal(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!(tr
        .ns
        .iter()
        .any(|n| n.abs_diff_eq(&Vector::from_vector(x, y, z), EPSILON)));
}

#[then("n = normalize(n)")]
async fn normals_are_normalized(tr: &mut TestRunner) {
    assert!(tr.ns.iter().all(|n| n.abs_diff_eq(&n.normalize(), EPSILON)));
}

#[then(regex = r"r = vector\((.*), (.*), (.*)\)")]
async fn check_reflection(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!(tr.r.abs_diff_eq(&Vector::from_vector(x, y, z), EPSILON));
}

#[then("light.position = position")]
async fn check_light_position(tr: &mut TestRunner) {
    assert!(tr.light.position.abs_diff_eq(&tr.position, EPSILON));
}

#[then("light.intensity = intensity")]
async fn check_light_intensity(tr: &mut TestRunner) {
    assert!(tr.light.color.abs_diff_eq(&tr.color, EPSILON));
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch06"]);
    runner.run_and_exit().await;
}
