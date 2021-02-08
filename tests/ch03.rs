use std::convert::Infallible;

use cucumber_rust::{async_trait, gherkin::Step, given, then, World, WorldInit};
use trtc::math::MatrixN;

const EPSILON: f32 = 1e-6;

#[derive(WorldInit)]
pub struct TestRunner {
    a: MatrixN,
    b: MatrixN,
}

#[async_trait(?Send)]
impl World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            a: MatrixN::zeros(0),
            b: MatrixN::zeros(0),
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
async fn a_mul_b(tr: &mut TestRunner, step: &Step) {
    let res = &tr.a * &tr.b;
    let exp = MatrixN::from_row_slice(4, parse_table_data(step));
    assert!(res.abs_diff_eq(&exp, EPSILON));
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch03"]);
    runner.run_and_exit().await;
}
