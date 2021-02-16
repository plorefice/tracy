use tracy::math::{Coords, MatrixN};
pub use utils::*;

mod utils;

#[test]
fn constructing_and_inspecting_a_4x4_matrix() {
    let m = MatrixN::from_row_slice(
        4,
        [
            1., 2., 3., 4., 5.5, 6.5, 7.5, 8.5, 9., 10., 11., 12., 13.5, 14.5, 15.5, 16.5,
        ],
    );

    for (pos, val) in vec![
        ((0, 0), 1.),
        ((0, 3), 4.),
        ((1, 0), 5.5),
        ((1, 2), 7.5),
        ((2, 2), 11.),
        ((3, 0), 13.5),
        ((3, 2), 15.5),
    ]
    .into_iter()
    {
        assert_f32!(m[pos], val);
    }
}

#[test]
fn a_2x2_matrix_ought_to_be_representable() {
    let m = MatrixN::from_row_slice(2, [-3., 5., 1., -2.]);

    for (pos, val) in vec![((0, 0), -3.), ((0, 1), 5.), ((1, 0), 1.), ((1, 1), -2.)].into_iter() {
        assert_f32!(m[pos], val);
    }
}

#[test]
fn a_3x3_matrix_ought_to_be_representable() {
    let m = MatrixN::from_row_slice(3, [-3., 5., 0., 1., -2., -7., 0., 1., 1.]);

    for (pos, val) in vec![((0, 0), -3.), ((1, 1), -2.), ((2, 2), 1.)].into_iter() {
        assert_f32!(m[pos], val);
    }
}

#[test]
fn matrix_equality_with_identical_matrices() {
    let a = MatrixN::from_row_slice(
        4,
        [
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ],
    );

    let b = MatrixN::from_row_slice(
        4,
        [
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ],
    );

    assert_abs_diff!(a, &b);
}

#[test]
fn matrix_equality_with_different_matrices() {
    let a = MatrixN::from_row_slice(
        4,
        [
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ],
    );

    let b = MatrixN::from_row_slice(
        4,
        [
            2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2., 1.,
        ],
    );

    assert_not_abs_diff!(a, b);
}

#[test]
fn multiplying_two_matrices() {
    let a = MatrixN::from_row_slice(
        4,
        [
            1., 2., 3., 4., 5., 6., 7., 8., 9., 8., 7., 6., 5., 4., 3., 2.,
        ],
    );

    let b = MatrixN::from_row_slice(
        4,
        [
            -2., 1., 2., 3., 3., 2., 1., -1., 4., 3., 6., 5., 1., 2., 7., 8.,
        ],
    );

    let prod = MatrixN::from_row_slice(
        4,
        [
            20., 22., 50., 48., 44., 54., 114., 108., 40., 58., 110., 102., 16., 26., 46., 42.,
        ],
    );

    assert_abs_diff!(a * b, prod);
}

#[test]
fn a_matrix_multiplied_by_a_tuple() {
    let a = MatrixN::from_row_slice(
        4,
        [
            1., 2., 3., 4., 2., 4., 4., 2., 8., 6., 4., 1., 0., 0., 0., 1.,
        ],
    );

    let b = Coords::from((1., 2., 3., 1.));

    assert_abs_diff!(a * b, Coords::from((18., 24., 33., 1.)));
}

#[test]
fn multiplying_a_matrix_by_the_identity_matrix() {
    let a = MatrixN::from_row_slice(
        4,
        [
            0., 1., 2., 4., 1., 2., 4., 8., 2., 4., 8., 16., 4., 8., 16., 32.,
        ],
    );

    assert_abs_diff!(&a * MatrixN::identity(4), a);
}

#[test]
fn multiplying_the_identity_matrix_by_a_tuple() {
    let a = Coords::from((1., 2., 3., 4.));
    assert_abs_diff!(MatrixN::identity(4) * a, a);
}

#[test]
fn transposing_a_matrix() {
    let a = MatrixN::from_row_slice(
        4,
        [
            0., 9., 3., 0., 9., 8., 0., 8., 1., 8., 5., 3., 0., 0., 5., 8.,
        ],
    );

    let transpose = MatrixN::from_row_slice(
        4,
        [
            0., 9., 1., 0., 9., 8., 8., 0., 3., 0., 5., 5., 0., 8., 3., 8.,
        ],
    );

    assert_abs_diff!(a.transpose(), transpose);
}

#[test]
fn transposing_the_identity_matrix() {
    assert_abs_diff!(MatrixN::identity(4).transpose(), MatrixN::identity(4));
}

#[test]
fn calculating_the_determinant_of_a_2x2_matrix() {
    let a = MatrixN::from_row_slice(2, [1., 5., -3., 2.]);
    assert_f32!(a.det(), 17.);
}

#[test]
fn a_submatrix_of_a_3x3_matrix_is_a_2x2_matrix() {
    let a = MatrixN::from_row_slice(3, [1., 5., 0., -3., 2., 7., 0., 6., -3.]);

    assert_abs_diff!(
        a.submatrix(0, 2),
        MatrixN::from_row_slice(2, [-3., 2., 0., 6.])
    );
}

#[test]
fn a_submatrix_of_a_4x4_matrix_is_a_3x3_matrix() {
    let a = MatrixN::from_row_slice(
        4,
        [
            -6., 1., 1., 6., -8., 5., 8., 6., -1., 0., 8., 2., -7., 1., -1., 1.,
        ],
    );

    let exp = MatrixN::from_row_slice(3, [-6., 1., 6., -8., 8., 6., -7., -1., 1.]);

    assert_abs_diff!(a.submatrix(2, 1), exp);
}

