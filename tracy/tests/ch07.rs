use std::convert::Infallible;

use cucumber_rust::{async_trait, gherkin::Step, given, then, when, WorldInit};
use tracy::{
    canvas::Color,
    math::{MatrixN, Point},
    query::{Object, World},
    rendering::{Material, PointLight},
    shape::{ShapeHandle, Sphere},
};

#[derive(WorldInit)]
pub struct TestRunner {
    world: World,
    light: PointLight,
    ss: Vec<Object>,
}

#[async_trait(?Send)]
impl cucumber_rust::World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            world: World::new(),
            light: PointLight::default(),
            ss: Vec::new(),
        })
    }
}

#[given("w ← world()")]
async fn given_a_world(tr: &mut TestRunner) {
    tr.world = World::new();
}

#[allow(clippy::many_single_char_names)]
#[given(regex = r"^light ← point_light\(point\((.*), (.*), (.*)\), color\((.*), (.*), (.*)\)\)$")]
async fn given_a_point_light(tr: &mut TestRunner, x: f32, y: f32, z: f32, r: f32, g: f32, b: f32) {
    tr.light = PointLight {
        position: Point::from_point(x, y, z),
        color: Color::new(r, g, b),
        intensity: 1.,
    };
}

#[given(regex = r"s([12]) ← sphere\(\) with:")]
async fn given_a_sphere(tr: &mut TestRunner, step: &Step, idx: String) {
    let idx = idx.parse::<usize>().unwrap() - 1;

    let mut transform = MatrixN::identity(4);
    let mut mat = Material::default();

    for cols in &step.table().unwrap().rows {
        let (ident, val) = (&cols[0], &cols[1]);

        match ident.as_str() {
            "material.color" => {
                let mut rgb = val
                    .trim_matches(|c| c == '(' || c == ')')
                    .split(',')
                    .map(|v| v.trim().parse::<f32>().unwrap());

                mat.color.r = rgb.next().unwrap();
                mat.color.g = rgb.next().unwrap();
                mat.color.b = rgb.next().unwrap();
            }
            "material.diffuse" => mat.diffuse = val.parse::<f32>().unwrap(),
            "material.specular" => mat.specular = val.parse::<f32>().unwrap(),
            "transform" => {
                let words = val.split('(').collect::<Vec<_>>();

                match words[0] {
                    "scaling" => {
                        let mut xyz = words[1]
                            .trim_matches(|c| c == '(' || c == ')')
                            .split(',')
                            .map(|v| v.trim().parse::<f32>().unwrap());

                        transform = MatrixN::from_scale(
                            xyz.next().unwrap(),
                            xyz.next().unwrap(),
                            xyz.next().unwrap(),
                        );
                    }
                    _ => unreachable!("unexpected transform value"),
                }
            }
            _ => unreachable!("unexpected property name"),
        }
    }

    assert_eq!(idx, tr.ss.len());

    tr.ss.push(Object::new_with_material(
        ShapeHandle::new(Sphere),
        transform,
        mat,
    ));
}

#[when("w ← default_world()")]
async fn world_is_default(tr: &mut TestRunner) {
    tr.world = World::default();
}

#[then("w contains no objects")]
async fn world_contains_no_objects(tr: &mut TestRunner) {
    assert_eq!(tr.world.objects().count(), 0);
}

#[then("w has no light source")]
async fn world_has_no_light_source(tr: &mut TestRunner) {
    assert_eq!(tr.world.lights().count(), 0);
}

#[then("w.light = light")]
async fn world_has_light(tr: &mut TestRunner) {
    assert_eq!(*tr.world.lights().next().unwrap(), tr.light);
}

#[then(regex = r"w contains s([12])")]
async fn world_contains_sphere(tr: &mut TestRunner, idx: String) {
    let idx = idx.parse::<usize>().unwrap() - 1;

    let res = tr.world.objects().nth(idx).unwrap();
    let exp = &tr.ss[idx];

    assert_eq!(res.transform(), exp.transform());
    assert_eq!(res.material(), exp.material());
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch07"]);
    runner.run_and_exit().await;
}
