use std::f32::consts::PI;

use criterion::{black_box, criterion_group, criterion_main, Criterion};
use tracy::{
    math::{MatrixN, Point, Vector},
    query::{Object, World},
    rendering::{Camera, Canvas, Color, Material, Pattern, PointLight},
    shape::Sphere,
};

fn render_shaded_sphere(width: u32, height: u32) -> Canvas {
    let mut world = World::new();

    world.add(Object::new_with_material(
        Sphere,
        MatrixN::identity(4),
        Material {
            pattern: Pattern::new(Color::new(1.0, 0.2, 1.0).into()),
            ..Default::default()
        },
    ));

    world.set_light(PointLight {
        position: Point::from_point(-10., 10., -10.),
        ..Default::default()
    });

    let camera = Camera::new_with_transform(
        width,
        height,
        PI / 3.0,
        MatrixN::look_at(
            Point::from_point(0.0, 1.5, -5.0),
            Point::from_point(0.0, 1.0, 0.0),
            Vector::from_vector(0.0, 1.0, 0.0),
        ),
    );

    camera.render(&world)
}

fn shaded_sphere(c: &mut Criterion) {
    c.bench_function("shaded sphere", |b| {
        b.iter(|| render_shaded_sphere(black_box(512), black_box(512)))
    });
}

criterion_group!(benches, shaded_sphere);
criterion_main!(benches);
