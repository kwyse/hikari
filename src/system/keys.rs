use component::Component::{self, KeysPressed, Velocity};
use input::Keys;
use system::System;

use std::time::Duration;

pub struct KeysSystem;

impl System for KeysSystem {
    fn update(&self, dependent: &mut Component, independent: &Component, _: &Duration) {
        if let (Velocity(x, y), KeysPressed(state)) = (dependent, independent) {
            let process = |key, func: &mut FnMut()| if state.is_set(key) { func() };

            process(Keys::W, &mut || *y += 1.0);
            process(Keys::S, &mut || *y -= 1.0);
            process(Keys::A, &mut || *x -= 1.0);
            process(Keys::D, &mut || *x += 1.0);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn no_keys_pressed() {
        let mut velocity = Velocity(1.0, 0.0);
        let delta = Duration::new(0, 0);
        KeysSystem.update(&mut velocity, &KeysPressed(0.into()), &delta);

        assert_eq!(velocity, Velocity(1.0, 0.0));
    }

    #[test]
    fn one_movement_key_pressed() {
        let mut velocity = Velocity(1.0, 0.0);
        let delta = Duration::new(0, 0);
        KeysSystem.update(&mut velocity, &KeysPressed((1_u128 << Keys::W as u64).into()), &delta);

        assert_eq!(velocity, Velocity(1.0, 1.0));
    }

    #[test]
    fn multiple_movement_keys_pressed() {
        let mut velocity = Velocity(1.0, 0.0);
        let delta = Duration::new(0, 0);
        let keys_state = KeysPressed((1_u128 << Keys::W as u64 | 1_u128 << Keys::D as u64).into());
        KeysSystem.update(&mut velocity, &keys_state.into(), &delta);

        assert_eq!(velocity, Velocity(2.0, 1.0));
    }
}
