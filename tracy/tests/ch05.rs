use itertools::Itertools;
use tracy::{
    math::{Matrix, Point3, Vec3},
    query::{Ray, RayIntersection, RayIntersections},
};
pub use utils::*;

mod utils;

#[test]
fn creating_and_querying_a_ray() {
    let origin = Point3::from_point(1., 2., 3.);
    let direction = Vec3::from_vector(4., 5., 6.);
    let r = Ray::new(origin, direction);

    assert_abs_diff!(r.origin, origin);
    assert_abs_diff!(r.dir, direction);
}

#[test]
fn computing_a_point_from_a_distance() {
    let r = Ray::new(
        Point3::from_point(2., 3., 4.),
        Vec3::from_vector(1., 0., 0.),
    );

    assert_abs_diff!(r.point_at(0.), Point3::from_point(2., 3., 4.));
    assert_abs_diff!(r.point_at(1.), Point3::from_point(3., 3., 4.));
    assert_abs_diff!(r.point_at(-1.), Point3::from_point(1., 3., 4.));
    assert_abs_diff!(r.point_at(2.5), Point3::from_point(4.5, 3., 4.));
}

#[test]
fn translating_a_ray() {
    let r = Ray::new(
        Point3::from_point(1., 2., 3.),
        Vec3::from_vector(0., 1., 0.),
    );
    let m = Matrix::from_translation(3., 4., 5.);
    let r2 = r.transform_by(&m);

    assert_abs_diff!(r2.origin, Point3::from_point(4., 6., 8.));
    assert_abs_diff!(r2.dir, Vec3::from_vector(0., 1., 0.));
}

#[test]
fn scaling_a_ray() {
    let r = Ray::new(
        Point3::from_point(1., 2., 3.),
        Vec3::from_vector(0., 1., 0.),
    );
    let m = Matrix::from_scale(2., 3., 4.);
    let r2 = r.transform_by(&m);

    assert_abs_diff!(r2.origin, Point3::from_point(2., 6., 12.));
    assert_abs_diff!(r2.dir, Vec3::from_vector(0., 3., 0.));
}

#[test]
fn a_ray_intersects_a_sphere_at_two_points() {
    let r = Ray::new(
        Point3::from_point(0., 0., -5.),
        Vec3::from_vector(0., 0., 1.),
    );

    let xs = tois_with_default_sphere(&r);

    assert_eq!(xs.len(), 2);
    assert_f32!(xs[0], 4.);
    assert_f32!(xs[1], 6.);
}

#[test]
fn a_ray_intersects_a_sphere_at_a_tangent() {
    let r = Ray::new(
        Point3::from_point(0., 1., -5.),
        Vec3::from_vector(0., 0., 1.),
    );

    let xs = tois_with_default_sphere(&r);

    assert_eq!(xs.len(), 2);
    assert_f32!(xs[0], 5.);
    assert_f32!(xs[1], 5.);
}

#[test]
fn a_ray_misses_a_sphere() {
    let r = Ray::new(
        Point3::from_point(0., 2., -5.),
        Vec3::from_vector(0., 0., 1.),
    );

    let xs = tois_with_default_sphere(&r);

    assert_eq!(xs.len(), 0);
}

#[test]
fn a_ray_originates_inside_a_sphere() {
    let r = Ray::new(
        Point3::from_point(0., 0., 0.),
        Vec3::from_vector(0., 0., 1.),
    );

    let xs = tois_with_default_sphere(&r);

    assert_eq!(xs.len(), 2);
    assert_f32!(xs[0], -1.);
    assert_f32!(xs[1], 1.);
}

#[test]
fn a_sphere_is_behind_a_ray() {
    let r = Ray::new(
        Point3::from_point(0., 0., 5.),
        Vec3::from_vector(0., 0., 1.),
    );

    let xs = tois_with_default_sphere(&r);

    assert_eq!(xs.len(), 2);
    assert_f32!(xs[0], -6.);
    assert_f32!(xs[1], -4.);
}

#[test]
fn a_sphere_default_transformation() {
    assert_abs_diff!(sphere().transform(), Matrix::identity(4));
}

