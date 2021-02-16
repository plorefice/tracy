use std::f32::consts::{FRAC_1_SQRT_2, PI};

use tracy::math::{MatrixN, Point, Vector};
pub use utils::*;

mod utils;

#[test]
fn multiplying_by_a_translation_matrix() {
    let t = MatrixN::from_translation(5., -3., 2.);
    let p = Point::from_point(-3., 4., 5.);

    assert_abs_diff!(t * p, Point::from_point(2., 1., 7.));
}

#[test]
fn multiplying_by_the_inverse_of_a_translation_matrix() {
    let t = MatrixN::from_translation(5., -3., 2.);
    let inv = t.inverse().unwrap();
    let p = Point::from_point(-3., 4., 5.);

    assert_abs_diff!(inv * p, Point::from_point(-8., 7., 3.));
}

#[test]
fn translation_does_not_affect_vectors() {
    let t = MatrixN::from_translation(5., -3., 2.);
    let v = Vector::from_vector(-3., 4., 5.);

    assert_abs_diff!(t * v, v);
}

#[test]
fn a_scaling_matrix_applied_to_a_point() {
    let t = MatrixN::from_scale(2., 3., 4.);
    let p = Point::from_point(-4., 6., 8.);

    assert_abs_diff!(t * p, Point::from_point(-8., 18., 32.));
}

#[test]
fn a_scaling_matrix_applied_to_a_vector() {
    let t = MatrixN::from_scale(2., 3., 4.);
    let v = Vector::from_vector(-4., 6., 8.);

    assert_abs_diff!(t * v, Vector::from_vector(-8., 18., 32.));
}

#[test]
fn multiplying_by_the_inverse_of_a_scaling_matrix() {
    let t = MatrixN::from_scale(2., 3., 4.);
    let inv = t.inverse().unwrap();
    let v = Vector::from_vector(-4., 6., 8.);

    assert_abs_diff!(inv * v, Vector::from_vector(-2., 2., 2.));
}

#[test]
fn reflection_is_scaling_by_a_negative_value() {
    let t = MatrixN::from_scale(-1., 1., 1.);
    let p = Point::from_point(2., 3., 4.);

    assert_abs_diff!(t * p, Point::from_point(-2., 3., 4.));
}

#[test]
fn rotating_a_point_around_the_x_axis() {
    let p = Point::from_point(0., 1., 0.);
    let hq = MatrixN::from_rotation_x(PI / 4.);
    let fq = MatrixN::from_rotation_x(PI / 2.);

    assert_abs_diff!(hq * p, Point::from_point(0., FRAC_1_SQRT_2, FRAC_1_SQRT_2));
    assert_abs_diff!(fq * p, Point::from_point(0., 0., 1.));
}

#[test]
fn the_inverse_of_an_x_rotation_rotates_in_the_opposite_direction() {
    let p = Point::from_point(0., 1., 0.);
    let hq = MatrixN::from_rotation_x(PI / 4.);
    let inv = hq.inverse().unwrap();

    assert_abs_diff!(
        inv * p,
        Point::from_point(0., FRAC_1_SQRT_2, -FRAC_1_SQRT_2)
    );
}

#[test]
fn rotating_a_point_around_the_y_axis() {
    let p = Point::from_point(0., 0., 1.);
    let hq = MatrixN::from_rotation_y(PI / 4.);
    let fq = MatrixN::from_rotation_y(PI / 2.);

    assert_abs_diff!(hq * p, Point::from_point(FRAC_1_SQRT_2, 0., FRAC_1_SQRT_2));
    assert_abs_diff!(fq * p, Point::from_point(1., 0., 0.));
}

#[test]
fn rotating_a_point_around_the_z_axis() {
    let p = Point::from_point(0., 1., 0.);
    let hq = MatrixN::from_rotation_z(PI / 4.);
    let fq = MatrixN::from_rotation_z(PI / 2.);

    assert_abs_diff!(hq * p, Point::from_point(-FRAC_1_SQRT_2, FRAC_1_SQRT_2, 0.));
    assert_abs_diff!(fq * p, Point::from_point(-1., 0., 0.));
}

#[test]
fn shearing_transformations() {
    let p = Point::from_point(2., 3., 4.);

    for (t, res) in vec![
        (
            MatrixN::from_shear(1., 0., 0., 0., 0., 0.),
            Point::from_point(5., 3., 4.),
        ),
        (
            MatrixN::from_shear(0., 1., 0., 0., 0., 0.),
            Point::from_point(6., 3., 4.),
        ),
        (
            MatrixN::from_shear(0., 0., 1., 0., 0., 0.),
            Point::from_point(2., 5., 4.),
        ),
        (
            MatrixN::from_shear(0., 0., 0., 1., 0., 0.),
            Point::from_point(2., 7., 4.),
        ),
        (
            MatrixN::from_shear(0., 0., 0., 0., 1., 0.),
            Point::from_point(2., 3., 6.),
        ),
        (
            MatrixN::from_shear(0., 0., 0., 0., 0., 1.),
            Point::from_point(2., 3., 7.),
        ),
    ]
    .into_iter()
    {
        assert_abs_diff!(t * p, res);
    }
}

#[test]
fn individual_transformations_are_applied_in_sequence() {
    let p = Point::from_point(1., 0., 1.);
    let a = MatrixN::from_rotation_x(PI / 2.);
    let b = MatrixN::from_scale(5., 5., 5.);
    let c = MatrixN::from_translation(10., 5., 7.);

    let p2 = a * p;
    assert_abs_diff!(p2, Point::from_point(1., -1., 0.));

    let p3 = b * p2;
    assert_abs_diff!(p3, Point::from_point(5., -5., 0.));

    let p4 = c * p3;
    assert_abs_diff!(p4, Point::from_point(15., 0., 7.));
}

#[test]
fn chained_transformations_must_be_applied_in_reverse_order() {
    let p = Point::from_point(1., 0., 1.);
    let a = MatrixN::from_rotation_x(PI / 2.);
    let b = MatrixN::from_scale(5., 5., 5.);
    let c = MatrixN::from_translation(10., 5., 7.);
    let transform = c * b * a;

    assert_abs_diff!(transform * p, Point::from_point(15., 0., 7.));
}
