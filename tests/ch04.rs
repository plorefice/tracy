use std::convert::Infallible;

use cucumber_rust::{async_trait, given, then, World, WorldInit};
use trtc::math::{Coords, MatrixN};

const EPSILON: f32 = 1e-4;

#[derive(WorldInit)]
pub struct TestRunner {
    p: Coords,
    v: Coords,
    inv: MatrixN,
    transform: MatrixN,
}

#[async_trait(?Send)]
impl World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            p: Default::default(),
            v: Default::default(),
            inv: MatrixN::zeros(0),
            transform: MatrixN::zeros(0),
        })
    }
}

#[given(regex = r"transform ← translation\((.*), (.*), (.*)\)")]
async fn given_a_translation(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.transform = MatrixN::translation(x, y, z);
}

#[given(regex = r"transform ← scaling\((.*), (.*), (.*)\)")]
async fn given_a_scaling(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.transform = MatrixN::scale(x, y, z);
}

#[given("inv ← inverse(transform)")]
async fn given_the_inverse(tr: &mut TestRunner) {
    tr.inv = tr.transform.inverse().unwrap();
}

#[given(regex = r"p ← point\((.*), (.*), (.*)\)")]
async fn given_a_point(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.p = Coords::from_point(x, y, z);
}

#[given(regex = r"v ← vector\((.*), (.*), (.*)\)")]
async fn given_a_vector(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.v = Coords::from_vector(x, y, z);
}

#[then(regex = r"transform \* p = point\((.*), (.*), (.*)\)")]
async fn transform_by_point(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!((&tr.transform * tr.p).abs_diff_eq(&Coords::from_point(x, y, z), EPSILON));
}

#[then(regex = r"transform \* v = vector\((.*), (.*), (.*)\)")]
async fn transform_by_vector(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!((&tr.transform * tr.v).abs_diff_eq(&Coords::from_vector(x, y, z), EPSILON));
}

#[then("transform * v = v")]
async fn transform_by_vector_is_self(tr: &mut TestRunner) {
    assert!((&tr.transform * tr.v).abs_diff_eq(&tr.v, EPSILON));
}

#[then(regex = r"inv \* p = point\((.*), (.*), (.*)\)")]
async fn inverse_by_point(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!((&tr.inv * tr.p).abs_diff_eq(&Coords::from_point(x, y, z), EPSILON));
}

#[then(regex = r"inv \* v = vector\((.*), (.*), (.*)\)")]
async fn inverse_by_vector(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!((&tr.inv * tr.v).abs_diff_eq(&Coords::from_vector(x, y, z), EPSILON));
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch04"]);
    runner.run_and_exit().await;
}
