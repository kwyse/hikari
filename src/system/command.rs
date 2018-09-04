use component::Component::{self, KeysPressed, Commands};
use command::QUIT;
use input::ESCAPE;
use system::System;

use std::time::Duration;

pub struct CommandSystem;

impl System for CommandSystem {
    fn update(&self, dependent: &mut Component, independent: &Component, _: &Duration) {
        if let (Commands(commands), KeysPressed(keys)) = (dependent, independent) {
            if keys & (1 << ESCAPE) != 0 {
                *commands |= 1 << QUIT
            }
        }
    }
}
