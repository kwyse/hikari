mod component;
mod storage;
mod system;

use component::Component;
use storage::Storage;
use storage::sequence::SequenceStorage;
use system::System;
use system::movement::MovementSystem;

use std::time::Duration;

fn main() {
    let mut positions = SequenceStorage::new();
    positions.add(Component::Position(0.0, 0.0));
    positions.add(Component::Position(1.0, 1.0));

    let mut velocities = SequenceStorage::new();
    velocities.add(Component::Velocity(3.0, 3.0));

    let duration = Duration::new(9, 0);

    let movement_system = MovementSystem;
    movement_system.run(&mut positions, &velocities, &duration);

    println!("Position1: {:?}", (&positions).get(0).unwrap());
    println!("Position2: {:?}", (&positions).get(1).unwrap());
}
