use std::f32::consts::{FRAC_1_SQRT_2, SQRT_2};

use tracy::{
    math::{Matrix, Point3, Vec3, EPSILON},
    query::{Object, Ray, World},
    rendering::{Color, Material, Pattern, PatternKind, PointLight, DEFAULT_RECURSION_DEPTH},
    shape::{Plane, Sphere},
};
pub use utils::*;

mod utils;

#[test]
fn reflectivity_for_the_default_material() {
    assert_f32!(Material::default().reflective, 0.0);
}

#[test]
fn precomputing_the_reflection_vector() {
    let mut w = World::new();
    w.add(plane());

    let r = Ray::new(
        Point3::new(0.0, 1.0, -1.0),
        Vec3::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - SQRT_2).abs() < EPSILON)
        .unwrap();

    assert_abs_diff!(
        interference.reflect,
        Vec3::new(0.0, FRAC_1_SQRT_2, FRAC_1_SQRT_2)
    );
}

#[test]
fn the_reflected_color_for_a_nonreflective_material() {
    let mut w = World::default();

    w.objects_mut().nth(1).unwrap().material_mut().ambient = 1.0;

    let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::unit_z());

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 1.0).abs() < EPSILON)
        .unwrap();

    assert_eq!(
        w.reflected_color(&interference, DEFAULT_RECURSION_DEPTH),
        Some(Color::BLACK)
    );
}

#[test]
fn the_reflected_color_for_a_reflective_material() {
    let mut w = World::default();

    w.add(Object::new_with_material(
        Plane,
        Matrix::from_translation(0.0, -1.0, 0.0),
        Material {
            reflective: 0.5,
            ..Default::default()
        },
    ));

    let r = Ray::new(
        Point3::new(0.0, 0.0, -3.0),
        Vec3::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - SQRT_2).abs() < EPSILON)
        .unwrap();

    assert_abs_diff!(
        w.reflected_color(&interference, DEFAULT_RECURSION_DEPTH)
            .unwrap(),
        Color::new(0.19032, 0.2379, 0.14274)
    );
}

#[test]
fn shade_hit_with_a_reflective_material() {
    let mut w = World::default();

    w.add(Object::new_with_material(
        Plane,
        Matrix::from_translation(0.0, -1.0, 0.0),
        Material {
            reflective: 0.5,
            ..Default::default()
        },
    ));

    let r = Ray::new(
        Point3::new(0.0, 0.0, -3.0),
        Vec3::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - SQRT_2).abs() < EPSILON)
        .unwrap();

    assert_abs_diff!(
        w.shade_hit(&interference, DEFAULT_RECURSION_DEPTH).unwrap(),
        Color::new(0.87677, 0.92436, 0.82918)
    );
}

#[test]
fn color_at_with_mutually_reflective_surfaces() {
    let mut w = World::new();

    w.set_light(PointLight::default());

    w.add(Object::new_with_material(
        Plane,
        Matrix::from_translation(0.0, -1.0, 0.0),
        Material {
            reflective: 1.0,
            ..Default::default()
        },
    ));

    w.add(Object::new_with_material(
        Plane,
        Matrix::from_translation(0.0, 1.0, 0.0),
        Material {
            reflective: 1.0,
            ..Default::default()
        },
    ));

    let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::unit_y());

    assert!(w.color_at(&r, DEFAULT_RECURSION_DEPTH).is_none());
}

#[test]
fn the_reflected_color_at_the_maximum_recursive_depth() {
    let mut w = World::default();

    w.add(Object::new_with_material(
        Plane,
        Matrix::from_translation(0.0, -1.0, 0.0),
        Material {
            reflective: 0.5,
            ..Default::default()
        },
    ));

    let r = Ray::new(
        Point3::new(0.0, 0.0, -3.0),
        Vec3::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - SQRT_2).abs() < EPSILON)
        .unwrap();

    assert!(w.reflected_color(&interference, 0).is_none());
}

#[test]
fn transparency_and_refractive_index_for_the_default_material() {
    assert_f32!(Material::default().transparency, 0.0);
    assert_f32!(Material::default().refractive_index, 1.0);
}

#[test]
fn a_helper_for_producing_a_sphere_with_a_glassy_material() {
    let s = glass_sphere();
    assert_eq!(s.transform(), &Matrix::identity(4));
    assert_f32!(s.material().transparency, 1.0);
    assert_f32!(s.material().refractive_index, 1.5);
}