#[test]
fn changing_a_sphere_transformation() {
    let mut s = sphere();
    let t = Matrix::from_translation(2., 3., 4.);
    s.set_transform(t.clone());

    assert_abs_diff!(s.transform(), t);
}

#[test]
fn intersecting_a_scaled_sphere_with_a_ray() {
    let r = Ray::new(
        Point3::from_point(0., 0., -5.),
        Vec3::from_vector(0., 0., 1.),
    );

    let mut s = sphere();
    s.set_transform(Matrix::from_scale(2., 2., 2.));

    let xs = s
        .interferences_with_ray(&r)
        .map(|x| x.toi)
        .collect::<Vec<_>>();

    assert_eq!(xs.len(), 2);
    assert_f32!(xs[0], 3.);
    assert_f32!(xs[1], 7.);
}

#[test]
fn intersecting_a_translated_sphere_with_a_ray() {
    let r = Ray::new(
        Point3::from_point(0., 0., -5.),
        Vec3::from_vector(0., 0., 1.),
    );

    let mut s = sphere();
    s.set_transform(Matrix::from_translation(5., 0., 0.));

    assert_eq!(s.interferences_with_ray(&r).count(), 0);
}

#[test]
fn an_intersection_encapsulates_t_and_object() {
    let i = RayIntersection {
        toi: 3.5,
        normal: Vec3::default(),
    };

    assert_f32!(i.toi, 3.5);
}

#[test]
fn aggregating_intersections() {
    let i1 = RayIntersection {
        toi: 1.,
        normal: Vec3::default(),
    };

    let i2 = RayIntersection {
        toi: 2.,
        normal: Vec3::default(),
    };

    let mut xs = RayIntersections::from(vec![i1, i2].into_iter());

    assert_f32!(xs.next().unwrap().toi, 1.);
    assert_f32!(xs.next().unwrap().toi, 2.);
}

#[test]
fn intersect_sets_the_object_on_the_intersection() {
    let r = Ray::new(
        Point3::from_point(0., 0., -5.),
        Vec3::from_vector(0., 0., 1.),
    );

    assert_eq!(sphere().interferences_with_ray(&r).count(), 2);
}

#[test]
fn the_hit_when_all_intersections_have_positive_t() {
    let i1 = RayIntersection {
        toi: 1.,
        normal: Vec3::default(),
    };
    let i2 = RayIntersection {
        toi: 2.,
        normal: Vec3::default(),
    };

    let i = RayIntersections::from(vec![i2, i1.clone()].into_iter()).hit();

    assert_f32!(i.unwrap().toi, i1.toi);
}

#[test]
fn the_hit_when_some_intersections_have_negative_t() {
    let i1 = RayIntersection {
        toi: -1.,
        normal: Vec3::default(),
    };
    let i2 = RayIntersection {
        toi: 1.,
        normal: Vec3::default(),
    };

    let i = RayIntersections::from(vec![i2.clone(), i1].into_iter()).hit();

    assert_f32!(i.unwrap().toi, i2.toi);
}

#[test]
fn the_hit_when_all_intersections_have_negative_t() {
    let i1 = RayIntersection {
        toi: -2.,
        normal: Vec3::default(),
    };
    let i2 = RayIntersection {
        toi: -1.,
        normal: Vec3::default(),
    };

    assert!(RayIntersections::from(vec![i2, i1].into_iter())
        .hit()
        .is_none());
}

#[test]
fn the_hit_is_always_the_lowest_nonnegative_intersection() {
    let xs = RayIntersections::from(
        [5., 7., -3., 2.]
            .iter()
            .map(|&toi| RayIntersection {
                toi,
                normal: Vec3::default(),
            })
            .collect_vec()
            .into_iter(),
    );

    assert_f32!(xs.hit().unwrap().toi, 2.);
}

fn tois_with_default_sphere(ray: &Ray) -> Vec<f32> {
    sphere()
        .interferences_with_ray(ray)
        .map(|x| x.toi)
        .collect()
}
