mod component;
mod storage;
mod system;

use component::Component;
use storage::{Storage, SequenceStorage};
use system::System;
use system::movement::MovementSystem;

use std::time::Duration;

fn main() {
    let mut positions = SequenceStorage::new();
    positions.add(Component::Position(0.0, 0.0));

    let mut velocities = SequenceStorage::new();
    velocities.add(Component::Velocity(3.0, 3.0));

    let duration = Duration::new(9, 0);

    let movement_system = MovementSystem;
    movement_system.run(&mut positions, &velocities, &duration);

    println!("Position: {:?}", (&positions).get(0).unwrap());
}
