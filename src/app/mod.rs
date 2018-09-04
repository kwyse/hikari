use self::run::{ExecutionFlow, ExecutionLoop};
use command::Command;
use component::Component::Commands;
use storage::Storage;
use system::System;
use system::command::CommandSystem;
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
            self.systems.command.run(&mut self.world.commands, &self.world.keys, &delta);
            self.systems.movement.run(&mut self.world.positions, &self.world.velocities, &delta);

            if let Some(player_id) = self.world.player_id() {
                if let Some(player_commands) = (&self.world.commands).get(player_id) {
                    if let Commands(commands) = player_commands {
                        if commands.is_set(Command::Quit) {
                            return ExecutionFlow::Quit;
                        }
                    }
                }
            }

            ExecutionFlow::Continue
        })
    }
}

struct Systems {
    command: CommandSystem,
    movement: MovementSystem,
}

impl Systems {
    fn new() -> Self {
        Self {
            command: CommandSystem,
            movement: MovementSystem,
        }
    }
}
