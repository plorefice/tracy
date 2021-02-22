use std::f32::consts::{FRAC_1_SQRT_2, PI};

use tracy::{
    math::{Matrix, Point3, Vec3, EPSILON},
    query::{Ray, World},
    rendering::{
        Camera, Color, Material, Pattern, PatternKind, PointLight, DEFAULT_RECURSION_DEPTH,
    },
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
        position: Point3::new(-10., 10., -10.),
        ..Default::default()
    };

    let mut s1 = sphere();
    s1.set_material(Material {
        pattern: Pattern::new(Color::new(0.8, 1.0, 0.6).into()),
        diffuse: 0.7,
        specular: 0.2,
        ..Default::default()
    });

    let mut s2 = sphere();
    s2.set_transform(Matrix::from_scale(0.5, 0.5, 0.5));

    let w = World::default();
    let mut objs = w.objects();

    assert_eq!(w.lights().next(), Some(&light));

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

    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::unit_z());

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
    let s = w.add(sphere());

    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::unit_z());

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 4.).abs() < EPSILON)
        .unwrap();

    assert_eq!(interference.handle, s);
    assert_f32!(interference.toi, 4.);
    assert_abs_diff!(interference.point, Point3::new(0.0, 0.0, -1.0));
    assert_abs_diff!(interference.eye, Vec3::new(0.0, 0.0, -1.0));
    assert_abs_diff!(interference.normal, Vec3::new(0.0, 0.0, -1.0));
}

#[test]
fn the_hit_when_an_intersection_occurs_on_the_outside() {
    let mut w = World::new();
    w.add(sphere());

    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::unit_z());

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 4.).abs() < EPSILON)
        .unwrap();

    assert!(!interference.inside);
}

#[test]
fn the_hit_when_an_intersection_occurs_on_the_inside() {
    let mut w = World::new();
    w.add(sphere());

    let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::unit_z());

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 1.).abs() < EPSILON)
        .unwrap();

    assert_abs_diff!(interference.point, Point3::new(0.0, 0.0, 1.0));
    assert_abs_diff!(interference.eye, Vec3::new(0.0, 0.0, -1.0));
    assert_abs_diff!(interference.normal, Vec3::new(0.0, 0.0, -1.0));
    assert!(interference.inside);
}

#[test]
fn shading_an_intersection() {
    let w = World::default();
    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::unit_z());

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 4.).abs() < EPSILON)
        .unwrap();

    assert_abs_diff!(
        w.shade_hit(&interference, DEFAULT_RECURSION_DEPTH),
        Color::new(0.38066, 0.47583, 0.2855)
    );
}

#[test]
fn shading_an_intersection_from_the_inside() {
    let mut w = World::default();

    *w.lights_mut().next().unwrap() = PointLight {
        position: Point3::new(0.0, 0.25, 0.0),
        ..Default::default()
    };

    let r = Ray::new(Point3::new(0.0, 0.0, 0.0), Vec3::unit_z());

    let interference = w
        .interferences_with_ray(&r)
        .find(|i| (i.toi - 0.5).abs() < EPSILON)
        .unwrap();

    assert_abs_diff!(
        w.shade_hit(&interference, DEFAULT_RECURSION_DEPTH),
        Color::new(0.90498, 0.90498, 0.90498)
    );
}

#[test]
fn the_color_when_a_ray_misses() {
    let w = World::default();
    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::unit_y());

    assert_abs_diff!(w.color_at(&r, DEFAULT_RECURSION_DEPTH), Color::BLACK);
}

#[test]
fn the_color_when_a_ray_hits() {
    let w = World::default();
    let r = Ray::new(Point3::new(0.0, 0.0, -5.0), Vec3::unit_z());

    assert_abs_diff!(
        w.color_at(&r, DEFAULT_RECURSION_DEPTH),
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

        match inner.material().pattern.kind() {
            PatternKind::Solid(c) => *c,
            _ => unreachable!("invalid pattern in material"),
        }
    };

    let r = Ray::new(Point3::new(0.0, 0.0, 0.75), Vec3::new(0.0, 0.0, -1.0));

    assert_abs_diff!(w.color_at(&r, DEFAULT_RECURSION_DEPTH), expected);
}

