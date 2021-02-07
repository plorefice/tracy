use std::convert::Infallible;

use cucumber_rust::{async_trait, gherkin::Step, given, then, when, World, WorldInit};
use trtc::canvas::{Canvas, Color};

const EPSILON: f32 = 1e-6;

#[derive(Default, WorldInit)]
pub struct TestRunner {
    canvas: Canvas,
    ppm: String,
    c1: Color,
    c2: Color,
    c3: Color,
}

#[async_trait(?Send)]
impl World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self::default())
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

#[given(regex = r"^c3 ← color\((.*), (.*), (.*)\)$")]
async fn given_yet_another_color(tr: &mut TestRunner, r: f32, g: f32, b: f32) {
    tr.c3 = Color::new(r, g, b);
}

#[given(regex = r"^c ← canvas\((.*), (.*)\)$")]
async fn given_a_canvas(tr: &mut TestRunner, w: usize, h: usize) {
    tr.canvas = Canvas::new(w, h);
}

#[given("red ← color(1, 0, 0)")]
async fn given_the_red_color(_: &mut TestRunner) {}

#[when(regex = r"^write_pixel\(c, (.*), (.*), red\)$")]
async fn write_red_to_position(tr: &mut TestRunner, x: usize, y: usize) {
    tr.canvas.put(x, y, Color::new(1., 0., 0.));
}

#[when(regex = r"^write_pixel\(c, (.*), (.*), c(\d)\)$")]
async fn write_color_to_position(tr: &mut TestRunner, x: usize, y: usize, idx: String) {
    match idx.as_str() {
        "1" => tr.canvas.put(x, y, tr.c1),
        "2" => tr.canvas.put(x, y, tr.c2),
        "3" => tr.canvas.put(x, y, tr.c3),
        _ => unreachable!("invalid color index"),
    }
}

#[when("ppm ← canvas_to_ppm(c)")]
async fn convert_to_ppm(tr: &mut TestRunner) {
    tr.ppm = tr.canvas.convert_to_ppm();
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

#[then(regex = r"^pixel_at\(c, (.*), (.*)\) = red$")]
async fn pixel_is_red(tr: &mut TestRunner, x: usize, y: usize) {
    assert!(tr
        .canvas
        .get(x, y)
        .unwrap()
        .abs_diff_eq(&Color::new(1., 0., 0.), EPSILON));
}

#[then(regex = r"^lines (.*)-(.*) of ppm are$")]
async fn ppm_lines(tr: &mut TestRunner, step: &Step, start: usize, end: usize) {
    assert_eq!(
        tr.ppm
            .lines()
            .skip(start - 1)
            .take(end - start + 1)
            .collect::<Vec<_>>(),
        step.docstring()
            .unwrap()
            .lines()
            .skip(1)
            .collect::<Vec<_>>(),
    );
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch02"]);
    runner.run_and_exit().await;
}
