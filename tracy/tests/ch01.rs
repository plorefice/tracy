#![allow(clippy::many_single_char_names)]

use tracy::math::{Coords, Point, Vector};

const EPSILON: f32 = 1e-4;

#[test]
fn a_tuple_with_w_equal_to_one_is_a_point() {
    let a = Coords::from((4.3, -4.2, 3.1, 1.0));
    assert!((a.x - 4.3).abs() < EPSILON);
    assert!((a.y + 4.2).abs() < EPSILON);
    assert!((a.z - 3.1).abs() < EPSILON);
    assert!((a.w - 1.0).abs() < EPSILON);
    assert!(a.is_point());
    assert!(!a.is_vector());
}

#[test]
fn a_tuple_with_w_equal_to_zero_is_a_vector() {
    let a = Coords::from((4.3, -4.2, 3.1, 0.0));
    assert!((a.x - 4.3).abs() < EPSILON);
    assert!((a.y + 4.2).abs() < EPSILON);
    assert!((a.z - 3.1).abs() < EPSILON);
    assert!((a.w - 0.0).abs() < EPSILON);
    assert!(!a.is_point());
    assert!(a.is_vector());
}

#[test]
fn point_creates_tuples_with_w_equal_to_one() {
    let p = Point::from_point(4., -4., 3.);
    assert!(p.abs_diff_eq(&Coords::from((4., -4., 3., 1.)), EPSILON));
}

#[test]
fn vector_creates_tuples_with_w_equal_to_zero() {
    let v = Vector::from_vector(4., -4., 3.);
    assert!(v.abs_diff_eq(&Coords::from((4., -4., 3., 0.)), EPSILON));
}

#[test]
fn adding_two_tuples() {
    let a1 = Coords::from((3., -2., 5., 1.));
    let a2 = Coords::from((-2., 3., 1., 0.));
    assert!((a1 + a2).abs_diff_eq(&Coords::from((1., 1., 6., 1.)), EPSILON));
}

#[test]
fn subtracting_two_points() {
    let p1 = Point::from_point(3., 2., 1.);
    let p2 = Point::from_point(5., 6., 7.);
    assert!((p1 - p2).abs_diff_eq(&Vector::from_vector(-2., -4., -6.), EPSILON));
}

#[test]
fn subtracting_a_vector_from_a_point() {
    let p = Point::from_point(3., 2., 1.);
    let v = Vector::from_vector(5., 6., 7.);
    assert!((p - v).abs_diff_eq(&Point::from_point(-2., -4., -6.), EPSILON));
}

#[test]
fn subtracting_two_vectors() {
    let v1 = Vector::from_vector(3., 2., 1.);
    let v2 = Vector::from_vector(5., 6., 7.);
    assert!((v1 - v2).abs_diff_eq(&Vector::from_vector(-2., -4., -6.), EPSILON));
}

#[test]
fn subtracting_a_vector_from_the_zero_vector() {
    let zero = Vector::from_vector(0., 0., 0.);
    let v = Vector::from_vector(1., -2., 3.);
    assert!((zero - v).abs_diff_eq(&Vector::from_vector(-1., 2., -3.), EPSILON));
}

#[test]
fn negating_a_tuple() {
    let a = Coords::from((1., -2., 3., -4.));
    assert!((-a).abs_diff_eq(&Coords::from((-1., 2., -3., 4.)), EPSILON));
}

#[test]
fn multiplying_a_tuple_by_a_scalar() {
    let a = Coords::from((1., -2., 3., -4.));
    assert!((a * 3.5).abs_diff_eq(&Coords::from((3.5, -7., 10.5, -14.)), EPSILON));
}

#[test]
fn multiplying_a_tuple_by_a_fraction() {
    let a = Coords::from((1., -2., 3., -4.));
    assert!((a * 0.5).abs_diff_eq(&Coords::from((0.5, -1., 1.5, -2.)), EPSILON));
}

#[test]
fn dividing_a_tuple_by_a_scalar() {
    let a = Coords::from((1., -2., 3., -4.));
    assert!((a / 2.).abs_diff_eq(&Coords::from((0.5, -1., 1.5, -2.)), EPSILON));
}

#[test]
fn computing_the_magnitude_of_vectors() {
    for (v, mag) in vec![
        (Vector::from_vector(1., 0., 0.), 1.),
        (Vector::from_vector(0., 1., 0.), 1.),
        (Vector::from_vector(0., 0., 1.), 1.),
        (Vector::from_vector(1., 2., 3.), f32::sqrt(14.)),
        (Vector::from_vector(-1., -2., -3.), f32::sqrt(14.)),
    ]
    .into_iter()
    {
        assert!((v.length() - mag) < EPSILON);
    }
}

#[test]
fn normalizing_vector() {
    for (v, norm) in vec![
        (
            Vector::from_vector(4., 0., 0.),
            Vector::from_vector(1., 0., 0.),
        ),
        (
            Vector::from_vector(1., 2., 3.),
            Vector::from_vector(0.26726, 0.53452, 0.80178),
        ),
    ]
    .into_iter()
    {
        assert!(v.normalize().abs_diff_eq(&norm, EPSILON));
    }
}

#[test]
fn magnitude_of_a_normalized_vector() {
    assert!((Vector::from_vector(1., 2., 3.).normalize().length() - 1.).abs() < EPSILON);
}

#[test]
fn dot_product_of_two_tuples() {
    let a = Vector::from_vector(1., 2., 3.);
    let b = Vector::from_vector(2., 3., 4.);
    assert!((a.dot(&b) - 20.).abs() < EPSILON);
}

#[test]
fn cross_product_of_two_vectors() {
    let a = Vector::from_vector(1., 2., 3.);
    let b = Vector::from_vector(2., 3., 4.);
    
    assert!(a
        .cross(&b)
        .abs_diff_eq(&Vector::from_vector(-1., 2., -1.), EPSILON));

    assert!(b
        .cross(&a)
        .abs_diff_eq(&Vector::from_vector(1., -2., 1.), EPSILON));
}
