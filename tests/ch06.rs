use std::{convert::Infallible, f32};

use cucumber_rust::{async_trait, given, then, when, WorldInit};
use trtc::{
    canvas::Color,
    math::{MatrixN, Point, Vector},
    query::{Object, Ray, RayCast},
    rendering::{phong_lighting, Material, PointLight},
    shape::{ShapeHandle, Sphere},
};

const EPSILON: f32 = 1e-4;

#[derive(WorldInit)]
pub struct TestRunner {
    sphere: Option<Object>,
    m: MatrixN,
    v: Vector,
    n: Vector,
    r: Vector,
    ns: Vec<Vector>,

    light: PointLight,
    color: Color,
    position: Point,
    material: Material,

    eyev: Vector,
    normalv: Vector,
    result: Color,
}

#[async_trait(?Send)]
impl cucumber_rust::World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            sphere: None,
            m: MatrixN::zeros(4),
            v: Vector::default(),
            n: Vector::default(),
            r: Vector::default(),
            ns: Vec::new(),

            light: PointLight::default(),
            color: Color::default(),
            position: Point::default(),
            material: Material::default(),

            eyev: Vector::default(),
            normalv: Vector::default(),
            result: Color::default(),
        })
    }
}

#[given("s ← sphere()")]
async fn given_a_sphere(tr: &mut TestRunner) {
    tr.sphere = Some(Object::new(ShapeHandle::new(Sphere), MatrixN::identity(4)));
}

#[given("m ← material()")]
async fn given_a_material(tr: &mut TestRunner) {
    tr.material = Material::default();
}

#[given("set_transform(s, m)")]
async fn given_a_transformed_sphere(tr: &mut TestRunner) {
    tr.sphere.as_mut().unwrap().set_transform(tr.m.clone());
}

#[given(regex = r"^set_transform\(s, translation\((.*), (.*), (.*)\)\)$")]
async fn given_a_translated_sphere(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.sphere
        .as_mut()
        .unwrap()
        .set_transform(MatrixN::from_translation(x, y, z));
}

#[given(regex = r"^m ← scaling\((.*), (.*), (.*)\) \* rotation_z\((.*)\)$")]
async fn given_a_scale_and_rotation(tr: &mut TestRunner, x: f32, y: f32, z: f32, rad: f32) {
    tr.m = MatrixN::from_scale(x, y, z) * MatrixN::from_rotation_z(rad);
}

#[given(regex = r"^v ← vector\((.*), (.*), (.*)\)$")]
async fn given_a_vector(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.v = Vector::from_vector(x, y, z);
}

#[given(regex = r"^n ← vector\((.*), (.*), (.*)\)$")]
async fn given_a_normal(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.n = Vector::from_vector(x, y, z);
}

#[given(regex = r"^intensity ← color\((.*), (.*), (.*)\)$")]
async fn given_light_intensity(tr: &mut TestRunner, r: f32, g: f32, b: f32) {
    tr.color = Color::new(r, g, b);
}

#[given(regex = r"^position ← point\((.*), (.*), (.*)\)$")]
async fn given_light_position(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.position = Point::from_point(x, y, z);
}

#[given(regex = r"^m\.ambient ← (.*)$")]
async fn given_an_ambient_value(tr: &mut TestRunner, ambient: f32) {
    tr.material.ambient = ambient;
}

#[given(regex = r"^eyev ← vector\((.*), (.*), (.*)\)$")]
async fn given_an_eye_vector(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.eyev = Vector::from_vector(x, y, z);
}