#[test]
fn calculating_a_minor_of_a_3x3_matrix() {
    let a = MatrixN::from_row_slice(3, [3., 5., 0., 2., -1., -7., 6., -1., 5.]);
    let b = a.submatrix(1, 0);

    assert_f32!(b.det(), 25.);
    assert_f32!(a.minor(1, 0), 25.);
}

#[test]
fn calculating_a_cofactor_of_a_3x3_matrix() {
    let a = MatrixN::from_row_slice(3, [3., 5., 0., 2., -1., -7., 6., -1., 5.]);

    assert_f32!(a.minor(0, 0), -12.);
    assert_f32!(a.cofactor(0, 0), -12.);
    assert_f32!(a.minor(1, 0), 25.);
    assert_f32!(a.cofactor(1, 0), -25.);
}

#[test]
fn calculating_the_determinant_of_a_3x3_matrix() {
    let a = MatrixN::from_row_slice(3, [1., 2., 6., -5., 8., -4., 2., 6., 4.]);

    assert_f32!(a.cofactor(0, 0), 56.);
    assert_f32!(a.cofactor(0, 1), 12.);
    assert_f32!(a.cofactor(0, 2), -46.);
    assert_f32!(a.det(), -196.);
}

#[test]
fn calculating_the_determinant_of_a_4x4_matrix() {
    let a = MatrixN::from_row_slice(
        4,
        [
            -2., -8., 3., 5., -3., 1., 7., 3., 1., 2., -9., 6., -6., 7., 7., -9.,
        ],
    );

    assert_f32!(a.cofactor(0, 0), 690.);
    assert_f32!(a.cofactor(0, 1), 447.);
    assert_f32!(a.cofactor(0, 2), 210.);
    assert_f32!(a.cofactor(0, 3), 51.);
    assert_f32!(a.det(), -4071.);
}

#[test]
fn testing_an_invertible_matrix_for_invertibility() {
    let a = MatrixN::from_row_slice(
        4,
        [
            6., 4., 4., 4., 5., 5., 7., 6., 4., -9., 3., -7., 9., 1., 7., -6.,
        ],
    );

    assert_f32!(a.det(), -2120.);
    assert!(a.inverse().is_some());
}

#[test]
fn testing_a_noninvertible_matrix_for_invertibility() {
    let a = MatrixN::from_row_slice(
        4,
        [
            -4., 2., -2., -3., 9., 6., 2., 6., 0., -5., 1., -5., 0., 0., 0., 0.,
        ],
    );

    assert_f32!(a.det(), 0.);
    assert!(a.inverse().is_none());
}

#[test]
fn calculating_the_inverse_of_a_matrix() {
    let a = MatrixN::from_row_slice(
        4,
        [
            -5., 2., 6., -8., 1., -5., 1., 8., 7., 7., -6., -7., 1., -3., 7., 4.,
        ],
    );

    let b = a.inverse().unwrap();

    assert_f32!(a.det(), 532.);
    assert_f32!(a.cofactor(2, 3), -160.);
    assert_f32!(b[(3, 2)], -160. / 532.);
    assert_f32!(a.cofactor(3, 2), 105.);
    assert_f32!(b[(2, 3)], 105. / 532.);

    let exp = MatrixN::from_row_slice(
        4,
        [
            0.21805, 0.45113, 0.24060, -0.04511, -0.80827, -1.45677, -0.44361, 0.52068, -0.07895,
            -0.22368, -0.05263, 0.19737, -0.52256, -0.81391, -0.30075, 0.30639,
        ],
    );

    assert_abs_diff!(b, exp);
}

#[test]
fn calculating_the_inverse_of_another_matrix() {
    let a = MatrixN::from_row_slice(
        4,
        [
            8., -5., 9., 2., 7., 5., 6., 1., -6., 0., 9., 6., -3., 0., -9., -4.,
        ],
    );

    let inv = MatrixN::from_row_slice(
        4,
        [
            -0.15385, -0.15385, -0.28205, -0.53846, -0.07692, 0.12308, 0.02564, 0.03077, 0.35897,
            0.35897, 0.43590, 0.92308, -0.69231, -0.69231, -0.76923, -1.92308,
        ],
    );

    assert_abs_diff!(a.inverse().unwrap(), inv);
}

#[test]
fn calculating_the_inverse_of_a_third_matrix() {
    let a = MatrixN::from_row_slice(
        4,
        [
            9., 3., 0., 9., -5., -2., -6., -3., -4., 9., 6., 4., -7., 6., 6., 2.,
        ],
    );

    let inv = MatrixN::from_row_slice(
        4,
        [
            -0.04074, -0.07778, 0.14444, -0.22222, -0.07778, 0.03333, 0.36667, -0.33333, -0.02901,
            -0.14630, -0.10926, 0.12963, 0.17778, 0.06667, -0.26667, 0.33333,
        ],
    );

    assert_abs_diff!(a.inverse().unwrap(), inv);
}

#[test]
fn multiplying_a_product_by_its_inverse() {
    let a = MatrixN::from_row_slice(
        4,
        [
            3., -9., 7., 3., 3., -8., 2., -9., -4., 4., 4., 1., -6., 5., -1., 1.,
        ],
    );

    let b = MatrixN::from_row_slice(
        4,
        [
            8., 2., 2., 2., 3., -1., 7., 0., 7., 0., 5., 4., 6., -2., 0., 5.,
        ],
    );

    let c = &a * &b;
    assert_abs_diff!(c * b.inverse().unwrap(), a);
}