#[test]
fn the_transform_matrix_for_the_default_orientation() {
    let eye = Point3::new(0.0, 0.0, 0.0);
    let center = Point3::new(0.0, 0.0, -1.0);
    let up = Vec3::unit_y();

    let t = Matrix::look_at(eye, center, up);
    assert_abs_diff!(t, Matrix::identity(4));
}

#[test]
fn a_view_transformation_matrix_looking_in_positive_z_direction() {
    let eye = Point3::new(0.0, 0.0, 0.0);
    let center = Point3::new(0.0, 0.0, 1.0);
    let up = Vec3::unit_y();

    let t = Matrix::look_at(eye, center, up);
    assert_abs_diff!(dbg!(t), Matrix::from_scale(-1.0, 1.0, -1.0));
}

#[test]
fn the_view_transformation_moves_the_world() {
    let eye = Point3::new(0.0, 0.0, 8.0);
    let center = Point3::new(0.0, 0.0, 0.0);
    let up = Vec3::unit_y();

    let t = Matrix::look_at(eye, center, up);
    assert_abs_diff!(t, Matrix::from_translation(0.0, 0.0, -8.0));
}

#[test]
fn an_arbitrary_view_transformation() {
    let eye = Point3::new(1.0, 3.0, 2.0);
    let center = Point3::new(4.0, -2.0, 8.0);
    let up = Vec3::new(1.0, 1.0, 0.0);

    let t = Matrix::look_at(eye, center, up);

    assert_abs_diff!(
        t,
        Matrix::from_row_slice(
            4,
            [
                -0.50709, 0.50709, 0.67612, -2.36643, 0.76772, 0.60609, 0.12122, -2.82843,
                -0.35857, 0.59761, -0.71714, 0.00000, 0.00000, 0.00000, 0.00000, 1.00000,
            ]
        )
    );
}

#[test]
fn constructing_a_camera() {
    let c = Camera::new(160, 120, PI / 2.0);

    assert_eq!(c.horizontal_size(), 160);
    assert_eq!(c.vertical_size(), 120);
    assert_f32!(c.fov(), PI / 2.);
    assert_abs_diff!(c.view_transform(), Matrix::identity(4));
}

#[test]
fn the_pixel_size_for_a_horizontal_canvas() {
    let c = Camera::new(200, 125, PI / 2.0);
    assert_f32!(c.pixel_size(), 0.01);
}

#[test]
fn the_pixel_size_for_a_vertical_canvas() {
    let c = Camera::new(125, 200, PI / 2.0);
    assert_f32!(c.pixel_size(), 0.01);
}

#[test]
fn constructing_a_ray_through_the_center_of_the_canvas() {
    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_to(100, 50);

    assert_abs_diff!(r.origin, Point3::new(0.0, 0.0, 0.0));
    assert_abs_diff!(r.dir, Vec3::new(0.0, 0.0, -1.0));
}

#[test]
fn constructing_a_ray_through_a_corner_of_the_canvas() {
    let c = Camera::new(201, 101, PI / 2.0);
    let r = c.ray_to(0, 0);

    assert_abs_diff!(r.origin, Point3::new(0.0, 0.0, 0.0));
    assert_abs_diff!(r.dir, Vec3::new(0.66519, 0.33259, -0.66851));
}

#[test]
fn constructing_a_ray_when_the_camera_is_transformed() {
    let c = Camera::new_with_transform(
        201,
        101,
        PI / 2.0,
        Matrix::from_rotation_y(PI / 4.0) * Matrix::from_translation(0.0, -2.0, 5.0),
    );

    let r = c.ray_to(100, 50);

    assert_abs_diff!(r.origin, Point3::new(0.0, 2.0, -5.0));
    assert_abs_diff!(r.dir, Vec3::new(FRAC_1_SQRT_2, 0.0, -FRAC_1_SQRT_2));
}

#[test]
fn rendering_a_world_with_a_camera() {
    let w = World::default();

    let c = Camera::new_with_transform(
        11,
        11,
        PI / 2.0,
        Matrix::look_at(
            Point3::new(0.0, 0.0, -5.0),
            Point3::new(0.0, 0.0, 0.0),
            Vec3::unit_y(),
        ),
    );

    let canvas = c.render(&w);

    assert_abs_diff!(
        canvas.get(5, 5).unwrap(),
        Color::new(0.38066, 0.47583, 0.2855)
    );
}
