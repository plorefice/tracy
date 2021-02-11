use std::{convert::Infallible, f32};

use cucumber_rust::{async_trait, given, then, when, WorldInit};
use trtc::{
    math::{MatrixN, Point, Vector},
    query::{Object, Ray, RayCast, RayIntersection, RayIntersections},
    shape::{ShapeHandle, Sphere},
};

const EPSILON: f32 = 1e-4;

#[derive(WorldInit)]
pub struct TestRunner {
    origin: Point,
    direction: Vector,
    sphere: Option<Object>,
    r1: Ray,
    r2: Ray,
    is: Vec<f32>,
    xs: Vec<f32>,
    hit: Option<f32>,
    m: MatrixN,
}

#[async_trait(?Send)]
impl cucumber_rust::World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            origin: Point::default(),
            direction: Vector::default(),
            sphere: None,
            r1: Ray {
                origin: Point::default(),
                dir: Vector::default(),
            },
            r2: Ray {
                origin: Point::default(),
                dir: Vector::default(),
            },
            is: Vec::new(),
            xs: Vec::new(),
            hit: None,
            m: MatrixN::identity(4),
        })
    }
}

#[given(regex = r"origin ← point\((.*), (.*), (.*)\)")]
async fn given_an_origin(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.origin = Point::from_point(x, y, z);
}

#[given(regex = r"direction ← vector\((.*), (.*), (.*)\)")]
async fn given_a_direction(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.direction = Vector::from_vector(x, y, z);
}

#[given(regex = r"r ← ray\(point\((.*), (.*), (.*)\), vector\((.*), (.*), (.*)\)\)")]
async fn given_a_ray(tr: &mut TestRunner, px: f32, py: f32, pz: f32, vx: f32, vy: f32, vz: f32) {
    tr.r1 = Ray::new(
        Point::from_point(px, py, pz),
        Vector::from_vector(vx, vy, vz),
    );
}

#[given("s ← sphere()")]
async fn given_a_sphere(tr: &mut TestRunner) {
    tr.sphere = Some(Object::new(ShapeHandle::new(Sphere), MatrixN::identity(4)));
}

#[given(regex = r"(i[0-9]?) ← intersection\((.*), s\)")]
async fn given_ray_intersection(tr: &mut TestRunner, _id: String, toi: f32) {
    tr.is.push(toi);
}

#[given(regex = r"xs ← intersections\((.*)\)")]
async fn given_a_bundle_of_intersections(tr: &mut TestRunner, ids: String) {
    let ids = ids
        .split(", ")
        .map(|id| id.trim_start_matches('i').parse::<usize>().unwrap() - 1)
        .collect::<Vec<_>>();

    for id in ids {
        tr.xs.push(tr.is[id]);
    }
}

#[given(regex = r"[mt] ← translation\((.*), (.*), (.*)\)")]
async fn given_a_translation(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.m = MatrixN::from_translation(x, y, z);
}

#[given(regex = r"m ← scaling\((.*), (.*), (.*)\)")]
async fn given_a_scaling(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.m = MatrixN::from_scale(x, y, z);
}

#[when("r ← ray(origin, direction)")]
async fn build_ray(tr: &mut TestRunner) {
    tr.r1 = Ray::new(tr.origin, tr.direction);
}

#[when("xs ← intersect(s, r)")]
async fn sphere_intersects_ray(tr: &mut TestRunner) {
    let co = tr.sphere.as_ref().unwrap();
    tr.xs = co.shape().toi_with_ray(co.transform(), &tr.r1);
}

#[when(regex = r"i ← intersection\((.*), s\)")]
async fn ray_intersection(tr: &mut TestRunner, toi: f32) {
    tr.is.push(toi);
}

#[when(regex = r"xs ← intersections\((.*)\)")]
async fn bundle_intersections(tr: &mut TestRunner, ids: String) {
    given_a_bundle_of_intersections(tr, ids).await
}

#[when("i ← hit(xs)")]
async fn ray_hit(tr: &mut TestRunner) {
    tr.hit = RayIntersections::from(
        tr.xs
            .iter()
            .map(|&toi| RayIntersection::new(toi, Vector::default()))
            .collect::<Vec<_>>()
            .into_iter(),
    )
    .hit()
    .map(|x| x.toi);
}

