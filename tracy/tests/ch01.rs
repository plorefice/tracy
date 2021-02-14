#![allow(clippy::many_single_char_names)]

use std::{collections::HashMap, convert::Infallible};

use cucumber_rust::{async_trait, given, then, when, World, WorldInit};
use tracy::math::{Coords, Point, Vector};

const EPSILON: f32 = 1e-6;

#[derive(WorldInit)]
pub struct TestRunner {
    vars: HashMap<String, Coords>,
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
    tr.vars.insert(var, Point::from_point(x, y, z));
}

#[given(regex = r"^([a-z0-9]+) ← vector\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$")]
async fn given_a_vector(tr: &mut TestRunner, var: String, x: f32, y: f32, z: f32) {
    tr.vars.insert(var, Vector::from_vector(x, y, z));
}

#[when(regex = r"^([a-z0-9]+) ← normalize\(([a-z0-9]+)\)$")]
async fn normalized(tr: &mut TestRunner, to: String, from: String) {
    tr.vars.insert(to, tr.vars[&from].normalize());
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

#[then(regex = r"^([a-z0-9]+) = tuple\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$")]
async fn point_is_tuple(tr: &mut TestRunner, var: String, x: f32, y: f32, z: f32, w: f32) {
    assert!(tr.vars[&var].abs_diff_eq(&Coords::from((x, y, z, w)), EPSILON));
}

#[then(regex = r"^([a-z0-9]+) = tuple\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$")]
async fn vector_is_tuple(tr: &mut TestRunner, var: String, x: f32, y: f32, z: f32, w: f32) {
    assert!(tr.vars[&var].abs_diff_eq(&Coords::from((x, y, z, w)), EPSILON));
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
    assert!((tr.vars[&a1] + tr.vars[&a2]).abs_diff_eq(&Coords::from((x, y, z, w)), EPSILON));
}

#[then(regex = r"^([a-z0-9]+) - ([a-z0-9]+) = vector\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$")]
async fn tuple_sub_equals_vector(
    tr: &mut TestRunner,
    a1: String,
    a2: String,
    x: f32,
    y: f32,
    z: f32,
) {
    assert!((tr.vars[&a1] - tr.vars[&a2]).abs_diff_eq(&Vector::from_vector(x, y, z), EPSILON));
}

#[then(regex = r"^([a-z0-9]+) - ([a-z0-9]+) = point\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$")]
async fn tuple_sub_equals_point(
    tr: &mut TestRunner,
    a1: String,
    a2: String,
    x: f32,
    y: f32,
    z: f32,
) {
    assert!((tr.vars[&a1] - tr.vars[&a2]).abs_diff_eq(&Point::from_point(x, y, z), EPSILON));
}

#[then(regex = r"^-([a-z0-9]+) = tuple\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$")]
async fn tuple_negation(tr: &mut TestRunner, a: String, x: f32, y: f32, z: f32, w: f32) {
    assert!((-tr.vars[&a]).abs_diff_eq(&Coords::from((x, y, z, w)), EPSILON));
}

#[then(
    regex = r"^([a-z0-9]+) \* ([0-9.-]+) = tuple\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$"
)]
async fn tuple_by_scalar(tr: &mut TestRunner, a: String, s: f32, x: f32, y: f32, z: f32, w: f32) {
    assert!((tr.vars[&a] * s).abs_diff_eq(&Coords::from((x, y, z, w)), EPSILON));
}

#[then(
    regex = r"^([a-z0-9]+) / ([0-9.-]+) = tuple\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$"
)]
async fn tuple_div_scalar(tr: &mut TestRunner, a: String, s: f32, x: f32, y: f32, z: f32, w: f32) {
    assert!((tr.vars[&a] / s).abs_diff_eq(&Coords::from((x, y, z, w)), EPSILON));
}

#[then(regex = r"^magnitude\(([a-z0-9]+)\) = (√)?([0-9.-]+)$")]
async fn vector_magnitude(tr: &mut TestRunner, a: String, sqrt: String, mut mag: f32) {
    if !sqrt.is_empty() {
        mag = mag.sqrt();
    }
    assert!((tr.vars[&a].length() - mag).abs() < EPSILON);
}

#[then(
    regex = r"^normalize\(([a-z0-9]+)\) = (?:approximately )?vector\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$"
)]
async fn vector_normalize(tr: &mut TestRunner, a: String, x: f32, y: f32, z: f32) {
    assert!((tr.vars[&a].normalize() - Vector::from_vector(x, y, z)).length() < 1e6);
}

#[then(regex = r"^dot\(([a-z0-9]+), ([a-z0-9]+)\) = ([0-9.-]+)$")]
async fn dot_product(tr: &mut TestRunner, a: String, b: String, dot: f32) {
    assert!((tr.vars[&a].dot(&tr.vars[&b]) - dot).abs() < EPSILON);
}

#[then(
    regex = r"^cross\(([a-z0-9]+), ([a-z0-9]+)\) = vector\(([0-9.-]+), ([0-9.-]+), ([0-9.-]+)\)$"
)]
async fn cross_product(tr: &mut TestRunner, a: String, b: String, x: f32, y: f32, z: f32) {
    assert!(tr.vars[&a]
        .cross(&tr.vars[&b])
        .abs_diff_eq(&Vector::from_vector(x, y, z), EPSILON));
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch01"]);
    runner.run_and_exit().await;
}
