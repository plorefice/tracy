use std::convert::Infallible;

use cucumber_rust::{async_trait, given, then, World, WorldInit};
use trtc::math::{Coords, MatrixN};

const EPSILON: f32 = 1e-4;

#[derive(WorldInit)]
pub struct TestRunner {
    p: Coords,
    v: Coords,
    inv: MatrixN,
    translation: MatrixN,
}

#[async_trait(?Send)]
impl World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            p: Default::default(),
            v: Default::default(),
            inv: MatrixN::zeros(0),
            translation: MatrixN::zeros(0),
        })
    }
}

#[given(regex = r"transform ← translation\((.*), (.*), (.*)\)")]
async fn given_a_translation(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.translation = MatrixN::translation(x, y, z);
}

#[given("inv ← inverse(transform)")]
async fn given_the_inverse(tr: &mut TestRunner) {
    tr.inv = tr.translation.inverse().unwrap();
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
    assert!((&tr.translation * tr.p).abs_diff_eq(&Coords::from_point(x, y, z), EPSILON));
}

#[then("transform * v = v")]
async fn transform_by_vector(tr: &mut TestRunner) {
    assert!((&tr.translation * tr.v).abs_diff_eq(&tr.v, EPSILON));
}

#[then(regex = r"inv \* p = point\((.*), (.*), (.*)\)")]
async fn inverse_by_point(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!((&tr.inv * tr.p).abs_diff_eq(&Coords::from_point(x, y, z), EPSILON));
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch04"]);
    runner.run_and_exit().await;
}
