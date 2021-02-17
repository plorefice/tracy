use tracy::{math::MatrixN, rendering::Material};
pub use utils::*;

mod utils;

#[test]
fn the_default_transformation() {
    let s = test_shape();
    assert_abs_diff!(s.transform(), MatrixN::identity(4));
}

#[test]
fn assigning_a_transformation() {
    let mut s = test_shape();
    s.set_transform(MatrixN::from_translation(2.0, 3.0, 4.0));
    assert_abs_diff!(s.transform(), MatrixN::from_translation(2.0, 3.0, 4.0));
}

#[test]
fn the_default_material() {
    assert_eq!(test_shape().material(), &Material::default());
}

#[test]
fn assigning_a_material() {
    let mut s = test_shape();
    let m = Material {
        ambient: 1.,
        ..Default::default()
    };

    s.set_material(m);

    assert_eq!(s.material(), &m);
}
