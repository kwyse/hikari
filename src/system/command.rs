use component::Component::{self, KeysPressed, Commands};
use command::Command;
use input::Keys;
use system::System;

use std::time::Duration;

pub struct CommandSystem;

impl System for CommandSystem {
    fn update(&self, dependent: &mut Component, independent: &Component, _: &Duration) {
        if let (Commands(commands), KeysPressed(keys)) = (dependent, independent) {
            if keys.is_set(Keys::Escape) {
                commands.set(Command::Quit)
            }
        }
    }
}
