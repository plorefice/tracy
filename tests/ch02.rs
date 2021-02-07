use std::convert::Infallible;

use cucumber_rust::{async_trait, given, then, World, WorldInit};
use trtc::canvas::{Canvas, Color};

const EPSILON: f32 = 1e-6;

#[derive(WorldInit)]
pub struct TestRunner {
    canvas: Canvas,
    c1: Color,
    c2: Color,
}

#[async_trait(?Send)]
impl World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            canvas: Canvas::default(),
            c1: Default::default(),
            c2: Default::default(),
        })
    }
}

#[given(regex = r"^c1? ← color\((.*), (.*), (.*)\)$")]
async fn given_a_color(tr: &mut TestRunner, r: f32, g: f32, b: f32) {
    tr.c1 = Color::new(r, g, b);
}

#[given(regex = r"^c2 ← color\((.*), (.*), (.*)\)$")]
async fn given_another_color(tr: &mut TestRunner, r: f32, g: f32, b: f32) {
    tr.c2 = Color::new(r, g, b);
}

#[given(regex = r"^c ← canvas\((.*), (.*)\)$")]
async fn given_a_canvas(tr: &mut TestRunner, w: usize, h: usize) {
    tr.canvas = Canvas::new(w, h);
}

#[then(regex = r"^c.(red|green|blue) = (.*)$")]
async fn color_component_equals(tr: &mut TestRunner, which: String, val: f32) {
    match which.as_str() {
        "red" => assert!((tr.c1.r - val).abs() < EPSILON),
        "green" => assert!((tr.c1.g - val).abs() < EPSILON),
        "blue" => assert!((tr.c1.b - val).abs() < EPSILON),
        _ => unreachable!("invalid color component"),
    }
}

#[then(regex = r"^c.(width|height) = (.*)$")]
async fn canvas_dimension_equals(tr: &mut TestRunner, dim: String, size: usize) {
    match dim.as_str() {
        "width" => assert_eq!(tr.canvas.width(), size),
        "height" => assert_eq!(tr.canvas.height(), size),
        _ => unreachable!("invalid color component"),
    }
}

#[then(regex = r"^c1 \+ c2 = color\((.*), (.*), (.*)\)$")]
async fn color_addition(tr: &mut TestRunner, r: f32, g: f32, b: f32) {
    assert!((tr.c1 + tr.c2).abs_diff_eq(&Color::new(r, g, b), EPSILON));
}

#[then(regex = r"^c1 - c2 = color\((.*), (.*), (.*)\)$")]
async fn color_subtraction(tr: &mut TestRunner, r: f32, g: f32, b: f32) {
    assert!((tr.c1 - tr.c2).abs_diff_eq(&Color::new(r, g, b), EPSILON));
}

#[then(regex = r"^c \* 2 = color\((.*), (.*), (.*)\)$")]
async fn color_by_scalar(tr: &mut TestRunner, r: f32, g: f32, b: f32) {
    assert!((tr.c1 * 2.).abs_diff_eq(&Color::new(r, g, b), EPSILON));
}

#[then(regex = r"^c1 \* c2 = color\((.*), (.*), (.*)\)$")]
async fn color_by_color(tr: &mut TestRunner, r: f32, g: f32, b: f32) {
    assert!((tr.c1 * tr.c2).abs_diff_eq(&Color::new(r, g, b), EPSILON));
}

#[then(regex = r"^every pixel of c is color\((.*), (.*), (.*)\)$")]
async fn canvas_fill(tr: &mut TestRunner, r: f32, g: f32, b: f32) {
    assert!(tr
        .canvas
        .iter()
        .all(|c| c.abs_diff_eq(&Color::new(r, g, b), EPSILON)));
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch02"]);
    runner.run_and_exit().await;
}