#[test]
fn finding_n1_and_n2_at_various_intersections() {
    let mut world = World::new();

    let mut a = glass_sphere();
    a.set_transform(Matrix::from_scale(2.0, 2.0, 2.0));
    a.material_mut().refractive_index = 1.5;
    world.add(a);

    let mut b = glass_sphere();
    b.set_transform(Matrix::from_translation(0.0, 0.0, -0.25));
    b.material_mut().refractive_index = 2.0;
    world.add(b);

    let mut c = glass_sphere();
    c.set_transform(Matrix::from_translation(0.0, 0.0, 0.25));
    c.material_mut().refractive_index = 2.5;
    world.add(c);

    let ray = Ray::new(Point3::new(0.0, 0.0, -4.0), Vec3::unit_z());

    let mut xs = world.interferences_with_ray(&ray);

    for &(n1, n2) in [
        (1.0, 1.5),
        (1.5, 2.0),
        (2.0, 2.5),
        (2.5, 2.5),
        (2.5, 1.5),
        (1.5, 1.0),
    ]
    .iter()
    {
        let x = xs.next().unwrap();
        assert_f32!(x.n1, n1);
        assert_f32!(x.n2, n2);
    }
}

#[test]
fn the_under_point_is_offset_below_the_surface() {
    let mut w = World::new();

    let mut s = glass_sphere();
    s.set_transform(Matrix::from_translation(0.0, 0.0, 1.0));
    w.add(s);

    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::unit_z());

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 5.0).abs() < EPSILON)
        .unwrap();

    assert!(interference.under_point.z > EPSILON / 2.0);
    assert!(interference.point.z < interference.under_point.z);
}

#[test]
fn the_refracted_color_with_an_opaque_surface() {
    let w = World::default();
    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::unit_z());

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 4.0).abs() < EPSILON)
        .unwrap();

    assert_eq!(
        w.refracted_color(&interference, DEFAULT_RECURSION_DEPTH),
        Some(Color::BLACK)
    );
}

#[test]
fn the_refracted_color_at_the_maximum_recursive_depth() {
    let mut w = World::default();

    let shape = w.objects_mut().next().unwrap();
    shape.material_mut().transparency = 1.0;
    shape.material_mut().refractive_index = 1.5;

    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::unit_z());

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 4.0).abs() < EPSILON)
        .unwrap();

    assert!(w.refracted_color(&interference, 0).is_none());
}

#[test]
fn the_refracted_color_under_total_internal_reflection() {
    let mut w = World::default();

    let shape = w.objects_mut().next().unwrap();
    shape.material_mut().transparency = 1.0;
    shape.material_mut().refractive_index = 1.5;

    let r = Ray::new(Point3::new(0.0, 0.0, FRAC_1_SQRT_2), Vec3::unit_y());

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - FRAC_1_SQRT_2).abs() < EPSILON)
        .unwrap();

    assert_eq!(w.refracted_color(&interference, 5), Some(Color::BLACK));
}

#[test]
fn the_refracted_color_with_a_refracted_ray() {
    let mut w = World::default();

    let a = w.objects_mut().next().unwrap();
    a.material_mut().ambient = 1.0;
    a.material_mut().pattern = Pattern::new(PatternKind::Test);

    let b = w.objects_mut().nth(1).unwrap();
    b.material_mut().transparency = 1.0;
    b.material_mut().refractive_index = 1.5;

    let r = Ray::new(Point3::new(0.0, 0.0, 0.1), Vec3::unit_y());

    let interference = w.interferences_with_ray(&r).nth(2).unwrap();

    assert_abs_diff!(
        w.refracted_color(&interference, 5).unwrap(),
        Color::new(0.0, 0.99888, 0.04725)
    );
}

#[test]
fn shade_hit_with_a_transparent_material() {
    let mut w = World::default();

    w.add(Object::new_with_material(
        Plane,
        Matrix::from_translation(0.0, -1.0, 0.0),
        Material {
            transparency: 0.5,
            refractive_index: 1.5,
            ..Default::default()
        },
    ));

    w.add(Object::new_with_material(
        Sphere,
        Matrix::from_translation(0.0, -3.5, -0.5),
        Material {
            pattern: Pattern::new(Color::new(1.0, 0.0, 0.0).into()),
            ambient: 0.5,
            ..Default::default()
        },
    ));

    let r = Ray::new(
        Point3::new(0.0, 0.0, -3.0),
        Vec3::new(0.0, -FRAC_1_SQRT_2, FRAC_1_SQRT_2),
    );

    let interference = w.interferences_with_ray(&r).next().unwrap();

    assert_abs_diff!(
        w.shade_hit(&interference, DEFAULT_RECURSION_DEPTH).unwrap(),
        Color::new(0.93642, 0.68642, 0.68642)
    );
}
