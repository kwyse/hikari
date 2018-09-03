use self::run::{ExecutionFlow, ExecutionLoop};
use system::System;
use system::movement::MovementSystem;
use world::World;

pub mod run;

/// Container for systems that update a world in the execution loop
pub struct App {
    world: World,
    systems: Systems,
}

impl App {
    /// Passes ownership of a `World` to this `App`
    pub fn new(world: World) -> Self {
        Self {
            world,
            systems: Systems::new(),
        }
    }

    /// Runs the execution loop on its `World`
    pub fn run(mut self) {
        ExecutionLoop::new(60).run(|delta| {
            self.systems.movement.run(&mut self.world.positions, &self.world.velocities, &delta);
            ExecutionFlow::Quit
        })
    }
}

struct Systems {
    movement: MovementSystem,
}

impl Systems {
    fn new() -> Self {
        Self {
            movement: MovementSystem,
        }
    }
}
