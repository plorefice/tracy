use std::convert::Infallible;

use cucumber_rust::{async_trait, given, then, WorldInit};
use tracy::query::World;

#[derive(WorldInit)]
pub struct TestRunner {
    world: World,
}

#[async_trait(?Send)]
impl cucumber_rust::World for TestRunner {
    type Error = Infallible;

    async fn new() -> Result<Self, Infallible> {
        Ok(Self {
            world: World::new(),
        })
    }
}

#[given("w ← world()")]
async fn given_a_world(tr: &mut TestRunner) {
    tr.world = World::new();
}

#[then("w contains no objects")]
async fn world_contains_no_objects(tr: &mut TestRunner) {
    assert_eq!(tr.world.objects().count(), 0);
}

#[then("w has no light source")]
async fn world_has_no_light_source(tr: &mut TestRunner) {
    assert_eq!(tr.world.lights().count(), 0);
}

#[tokio::main]
async fn main() {
    let runner = TestRunner::init(&["./features/ch07"]);
    runner.run_and_exit().await;
}
