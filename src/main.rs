mod component;
mod storage;
mod system;
mod world;

use component::Component;
use storage::Storage;
use storage::map::MapStorage;
use storage::sequence::SequenceStorage;
use system::System;
use system::movement::MovementSystem;
use world::World;

use std::time::Duration;

fn main() {
    let mut positions = SequenceStorage::new();
    positions.add(0, Component::Position(0.0, 0.0));
    positions.add(1, Component::Position(1.0, 1.0));

    let mut velocities = MapStorage::new();
    velocities.add(0, Component::Velocity(3.0, 3.0));

    let duration = Duration::new(9, 0);

    let movement_system = MovementSystem;
    movement_system.run(&mut positions, &velocities, &duration);

    println!("Position1: {:?}", (&positions).get(0).unwrap());
    println!("Position2: {:?}", (&positions).get(1).unwrap());

    let mut world = World::new();
    world.create_entity()
        .with_component(Component::Position(10.0, 10.0))
        .build();
}
