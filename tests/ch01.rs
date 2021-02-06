use std::{convert::Infallible, f32::EPSILON};

use cucumber_rust::{async_trait, given, then, World, WorldInit};
use trtc::Tuple;

#[derive(WorldInit)]
pub struct Runner {
    a: Tuple,
    p: Tuple,
    v: Tuple,
}

#[async_trait(?Send)]
impl World for Runner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            a: Default::default(),
            p: Default::default(),
            v: Default::default(),
        })
    }
}

#[given(regex = r"a ← tuple\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)")]
async fn given_a_tuple(runner: &mut Runner, x: f32, y: f32, z: f32, w: f32) {
    runner.a = (x, y, z, w).into();
}

#[given(regex = r"p ← point\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)")]
async fn given_a_point(runner: &mut Runner, x: f32, y: f32, z: f32) {
    runner.p = Tuple::from_point(x, y, z);
}

#[given(regex = r"v ← vector\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)")]
async fn given_a_vector(runner: &mut Runner, x: f32, y: f32, z: f32) {
    runner.v = Tuple::from_vector(x, y, z);
}

#[then(regex = r"a.(\w) = ([0-9.-]+)")]
async fn tuple_field_equals_value(runner: &mut Runner, field: char, value: f32) {
    match field {
        'x' => assert!((runner.a.x - value).abs() < EPSILON),
        'y' => assert!((runner.a.y - value).abs() < EPSILON),
        'z' => assert!((runner.a.z - value).abs() < EPSILON),
        'w' => assert!((runner.a.w - value).abs() < EPSILON),
        _ => unreachable!("invalid tuple field"),
    }
}

#[then(regex = r"a is (not )?a point")]
async fn tuple_is_point(runner: &mut Runner, not_str: String) {
    assert_eq!(runner.a.is_point(), not_str.is_empty())
}

#[then(regex = r"a is (not )?a vector")]
async fn tuple_is_vector(runner: &mut Runner, not_str: String) {
    assert_eq!(runner.a.is_vector(), not_str.is_empty())
}

#[then(regex = r"p = tuple\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)")]
async fn point_is_tuple(runner: &mut Runner, x: f32, y: f32, z: f32, w: f32) {
    assert_eq!(runner.p, Tuple::from((x, y, z, w)));
}

#[then(regex = r"v = tuple\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)")]
async fn vector_is_tuple(runner: &mut Runner, x: f32, y: f32, z: f32, w: f32) {
    assert_eq!(runner.v, Tuple::from((x, y, z, w)));
}

#[tokio::main]
async fn main() {
    let runner = Runner::init(&["./features"]);
    runner.run_and_exit().await;
}
