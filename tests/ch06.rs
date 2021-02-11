use std::{convert::Infallible, f32};

use cucumber_rust::{async_trait, given, then, when, WorldInit};
use trtc::{
    math::{Coords, MatrixN},
    query::{CollisionObject, CollisionObjectHandle, World},
    shape::{ShapeHandle, Sphere},
};

const EPSILON: f32 = 1e-4;

#[derive(WorldInit)]
pub struct TestRunner {
    world: World,
    s: Option<CollisionObjectHandle>,
    m: MatrixN,
    n: Coords,
}

#[async_trait(?Send)]
impl cucumber_rust::World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            world: World::new(),
            s: None,
            m: MatrixN::zeros(4),
            n: Coords::default(),
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

#[when(regex = r"n ← normal_at\(s, point\((.*), (.*), (.*)\)\)")]
async fn compute_normal(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.n = tr
        .world
        .get(tr.s.unwrap())
        .unwrap()
        .shape()
        .normal_at(&Coords::from_point(x, y, z));
}

#[then(regex = r"n = vector\((.*), (.*), (.*)\)")]
async fn check_normal(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!(tr.n.abs_diff_eq(&Coords::from_vector(x, y, z), EPSILON));
}

#[then("n = normalize(n)")]
async fn normals_are_normalized(tr: &mut TestRunner) {
    assert!(tr.n.abs_diff_eq(&tr.n.normalize(), EPSILON));
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch06"]);
    runner.run_and_exit().await;
}
