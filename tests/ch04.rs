use std::{convert::Infallible, f32};

use cucumber_rust::{async_trait, given, then, when, World, WorldInit};
use trtc::math::{MatrixN, Point, Vector};

const EPSILON: f32 = 1e-4;

#[derive(WorldInit)]
pub struct TestRunner {
    p: Point,
    p2: Point,
    p3: Point,
    p4: Point,
    v: Vector,

    inv: MatrixN,
    transform: MatrixN,
    hq: MatrixN,
    fq: MatrixN,
    a: MatrixN,
    b: MatrixN,
    c: MatrixN,
}

#[async_trait(?Send)]
impl World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            p: Default::default(),
            p2: Default::default(),
            p3: Default::default(),
            p4: Default::default(),
            v: Default::default(),

            inv: MatrixN::zeros(0),
            transform: MatrixN::zeros(0),
            hq: MatrixN::zeros(0),
            fq: MatrixN::zeros(0),
            a: MatrixN::zeros(0),
            b: MatrixN::zeros(0),
            c: MatrixN::zeros(0),
        })
    }
}

#[given(regex = r"(transform|C)? ← translation\((.*), (.*), (.*)\)")]
async fn given_a_translation(tr: &mut TestRunner, which: String, x: f32, y: f32, z: f32) {
    match which.as_str() {
        "transform" => tr.transform = MatrixN::from_translation(x, y, z),
        "C" => tr.c = MatrixN::from_translation(x, y, z),
        _ => unreachable!("unexpected variable name"),
    }
}

#[given(regex = r"(transform|B)? ← scaling\((.*), (.*), (.*)\)")]
async fn given_a_scaling(tr: &mut TestRunner, which: String, x: f32, y: f32, z: f32) {
    match which.as_str() {
        "transform" => tr.transform = MatrixN::from_scale(x, y, z),
        "B" => tr.b = MatrixN::from_scale(x, y, z),
        _ => unreachable!("unexpected variable name"),
    }
}

#[given("A ← rotation_x(π / 2)")]
async fn given_a_rotation(tr: &mut TestRunner) {
    tr.a = MatrixN::from_rotation_x(f32::consts::PI / 2.);
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
    tr.p = Point::from_point(x, y, z);
}

#[given(regex = r"v ← vector\((.*), (.*), (.*)\)")]
async fn given_a_vector(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.v = Vector::from_vector(x, y, z);
}

#[when(regex = r"(p[234]) ← ([ABC]) \* (p[234]?)")]
async fn point_by_matrix(tr: &mut TestRunner, dst: String, mat: String, src: String) {
    let (src, dst) = match (src.as_str(), dst.as_str()) {
        ("p", "p2") => (&tr.p, &mut tr.p2),
        ("p2", "p3") => (&tr.p2, &mut tr.p3),
        ("p3", "p4") => (&tr.p3, &mut tr.p4),
        _ => unreachable!("unexpected src/dst pair"),
    };

    let mat = match mat.as_str() {
        "A" => &tr.a,
        "B" => &tr.b,
        "C" => &tr.c,
        _ => unreachable!("unexpected matrix name"),
    };

    *dst = mat * *src;
}

#[when("T ← C * B * A")]
async fn transform_chain(tr: &mut TestRunner) {
    tr.transform = &tr.c * &tr.b * &tr.a;
}

#[then(regex = r"(?:transform|T) \* p = point\((.*), (.*), (.*)\)")]
async fn transform_by_point(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!((&tr.transform * tr.p).abs_diff_eq(&Point::from_point(x, y, z), EPSILON));
}

#[then(regex = r"transform \* v = vector\((.*), (.*), (.*)\)")]
async fn transform_by_vector(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!((&tr.transform * tr.v).abs_diff_eq(&Vector::from_vector(x, y, z), EPSILON));
}

#[then("transform * v = v")]
async fn transform_by_vector_is_self(tr: &mut TestRunner) {
    assert!((&tr.transform * tr.v).abs_diff_eq(&tr.v, EPSILON));
}

#[then(regex = r"inv \* p = point\((.*), (.*), (.*)\)")]
async fn inverse_by_point(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!((&tr.inv * tr.p).abs_diff_eq(&Point::from_point(x, y, z), EPSILON));
}

#[then(regex = r"inv \* v = vector\((.*), (.*), (.*)\)")]
async fn inverse_by_vector(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!((&tr.inv * tr.v).abs_diff_eq(&Vector::from_vector(x, y, z), EPSILON));
}

#[then(regex = r"half_quarter \* p = point\((.*), (.*), (.*)\)")]
async fn half_quarter_by_point(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!((&tr.hq * tr.p).abs_diff_eq(&Point::from_point(x, y, z), EPSILON));
}

#[then(regex = r"full_quarter \* p = point\((.*), (.*), (.*)\)")]
async fn full_quarter_by_point(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!((&tr.fq * tr.p).abs_diff_eq(&Point::from_point(x, y, z), EPSILON));
}

#[then(regex = r"(p[234]) = point\((.*), (.*), (.*)\)")]
async fn point_equals(tr: &mut TestRunner, which: String, x: f32, y: f32, z: f32) {
    let p = match which.as_str() {
        "p2" => &tr.p2,
        "p3" => &tr.p3,
        "p4" => &tr.p4,
        _ => unreachable!("unexpected point name"),
    };

    assert!(p.abs_diff_eq(&Point::from_point(x, y, z), EPSILON));
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch04"]);
    runner.run_and_exit().await;
}
