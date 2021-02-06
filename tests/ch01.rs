use std::{collections::HashMap, convert::Infallible, f32::EPSILON};

use cucumber_rust::{async_trait, given, then, World, WorldInit};
use trtc::Tuple;

#[derive(WorldInit)]
pub struct TestRunner {
    vars: HashMap<String, Tuple>,
}

#[async_trait(?Send)]
impl World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            vars: Default::default(),
        })
    }
}

#[given(regex = r"^([a-z0-9]+) ← tuple\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$")]
async fn given_a_tuple(tr: &mut TestRunner, var: String, x: f32, y: f32, z: f32, w: f32) {
    tr.vars.insert(var, (x, y, z, w).into());
}

#[given(regex = r"^([a-z0-9]+) ← point\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$")]
async fn given_a_point(tr: &mut TestRunner, var: String, x: f32, y: f32, z: f32) {
    tr.vars.insert(var, Tuple::from_point(x, y, z));
}

#[given(regex = r"^([a-z0-9]+) ← vector\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$")]
async fn given_a_vector(tr: &mut TestRunner, var: String, x: f32, y: f32, z: f32) {
    tr.vars.insert(var, Tuple::from_vector(x, y, z));
}

#[then(regex = r"^([a-z0-9]+).([xyzw]) = ([0-9.-]+)$")]
async fn tuple_field_equals_value(tr: &mut TestRunner, var: String, field: char, value: f32) {
    match field {
        'x' => assert!((tr.vars[&var].x - value).abs() < EPSILON),
        'y' => assert!((tr.vars[&var].y - value).abs() < EPSILON),
        'z' => assert!((tr.vars[&var].z - value).abs() < EPSILON),
        'w' => assert!((tr.vars[&var].w - value).abs() < EPSILON),
        _ => unreachable!("invalid tuple field"),
    }
}

#[then(regex = r"^([a-z0-9]+) is (not )?a point$")]
async fn tuple_is_point(tr: &mut TestRunner, var: String, not_str: String) {
    assert_eq!(tr.vars[&var].is_point(), not_str.is_empty())
}

#[then(regex = r"^([a-z0-9]+) is (not )?a vector$")]
async fn tuple_is_vector(tr: &mut TestRunner, var: String, not_str: String) {
    assert_eq!(tr.vars[&var].is_vector(), not_str.is_empty())
}

#[then(
    regex = r"^([a-z0-9]+) \+ ([a-z0-9]+) = tuple\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$"
)]
async fn tuple_sum_equals(
    tr: &mut TestRunner,
    a1: String,
    a2: String,
    x: f32,
    y: f32,
    z: f32,
    w: f32,
) {
    assert_eq!(tr.vars[&a1] + tr.vars[&a2], Tuple::from((x, y, z, w)));
}

#[then(regex = r"^([a-z0-9]+) = tuple\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$")]
async fn point_is_tuple(tr: &mut TestRunner, var: String, x: f32, y: f32, z: f32, w: f32) {
    assert_eq!(tr.vars[&var], Tuple::from((x, y, z, w)));
}

#[then(regex = r"^([a-z0-9]+) = tuple\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$")]
async fn vector_is_tuple(tr: &mut TestRunner, var: String, x: f32, y: f32, z: f32, w: f32) {
    assert_eq!(tr.vars[&var], Tuple::from((x, y, z, w)));
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features"]);
    runner.run_and_exit().await;
}
