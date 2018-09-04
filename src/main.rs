pub mod app;
pub mod command;
pub mod component;
pub mod input;
pub mod storage;
pub mod system;
pub mod util;
pub mod world;

use app::App;
use component::Component;
use world::World;

fn main() {
    let mut world = World::new();
    world.create_entity()
        .with_component(Component::Position(10.0, 10.0))
        .with_component(Component::Velocity(5.0, 1.0))
        .build();

    App::new(world).run();
}