#[when("r2 ← transform(r, m)")]
async fn transform_ray(tr: &mut TestRunner) {
    tr.r2 = tr.r1.transform_by(&tr.m);
}

#[when("set_transform(s, t)")]
async fn set_transform(tr: &mut TestRunner) {
    let co = tr.sphere.as_mut().unwrap();
    co.set_transform(tr.m.clone());
}

#[when(regex = r"set_transform\(s, scaling\((.*), (.*), (.*)\)\)")]
async fn set_scaling(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    let co = tr.sphere.as_mut().unwrap();
    co.set_transform(MatrixN::from_scale(x, y, z));
}

#[when(regex = r"set_transform\(s, translation\((.*), (.*), (.*)\)\)")]
async fn set_translation(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    let co = tr.sphere.as_mut().unwrap();
    co.set_transform(MatrixN::from_translation(x, y, z));
}

#[then("r.origin = origin")]
async fn check_ray_origin(tr: &mut TestRunner) {
    assert!(tr.r1.origin.abs_diff_eq(&tr.origin, EPSILON));
}

#[then(regex = r"r2\.origin = point\((.*), (.*), (.*)\)")]
async fn check_r2_origin(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!(tr
        .r2
        .origin
        .abs_diff_eq(&Point::from_point(x, y, z), EPSILON));
}

#[then("r.direction = direction")]
async fn check_ray_direction(tr: &mut TestRunner) {
    assert!(tr.r1.dir.abs_diff_eq(&tr.direction, EPSILON));
}

#[then(regex = r"r2\.direction = vector\((.*), (.*), (.*)\)")]
async fn check_r2_direction(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!(tr
        .r2
        .dir
        .abs_diff_eq(&Vector::from_vector(x, y, z), EPSILON));
}

#[then(regex = r"position\(r, (.*)\) = point\((.*), (.*), (.*)\)")]
async fn check_ray_position(tr: &mut TestRunner, t: f32, x: f32, y: f32, z: f32) {
    assert!(tr
        .r1
        .point_at(t)
        .abs_diff_eq(&Point::from_point(x, y, z), EPSILON));
}

#[then(regex = r"xs\.count = (.*)")]
async fn xs_count(tr: &mut TestRunner, n: usize) {
    assert_eq!(tr.xs.len(), n);
}

#[then(regex = r"xs\[(.*)\](?:\.t)? = (.*)")]
async fn xs_index_toi(tr: &mut TestRunner, i: usize, t: f32) {
    assert!((tr.xs[i] - t).abs() < EPSILON);
}

#[then(regex = r"xs\[(.*)\]\.object = s")]
async fn xs_index_object(_tr: &mut TestRunner, _i: usize) {
    // TODO: right now, `ShapeHandle` cannot be compared for equality
}

#[then(regex = r"i\.t = (.*)")]
async fn intersection_toi(tr: &mut TestRunner, toi: f32) {
    assert!((tr.is[0] - toi).abs() < EPSILON);
}

#[then("i.object = s")]
async fn intersection_object_is(_: &mut TestRunner) {}

#[then(regex = r"i = i(.*)")]
async fn check_hit(tr: &mut TestRunner, id: String) {
    let id = id.parse::<usize>().unwrap() - 1;
    assert!((tr.hit.as_ref().unwrap() - tr.is[id]).abs() < EPSILON);
}

#[then("i is nothing")]
async fn check_not_hit(tr: &mut TestRunner) {
    assert!(tr.hit.is_none());
}

#[then("s.transform = identity_matrix")]
async fn sphere_default_transform(_: &mut TestRunner) {
    let co = Object::new(ShapeHandle::new(Sphere), MatrixN::identity(4));
    assert!(co.transform().abs_diff_eq(&MatrixN::identity(4), EPSILON));
}

#[then("s.transform = t")]
async fn sphere_transform(tr: &mut TestRunner) {
    let co = tr.sphere.as_mut().unwrap();
    assert!(co.transform().abs_diff_eq(&tr.m, EPSILON));
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch05"]);
    runner.run_and_exit().await;
}
