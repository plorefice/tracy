use tracy::{
    math::{Point3, Vec3, EPSILON},
    query::{Ray, RayCast},
    shape::Cylinder,
};
pub use utils::*;

mod utils;

#[test]
fn a_ray_misses_a_cylinder() {
    for &(origin, dir) in &[
        (Point3::new(1.0, 0.0, 0.0), Vec3::unit_y()),
        (Point3::new(0.0, 0.0, 0.0), Vec3::unit_y()),
        (Point3::new(0.0, 0.0, -5.0), Vec3::new(1.0, 1.0, 1.0)),
    ] {
        let cyl = Cylinder::default();
        let r = Ray::new(origin, dir.normalize());

        assert_eq!(cyl.intersections_in_local_space(&r).count(), 0);
    }
}

#[test]
fn a_ray_strikes_a_cylinder() {
    for &(origin, dir, t1, t2) in &[
        (Point3::new(1.0, 0.0, -5.0), Vec3::unit_z(), 5.0, 5.0),
        (Point3::new(0.0, 0.0, -5.0), Vec3::unit_z(), 4.0, 6.0),
        (
            Point3::new(0.5, 0.0, -5.0),
            Vec3::new(0.1, 1.0, 1.0),
            6.80798,
            7.08872,
        ),
    ] {
        let cyl = Cylinder::default();
        let r = Ray::new(origin, dir.normalize());

        let mut xs = cyl.intersections_in_local_space(&r);
        assert_f32!(xs.next().unwrap().toi, t1);
        assert_f32!(xs.next().unwrap().toi, t2);
    }
}

#[test]
fn normal_vector_on_a_cylinder() {
    for &(point, normal) in &[
        (Point3::new(1.0, 0.0, 0.0), Vec3::unit_x()),
        (Point3::new(0.0, 5.0, -1.0), -Vec3::unit_z()),
        (Point3::new(0.0, -2.0, 1.0), Vec3::unit_z()),
        (Point3::new(-1.0, 1.0, 0.0), -Vec3::unit_x()),
    ] {
        let cyl = Cylinder::default();
        let r = Ray::new(Point3::default(), point.into());

        assert!(cyl
            .intersections_in_local_space(&r)
            .any(|x| x.normal.abs_diff_eq(&normal, EPSILON)));
    }
}

#[test]
#[allow(clippy::float_cmp)]
fn the_default_minimum_and_maximum_for_a_cylinder() {
    let cyl = Cylinder::default();
    assert_eq!(cyl.bottom(), f32::NEG_INFINITY);
    assert_eq!(cyl.top(), f32::INFINITY);
}
