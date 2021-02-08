use std::convert::Infallible;

use cucumber_rust::{async_trait, gherkin::Step, given, then, World, WorldInit};
use trtc::math::MatrixN;

const EPSILON: f32 = 1e-6;

#[derive(WorldInit)]
pub struct TestRunner {
    m: MatrixN,
}

#[async_trait(?Send)]
impl World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            m: MatrixN::zeros(0),
        })
    }
}

#[given(regex = r"^the following (.*)x(?:.*) matrix M:$")]
async fn given_a_matrix(tr: &mut TestRunner, step: &Step, order: usize) {
    let table = step.table().unwrap();
    let rows = table
        .rows
        .iter()
        .flat_map(|row| row.iter().map(|v| v.parse::<f32>()))
        .collect::<Result<Vec<_>, _>>()
        .unwrap();

    tr.m = MatrixN::from_row_slice(order, &rows);
}

#[then(regex = r"^M\[(.*),(.*)\] = (.*)$")]
async fn matrix_element_equals(tr: &mut TestRunner, i: usize, j: usize, val: f32) {
    assert!((val - tr.m.get((i, j)).unwrap()).abs() < EPSILON);
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch03"]);
    runner.run_and_exit().await;
}
