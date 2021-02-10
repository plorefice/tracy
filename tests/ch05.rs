use std::{convert::Infallible, f32};

use cucumber_rust::{async_trait, given, then, when, WorldInit};
use trtc::{
    math::Coords,
    query::{Ray, RayIntersection, RayIntersections, World, WorldHandle},
    shape::{ShapeHandle, Sphere},
};

const EPSILON: f32 = 1e-4;

#[derive(WorldInit)]
pub struct TestRunner {
    origin: Coords,
    direction: Coords,
    world: World,
    hnd: Option<WorldHandle>,
    ray: Ray,
    is: Vec<RayIntersection>,
    xs: Option<RayIntersections>,
    hit: Option<RayIntersection>,
}

#[async_trait(?Send)]
impl cucumber_rust::World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            origin: Coords::default(),
            direction: Coords::default(),
            world: World::default(),
            hnd: None,
            ray: Ray::default(),
            is: Vec::new(),
            xs: None,
            hit: None,
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

#[given("s ← sphere()")]
async fn given_a_sphere(tr: &mut TestRunner) {
    tr.hnd = Some(tr.world.add(ShapeHandle::new(Sphere)));
}

#[given(regex = r"(i[0-9]?) ← intersection\((.*), s\)")]
async fn given_ray_intersection(tr: &mut TestRunner, _id: String, toi: f32) {
    tr.is.push(RayIntersection { toi });
}

#[given(regex = r"xs ← intersections\((.*)\)")]
async fn given_a_bundle_of_intersections(tr: &mut TestRunner, ids: String) {
    let ids = ids
        .split(", ")
        .map(|id| id.trim_start_matches('i').parse::<usize>().unwrap() - 1)
        .collect::<Vec<_>>();

    let mut reordered = Vec::with_capacity(ids.len());
    for id in ids {
        reordered.push(tr.is[id].clone());
    }

    tr.xs = Some(RayIntersections::from(reordered.into_iter()));
}

#[when("r ← ray(origin, direction)")]
async fn build_ray(tr: &mut TestRunner) {
    tr.ray = Ray::new(tr.origin, tr.direction);
}

#[when("xs ← intersect(s, r)")]
async fn sphere_intersects_ray(tr: &mut TestRunner) {
    tr.xs = tr
        .world
        .interferences_with_ray(&tr.ray)
        .map(|(_, ri)| ri)
        .next();
}

#[when(regex = r"i ← intersection\((.*), s\)")]
async fn ray_intersection(tr: &mut TestRunner, toi: f32) {
    tr.is.push(RayIntersection { toi });
}

#[when(regex = r"xs ← intersections\((.*)\)")]
async fn bundle_intersections(tr: &mut TestRunner, ids: String) {
    let ids = ids
        .split(", ")
        .map(|id| id.trim_start_matches('i').parse::<usize>().unwrap() - 1)
        .collect::<Vec<_>>();

    let mut reordered = Vec::with_capacity(ids.len());
    for id in ids {
        reordered.push(tr.is[id].clone());
    }

    tr.xs = Some(RayIntersections::from(reordered.into_iter()));
}

#[when("i ← hit(xs)")]
async fn ray_hit(tr: &mut TestRunner) {
    tr.hit = tr.xs.as_ref().unwrap().clone().hit();
}

#[then("r.origin = origin")]
async fn check_ray_origin(tr: &mut TestRunner) {
    assert!(tr.ray.origin.abs_diff_eq(&tr.origin, EPSILON));
}

#[then("r.direction = direction")]
async fn check_ray_direction(tr: &mut TestRunner) {
    assert!(tr.ray.dir.abs_diff_eq(&tr.direction, EPSILON));
}

#[then(regex = r"position\(r, (.*)\) = point\((.*), (.*), (.*)\)")]
async fn check_ray_position(tr: &mut TestRunner, t: f32, x: f32, y: f32, z: f32) {
    assert!(tr
        .ray
        .point_at(t)
        .abs_diff_eq(&Coords::from_point(x, y, z), EPSILON));
}

#[then(regex = r"xs\.count = (.*)")]
async fn xs_count(tr: &mut TestRunner, n: usize) {
    if n == 0 {
        assert!(tr.xs.is_none())
    } else {
        assert_eq!(tr.xs.as_ref().unwrap().clone().count(), n);
    }
}

#[then(regex = r"xs\[(.*)\](?:\.t)? = (.*)")]
async fn xs_index_toi(tr: &mut TestRunner, i: usize, t: f32) {
    let intersections = tr.xs.as_ref().unwrap().clone().collect::<Vec<_>>();
    assert!((intersections[i].toi - t).abs() < EPSILON);
}

#[then(regex = r"xs\[(.*)\]\.object = s")]
async fn xs_index_object(_tr: &mut TestRunner, _i: usize) {
    // TODO: right now, `ShapeHandle` cannot be compared for equality
}

#[then(regex = r"i\.t = (.*)")]
async fn intersection_toi(tr: &mut TestRunner, toi: f32) {
    assert!((tr.is[0].toi - toi).abs() < EPSILON);
}

#[then("i.object = s")]
async fn intersection_object_is(_: &mut TestRunner) {}

#[then(regex = r"i = i(.*)")]
async fn check_hit(tr: &mut TestRunner, id: String) {
    let id = id.parse::<usize>().unwrap() - 1;
    assert!((tr.hit.as_ref().unwrap().toi - tr.is[id].toi).abs() < EPSILON);
}

#[then("i is nothing")]
async fn check_not_hit(tr: &mut TestRunner) {
    assert!(tr.hit.is_none());
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch05"]);
    runner.run_and_exit().await;
}