#[given(regex = r"^normalv ← vector\((.*), (.*), (.*)\)$")]
async fn given_a_normal_vector(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    tr.normalv = Vector::from_vector(x, y, z);
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

#[when(regex = r"^n ← normal_at\(s, point\((.*), (.*), (.*)\)\)$")]
async fn compute_normal(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    let sphere = tr.sphere.as_ref().unwrap();

    tr.ns = sphere
        .shape()
        .toi_and_normal_with_ray(
            sphere.transform(),
            &Ray::new(Point::default(), Vector::from_vector(x, y, z)),
        )
        .map(|xs| xs.map(|x| x.normal).collect())
        .unwrap_or_default();
}

#[when("r ← reflect(v, n)")]
async fn reflect_vector(tr: &mut TestRunner) {
    tr.r = tr.v.reflect(&tr.n);
}

#[when("light ← point_light(position, intensity)")]
async fn create_point_light(tr: &mut TestRunner) {
    tr.light = PointLight {
        position: tr.position,
        color: tr.color,
        intensity: 1.,
    };
}

#[when("m ← s.material")]
async fn sphere_to_material(tr: &mut TestRunner) {
    tr.material = *tr.sphere.as_ref().unwrap().material();
}

#[when("s.material ← m")]
async fn material_to_sphere(tr: &mut TestRunner) {
    tr.sphere.as_mut().unwrap().set_material(tr.material);
}

#[when("result ← lighting(m, light, position, eyev, normalv)")]
async fn compute_lighting(tr: &mut TestRunner) {
    tr.result = phong_lighting(&tr.material, &tr.light, &tr.position, &tr.eyev, &tr.normalv);
}

#[then(regex = r"^n = vector\((.*), (.*), (.*)\)$")]
async fn check_normal(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!(tr
        .ns
        .iter()
        .any(|n| n.abs_diff_eq(&Vector::from_vector(x, y, z), EPSILON)));
}

#[then("n = normalize(n)")]
async fn normals_are_normalized(tr: &mut TestRunner) {
    assert!(tr.ns.iter().all(|n| n.abs_diff_eq(&n.normalize(), EPSILON)));
}

#[then(regex = r"^r = vector\((.*), (.*), (.*)\)$")]
async fn check_reflection(tr: &mut TestRunner, x: f32, y: f32, z: f32) {
    assert!(tr.r.abs_diff_eq(&Vector::from_vector(x, y, z), EPSILON));
}

#[then("light.position = position")]
async fn check_light_position(tr: &mut TestRunner) {
    assert!(tr.light.position.abs_diff_eq(&tr.position, EPSILON));
}

#[then("light.intensity = intensity")]
async fn check_light_intensity(tr: &mut TestRunner) {
    assert!(tr.light.color.abs_diff_eq(&tr.color, EPSILON));
}

#[then(regex = r"^m.color = color\((.*), (.*), (.*)\)$")]
async fn check_material_color(tr: &mut TestRunner, r: f32, g: f32, b: f32) {
    assert!(tr.material.color.abs_diff_eq(&Color::new(r, g, b), EPSILON));
}

#[then(regex = r"^m.(ambient|diffuse|specular|shininess) = (.*)$")]
async fn check_material_properties(tr: &mut TestRunner, field: String, val: f32) {
    let field = match field.as_str() {
        "ambient" => tr.material.ambient,
        "diffuse" => tr.material.diffuse,
        "specular" => tr.material.specular,
        "shininess" => tr.material.shininess,
        _ => unreachable!("invalid field name"),
    };
    assert!((field - val).abs() < EPSILON);
}

#[then("m = material()")]
async fn material_is_default(tr: &mut TestRunner) {
    assert_eq!(tr.material, Material::default());
}

#[then("s.material = m")]
async fn sphere_material(tr: &mut TestRunner) {
    assert_eq!(*tr.sphere.as_ref().unwrap().material(), tr.material);
}

#[then(regex = r"^result = color\((.*), (.*), (.*)\)$")]
async fn check_result(tr: &mut TestRunner, r: f32, g: f32, b: f32) {
    assert!(tr.result.abs_diff_eq(&Color::new(r, g, b), 1e-2));
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch06"]);
    runner.run_and_exit().await;
}
