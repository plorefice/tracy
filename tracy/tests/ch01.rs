use tracy::math::{Coords, Point, Vector};
pub use utils::*;

mod utils;

#[test]
fn a_tuple_with_w_equal_to_one_is_a_point() {
    let a = Coords::from((4.3, -4.2, 3.1, 1.0));
    assert_f32!(a.x, 4.3);
    assert_f32!(a.y, -4.2);
    assert_f32!(a.z, 3.1);
    assert_f32!(a.w, 1.0);
    assert!(a.is_point());
    assert!(!a.is_vector());
}

#[test]
fn a_tuple_with_w_equal_to_zero_is_a_vector() {
    let a = Coords::from((4.3, -4.2, 3.1, 0.0));
    assert_f32!(a.x, 4.3);
    assert_f32!(a.y, -4.2);
    assert_f32!(a.z, 3.1);
    assert_f32!(a.w, 0.0);
    assert!(!a.is_point());
    assert!(a.is_vector());
}

#[test]
fn point_creates_tuples_with_w_equal_to_one() {
    let p = Point::from_point(4., -4., 3.);
    assert_abs_diff!(p, Coords::from((4., -4., 3., 1.)));
}

#[test]
fn vector_creates_tuples_with_w_equal_to_zero() {
    let v = Vector::from_vector(4., -4., 3.);
    assert_abs_diff!(v, Coords::from((4., -4., 3., 0.)));
}

#[test]
fn adding_two_tuples() {
    let a1 = Coords::from((3., -2., 5., 1.));
    let a2 = Coords::from((-2., 3., 1., 0.));
    assert_abs_diff!(a1 + a2, Coords::from((1., 1., 6., 1.)));
}

#[test]
fn subtracting_two_points() {
    let p1 = Point::from_point(3., 2., 1.);
    let p2 = Point::from_point(5., 6., 7.);
    assert_abs_diff!(p1 - p2, Vector::from_vector(-2., -4., -6.));
}

#[test]
fn subtracting_a_vector_from_a_point() {
    let p = Point::from_point(3., 2., 1.);
    let v = Vector::from_vector(5., 6., 7.);
    assert_abs_diff!(p - v, Point::from_point(-2., -4., -6.));
}

#[test]
fn subtracting_two_vectors() {
    let v1 = Vector::from_vector(3., 2., 1.);
    let v2 = Vector::from_vector(5., 6., 7.);
    assert_abs_diff!(v1 - v2, Vector::from_vector(-2., -4., -6.));
}

#[test]
fn subtracting_a_vector_from_the_zero_vector() {
    let zero = Vector::from_vector(0., 0., 0.);
    let v = Vector::from_vector(1., -2., 3.);
    assert_abs_diff!(zero - v, Vector::from_vector(-1., 2., -3.));
}

#[test]
fn negating_a_tuple() {
    let a = Coords::from((1., -2., 3., -4.));
    assert_abs_diff!(-a, Coords::from((-1., 2., -3., 4.)));
}

#[test]
fn multiplying_a_tuple_by_a_scalar() {
    let a = Coords::from((1., -2., 3., -4.));
    assert_abs_diff!(a * 3.5, Coords::from((3.5, -7., 10.5, -14.)));
}

#[test]
fn multiplying_a_tuple_by_a_fraction() {
    let a = Coords::from((1., -2., 3., -4.));
    assert_abs_diff!(a * 0.5, Coords::from((0.5, -1., 1.5, -2.)));
}

#[test]
fn dividing_a_tuple_by_a_scalar() {
    let a = Coords::from((1., -2., 3., -4.));
    assert_abs_diff!(a / 2., Coords::from((0.5, -1., 1.5, -2.)));
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
        assert_f32!(v.length(), mag);
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
        assert_abs_diff!(v.normalize(), norm);
    }
}

#[test]
fn magnitude_of_a_normalized_vector() {
    assert_f32!(Vector::from_vector(1., 2., 3.).normalize().length(), 1.);
}

#[test]
fn dot_product_of_two_tuples() {
    let a = Vector::from_vector(1., 2., 3.);
    let b = Vector::from_vector(2., 3., 4.);
    assert_f32!(a.dot(&b), 20.);
}

#[test]
fn cross_product_of_two_vectors() {
    let a = Vector::from_vector(1., 2., 3.);
    let b = Vector::from_vector(2., 3., 4.);

    assert_abs_diff!(a.cross(&b), Vector::from_vector(-1., 2., -1.));
    assert_abs_diff!(b.cross(&a), Vector::from_vector(1., -2., 1.));
}
