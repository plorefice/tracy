use tracy::{
    canvas::Color,
    math::{MatrixN, Point, Vector},
    query::{Ray, World},
    rendering::{Material, PointLight},
};
pub use utils::*;

mod utils;

#[test]
fn creating_a_world() {
    let w = World::new();

    assert_eq!(w.objects().count(), 0);
    assert_eq!(w.lights().count(), 0);
}

#[test]
fn the_default_world() {
    let light = PointLight {
        position: Point::from_point(-10., 10., -10.),
        color: Color::new(1., 1., 1.),
        intensity: 1.,
    };

    let mut s1 = sphere();
    s1.set_material(Material {
        color: Color::new(0.8, 1.0, 0.6),
        diffuse: 0.7,
        specular: 0.2,
        ..Default::default()
    });

    let mut s2 = sphere();
    s2.set_transform(MatrixN::from_scale(0.5, 0.5, 0.5));

    let w = World::default();
    let mut objs = w.objects();

    assert_eq!(w.lights().next().unwrap(), &light);

    let obj = objs.next().unwrap();
    assert_eq!(obj.material(), s1.material());
    assert_eq!(obj.transform(), s1.transform());

    let obj = objs.next().unwrap();
    assert_eq!(obj.material(), s2.material());
    assert_eq!(obj.transform(), s2.transform());
}

#[test]
fn intersect_a_world_with_a_ray() {
    let w = World::default();

    let r = Ray::new(
        Point::from_point(0.0, 0.0, -5.0),
        Vector::from_vector(0.0, 0.0, 1.0),
    );

    let xs = w.interferences_with_ray(&r).collect::<Vec<_>>();

    assert_eq!(xs.len(), 4);
    assert_f32!(xs[0].1.toi, 4.0);
    assert_f32!(xs[1].1.toi, 4.5);
    assert_f32!(xs[2].1.toi, 5.5);
    assert_f32!(xs[3].1.toi, 6.0);
}
