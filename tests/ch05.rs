use std::{convert::Infallible, f32};

use cucumber_rust::{async_trait, given, then, when, World, WorldInit};
use trtc::math::{Coords, Ray};

const EPSILON: f32 = 1e-4;

#[derive(WorldInit)]
pub struct TestRunner {
    origin: Coords,
    direction: Coords,
    ray: Ray,
}

#[async_trait(?Send)]
impl World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            origin: Coords::default(),
            direction: Coords::default(),
            ray: Ray::default(),
        })
    }
}

#[given(regex = r"origin ← point\((.*), (.*), (.*)\)")]
async fn given_an_origin(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.origin = Coords::from_point(x, y, z);
}

#[given(regex = r"direction ← vector\((.*), (.*), (.*)\)")]
async fn given_a_direction(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.direction = Coords::from_vector(x, y, z);
}

#[given(regex = r"r ← ray\(point\((.*), (.*), (.*)\), vector\((.*), (.*), (.*)\)\)")]
async fn given_a_ray(tr: &mut TestRunner, px: f32, py: f32, pz: f32, vx: f32, vy: f32, vz: f32) {
    tr.ray = Ray::new(
        Coords::from_point(px, py, pz),
        Coords::from_vector(vx, vy, vz),
    );
}

#[when("r ← ray(origin, direction)")]
async fn build_ray(tr: &mut TestRunner) {
    tr.ray = Ray::new(tr.origin, tr.direction);
}

#[then("r.origin = origin")]
async fn check_ray_origin(tr: &mut TestRunner) {
    assert!(tr.ray.origin().abs_diff_eq(&tr.origin, EPSILON));
}

#[then("r.direction = direction")]
async fn check_ray_direction(tr: &mut TestRunner) {
    assert!(tr.ray.direction().abs_diff_eq(&tr.direction, EPSILON));
}

#[then(regex = r"position\(r, (.*)\) = point\((.*), (.*), (.*)\)")]
async fn check_ray_position(tr: &mut TestRunner, t: f32, x: f32, y: f32, z: f32) {
    assert!(tr
        .ray
        .position(t)
        .abs_diff_eq(&Coords::from_point(x, y, z), EPSILON));
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch05"]);
    runner.run_and_exit().await;
}
