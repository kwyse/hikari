use self::run::{ExecutionFlow, ExecutionLoop};
use command::Command;
use component::Component::{Commands, KeysPressed};
use input::Keys;
use storage::{Storage, StorageMut};
use system::System;
use system::command::CommandSystem;
use system::movement::MovementSystem;
use world::World;

use winit::{Event, EventsLoop, KeyboardInput, VirtualKeyCode, Window, WindowEvent};

pub mod run;

/// Container for systems that update a world in the execution loop
pub struct App {
    world: World,
    systems: Systems,
    events: EventsLoop,
}

impl App {
    /// Passes ownership of a `World` to this `App`
    pub fn new(world: World) -> Self {
        Self {
            world,
            systems: Systems::new(),
            events: EventsLoop::new(),
        }
    }

    /// Runs the execution loop on its `World`
    pub fn run(mut self) {
        let _window = Window::new(&self.events).unwrap();

        ExecutionLoop::new(60).run(|delta| {
            let id = self.world.player_id();

            // TODO: This is wasteful becuase the events are iterated through twice!
            let mut events = Vec::new();
            self.events.poll_events(|event| events.push(event));
            for event in &events {
                if let Some(player_id) = id {
                    if let Some(player_keys) = (&mut self.world.keys).get_mut(player_id) {
                        if let KeysPressed(keys) = player_keys {
                            if let Event::WindowEvent { event: WindowEvent::KeyboardInput { input, .. }, .. } = event {
                                if let KeyboardInput { virtual_keycode: Some(key), .. } = input {
                                    if *key == VirtualKeyCode::Escape {
                                        keys.set(Keys::Escape)
                                    }
                                }
                            }
                        }
                    }
                }
            }

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
