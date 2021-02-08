use std::convert::Infallible;

use cucumber_rust::{async_trait, gherkin::Step, given, then, World, WorldInit};
use trtc::math::{Coords, MatrixN};

const EPSILON: f32 = 1e-6;

#[derive(WorldInit)]
pub struct TestRunner {
    a: MatrixN,
    b: MatrixN,
    tuple: Coords,
}

#[async_trait(?Send)]
impl World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            a: MatrixN::zeros(0),
            b: MatrixN::zeros(0),
            tuple: Default::default(),
        })
    }
}

fn parse_table_data(step: &Step) -> Vec<f32> {
    step.table()
        .unwrap()
        .rows
        .iter()
        .flat_map(|row| row.iter().map(|v| v.parse::<f32>()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
}

#[given("the following matrix A:")]
async fn given_matrix_a(tr: &mut TestRunner, step: &Step) {
    tr.a = MatrixN::from_row_slice(4, parse_table_data(step));
}

#[given("the following matrix B:")]
async fn given_matrix_b(tr: &mut TestRunner, step: &Step) {
    tr.b = MatrixN::from_row_slice(4, parse_table_data(step));
}

#[given(regex = r"^the following (.*)x(?:.*) matrix M:$")]
async fn given_a_matrix(tr: &mut TestRunner, step: &Step, order: usize) {
    tr.a = MatrixN::from_row_slice(order, parse_table_data(step));
}

#[given(regex = r".* ← tuple\((.*), (.*), (.*), (.*)\)")]
async fn given_a_tuple(tr: &mut TestRunner, x: f32, y: f32, z: f32, w: f32) {
    tr.tuple = Coords::from((x, y, z, w));
}

#[given("A ← transpose(identity_matrix)")]
async fn transpose_identity(tr: &mut TestRunner) {
    tr.a = MatrixN::identity(4).transpose();
}

#[then(regex = r"^M\[(.*),(.*)\] = (.*)$")]
async fn matrix_element_equals(tr: &mut TestRunner, i: usize, j: usize, val: f32) {
    assert!((val - tr.a.get((i, j)).unwrap()).abs() < EPSILON);
}

#[then("A = B")]
async fn a_eq_b(tr: &mut TestRunner) {
    assert!(tr.a.abs_diff_eq(&tr.b, EPSILON));
}

#[then("A != B")]
async fn a_ne_b(tr: &mut TestRunner) {
    assert!(!tr.a.abs_diff_eq(&tr.b, EPSILON));
}

#[then("A * B is the following 4x4 matrix:")]
async fn a_times_b(tr: &mut TestRunner, step: &Step) {
    let res = &tr.a * &tr.b;
    let exp = MatrixN::from_row_slice(4, parse_table_data(step));
    assert!(res.abs_diff_eq(&exp, EPSILON));
}

#[then("A * b = tuple(18, 24, 33, 1)")]
async fn a_times_tuple(tr: &mut TestRunner) {
    let res = &tr.a * tr.tuple;
    assert!(res.abs_diff_eq(&Coords::from((18., 24., 33., 1.)), EPSILON));
}

#[then("A * identity_matrix = A")]
async fn a_times_identity(tr: &mut TestRunner) {
    assert!((&tr.a * MatrixN::identity(tr.a.order())).abs_diff_eq(&tr.a, EPSILON));
}

#[then("identity_matrix * a = a")]
async fn identity_times_tuple(tr: &mut TestRunner) {
    assert!((MatrixN::identity(4) * tr.tuple).abs_diff_eq(&tr.tuple, EPSILON));
}

#[then("transpose(A) is the following matrix:")]
async fn transpose(tr: &mut TestRunner, step: &Step) {
    let exp = MatrixN::from_row_slice(4, parse_table_data(step));
    assert!(tr.a.transpose().abs_diff_eq(&exp, EPSILON));
}

#[then("A = identity_matrix")]
async fn is_identity(tr: &mut TestRunner) {
    assert!(tr.a.abs_diff_eq(&MatrixN::identity(tr.a.order()), EPSILON));
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch03"]);
    runner.run_and_exit().await;
}
