use tracy::rendering::Material;
pub use utils::*;

mod utils;

#[test]
fn reflectivity_for_the_default_material() {
	assert_f32!(Material::default().reflective, 0.0);
}
