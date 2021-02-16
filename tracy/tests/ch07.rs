use tracy::{
    canvas::Color,
    math::{MatrixN, Point, Vector},
    query::{Object, Ray, World},
    rendering::{Material, PointLight},
    shape::{ShapeHandle, Sphere},
};
pub use utils::*;

mod utils;

#[test]
fn creating_a_world() {
    let w = World::new();

    assert_eq!(w.objects().count(), 0);
    assert!(w.light().is_none());
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

    assert_eq!(w.light(), Some(&light));

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
    assert_f32!(xs[0].toi, 4.0);
    assert_f32!(xs[1].toi, 4.5);
    assert_f32!(xs[2].toi, 5.5);
    assert_f32!(xs[3].toi, 6.0);
}

#[test]
fn precomputing_the_state_of_an_intersection() {
    let mut w = World::new();
    let s = w.add(Object::new(ShapeHandle::new(Sphere), MatrixN::identity(4)));

    let r = Ray::new(
        Point::from_point(0.0, 0.0, -5.0),
        Vector::from_vector(0.0, 0.0, 1.0),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 4.).abs() < 1e-4)
        .unwrap();

    assert_eq!(interference.handle, s);
    assert_f32!(interference.toi, 4.);
    assert_abs_diff!(interference.point, Point::from_point(0.0, 0.0, -1.0));
    assert_abs_diff!(interference.eye, Vector::from_vector(0.0, 0.0, -1.0));
    assert_abs_diff!(interference.normal, Vector::from_vector(0.0, 0.0, -1.0));
}

#[test]
fn the_hit_when_an_intersection_occurs_on_the_outside() {
    let mut w = World::new();

    w.add(Object::new(ShapeHandle::new(Sphere), MatrixN::identity(4)));

    let r = Ray::new(
        Point::from_point(0.0, 0.0, -5.0),
        Vector::from_vector(0.0, 0.0, 1.0),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 4.).abs() < 1e-4)
        .unwrap();

    assert!(!interference.inside);
}

#[test]
fn the_hit_when_an_intersection_occurs_on_the_inside() {
    let mut w = World::new();

    w.add(Object::new(ShapeHandle::new(Sphere), MatrixN::identity(4)));

    let r = Ray::new(
        Point::from_point(0.0, 0.0, 0.0),
        Vector::from_vector(0.0, 0.0, 1.0),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 1.).abs() < 1e-4)
        .unwrap();

    assert_abs_diff!(interference.point, Point::from_point(0.0, 0.0, 1.0));
    assert_abs_diff!(interference.eye, Vector::from_vector(0.0, 0.0, -1.0));
    assert_abs_diff!(interference.normal, Vector::from_vector(0.0, 0.0, -1.0));
    assert!(interference.inside);
}

#[test]
fn shading_an_intersection() {
    let w = World::default();
    let r = Ray::new(
        Point::from_point(0.0, 0.0, -5.0),
        Vector::from_vector(0.0, 0.0, 1.0),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 4.).abs() < 1e-4)
        .unwrap();

    assert_abs_diff!(
        w.shade_hit(&interference).unwrap(),
        Color::new(0.38066, 0.47583, 0.2855)
    );
}

#[test]
fn shading_an_intersection_from_the_inside() {
    let mut w = World::default();

    w.set_light(PointLight {
        position: Point::from_point(0.0, 0.25, 0.0),
        color: Color::new(1.0, 1.0, 1.0),
        intensity: 1.0,
    });

    let r = Ray::new(
        Point::from_point(0.0, 0.0, 0.0),
        Vector::from_vector(0.0, 0.0, 1.0),
    );

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 0.5).abs() < 1e-4)
        .unwrap();

    assert_abs_diff!(
        w.shade_hit(&interference).unwrap(),
        Color::new(0.90498, 0.90498, 0.90498)
    );
}

#[test]
fn the_color_when_a_ray_misses() {
    let w = World::default();
    let r = Ray::new(
        Point::from_point(0.0, 0.0, -5.0),
        Vector::from_vector(0.0, 1.0, 0.0),
    );

    assert!(w.color_at(&r).is_none());
}

#[test]
fn the_color_when_a_ray_hits() {
    let w = World::default();
    let r = Ray::new(
        Point::from_point(0.0, 0.0, -5.0),
        Vector::from_vector(0.0, 0.0, 1.0),
    );

    assert_abs_diff!(
        w.color_at(&r).unwrap(),
        Color::new(0.38066, 0.47583, 0.2855)
    );
}

#[test]
fn the_color_with_an_intersection_behind_the_ray() {
    let mut w = World::default();

    let expected = {
        let mut objs = w.objects_mut();
        let outer = objs.next().unwrap();
        let inner = objs.next().unwrap();

        outer.material_mut().ambient = 1.0;
        inner.material_mut().ambient = 1.0;

        inner.material().color
    };

    let r = Ray::new(
        Point::from_point(0.0, 0.0, 0.75),
        Vector::from_vector(0.0, 0.0, -1.0),
    );

    assert_abs_diff!(w.color_at(&r).unwrap(), expected);
}
