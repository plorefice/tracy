use tracy::{
    math::{Point3, Vec3, EPSILON},
    query::{Ray, World},
};
pub use utils::*;

mod utils;

#[test]
fn a_ray_intersects_a_cube() {
    for &(origin, dir, t1, t2) in &[
        (Point3::new(5., 0.5, 0.), Vec3::new(-1., 0., 0.), 4., 6.),
        (Point3::new(-5., 0.5, 0.), Vec3::new(1., 0., 0.), 4., 6.),
        (Point3::new(0.5, 5., 0.), Vec3::new(0., -1., 0.), 4., 6.),
        (Point3::new(0.5, -5., 0.), Vec3::new(0., 1., 0.), 4., 6.),
        (Point3::new(0.5, 0., 5.), Vec3::new(0., 0., -1.), 4., 6.),
        (Point3::new(0.5, 0., -5.), Vec3::new(0., 0., 1.), 4., 6.),
        (Point3::new(0., 0.5, 0.), Vec3::new(0., 0., 1.), -1., 1.),
    ] {
        let mut w = World::new();
        w.add(cube());

        let r = Ray::new(origin, dir);

        let mut xs = w.interferences_with_ray(&r);
        assert_f32!(xs.next().unwrap().toi, t1);
        assert_f32!(xs.next().unwrap().toi, t2);
        assert_eq!(xs.count(), 0);
    }
}

#[test]
fn a_ray_misses_a_cube() {
    for &(origin, dir) in &[
        (Point3::new(-2., 0., 0.), Vec3::new(0.2673, 0.5345, 0.8018)),
        (Point3::new(0., -2., 0.), Vec3::new(0.8018, 0.2673, 0.5345)),
        (Point3::new(0., 0., -2.), Vec3::new(0.5345, 0.8018, 0.2673)),
        (Point3::new(2., 0., 2.), Vec3::new(0., 0., -1.)),
        (Point3::new(0., 2., 2.), Vec3::new(0., -1., 0.)),
        (Point3::new(2., 2., 0.), Vec3::new(-1., 0., 0.)),
    ] {
        let mut w = World::new();
        w.add(cube());

        let r = Ray::new(origin, dir);

        assert_eq!(w.interferences_with_ray(&r).count(), 0);
    }
}

#[test]
fn the_normal_on_the_surface_of_a_cube() {
    for &(point, normal) in &[
        (Vec3::new(1., 0.5, -0.8), Vec3::unit_x()),
        (Vec3::new(-1., -0.2, 0.9), -Vec3::unit_x()),
        (Vec3::new(-0.4, 1., -0.1), Vec3::unit_y()),
        (Vec3::new(0.3, -1., -0.7), -Vec3::unit_y()),
        (Vec3::new(-0.6, 0.3, 1.), Vec3::unit_z()),
        (Vec3::new(0.4, 0.4, -1.), -Vec3::unit_z()),
        (Vec3::new(1., 1., 1.), Vec3::unit_x()),
        (Vec3::new(-1., -1., -1.), -Vec3::unit_x()),
    ] {
        let r = Ray::new(Point3::default(), point);

        assert!(cube()
            .shape()
            .intersections_in_local_space(&r)
            .any(|x| x.normal.abs_diff_eq(&normal, EPSILON)));
    }
}
