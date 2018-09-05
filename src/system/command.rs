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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_keys_results_in_unmodified_commands() {
        let mut commands = Commands(0.into());
        let keys = KeysPressed(0.into());

        let system = CommandSystem;
        let duration = Duration::new(0, 0);

        system.update(&mut commands, &keys, &duration);
        assert_eq!(commands, Commands(0.into()));

        commands = Commands(0b10.into());
        system.update(&mut commands, &keys, &duration);
        assert_eq!(commands, Commands(0b10.into()));
    }

    #[test]
    fn escape_key_signals_quit() {
        use util::BitVector;

        let mut commands = Commands(0.into());
        let mut keys_bits: BitVector = 0.into();
        keys_bits.set(Keys::Escape);
        let keys = KeysPressed(keys_bits);

        let system = CommandSystem;
        let duration = Duration::new(0, 0);

        system.update(&mut commands, &keys, &duration);
        let mut commands_bits: BitVector = 0.into();
        commands_bits.set(Command::Quit);
        assert_eq!(commands, Commands(commands_bits));
    }
}
