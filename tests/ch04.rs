use std::{convert::Infallible, f32};

use cucumber_rust::{async_trait, given, then, World, WorldInit};
use trtc::math::{Coords, MatrixN};

const EPSILON: f32 = 1e-4;

#[derive(WorldInit)]
pub struct TestRunner {
    p: Coords,
    v: Coords,
    inv: MatrixN,
    transform: MatrixN,
    hq: MatrixN,
    fq: MatrixN,
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
            hq: MatrixN::zeros(0),
            fq: MatrixN::zeros(0),
        })
    }
}

#[given(regex = r"transform ← translation\((.*), (.*), (.*)\)")]
async fn given_a_translation(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.transform = MatrixN::from_translation(x, y, z);
}

#[given(regex = r"transform ← scaling\((.*), (.*), (.*)\)")]
async fn given_a_scaling(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.transform = MatrixN::from_scale(x, y, z);
}

#[given(regex = r"half_quarter ← rotation_([xyz])\(π / 4\)")]
async fn given_a_half_quarter_rotation(tr: &mut TestRunner, axis: String) {
    tr.hq = match axis.as_str() {
        "x" => MatrixN::from_rotation_x(f32::consts::PI / 4.),
        "y" => MatrixN::from_rotation_y(f32::consts::PI / 4.),
        "z" => MatrixN::from_rotation_z(f32::consts::PI / 4.),
        _ => unreachable!("invalid rotation axis"),
    };
}

#[given(regex = r"full_quarter ← rotation_([xyz])\(π / 2\)")]
async fn given_a_full_quarter_rotation(tr: &mut TestRunner, axis: String) {
    tr.fq = match axis.as_str() {
        "x" => MatrixN::from_rotation_x(f32::consts::PI / 2.),
        "y" => MatrixN::from_rotation_y(f32::consts::PI / 2.),
        "z" => MatrixN::from_rotation_z(f32::consts::PI / 2.),
        _ => unreachable!("invalid rotation axis"),
    };
}

#[given(regex = r"transform ← shearing\((.*), (.*), (.*), (.*), (.*), (.*)\)")]
async fn given_a_shearing(
    tr: &mut TestRunner,
    xy: f32,
    xz: f32,
    yx: f32,
    yz: f32,
    zx: f32,
    zy: f32,
) {
    tr.transform = MatrixN::from_shear(xy, xz, yx, yz, zx, zy);
}

#[given("inv ← inverse(transform)")]
async fn given_the_inverse_of_a_transform(tr: &mut TestRunner) {
    tr.inv = tr.transform.inverse().unwrap();
}

#[given("inv ← inverse(half_quarter)")]
async fn given_the_inverse_of_half_quarter(tr: &mut TestRunner) {
    tr.inv = tr.hq.inverse().unwrap();
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

#[then(regex = r"half_quarter \* p = point\((.*), (.*), (.*)\)")]
async fn half_quarter_by_point(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!((&tr.hq * tr.p).abs_diff_eq(&Coords::from_point(x, y, z), EPSILON));
}

#[then(regex = r"full_quarter \* p = point\((.*), (.*), (.*)\)")]
async fn full_quarter_by_point(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!((&tr.fq * tr.p).abs_diff_eq(&Coords::from_point(x, y, z), EPSILON));
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch04"]);
    runner.run_and_exit().await;
}
