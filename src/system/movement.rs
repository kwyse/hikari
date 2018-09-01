use component::Component;
use system::System;

use std::time::Duration;

pub struct MovementSystem;

impl System for MovementSystem {
    fn update(&self, dependent: &mut Component, independent: &Component, delta: &Duration) {
        if let Component::Position(pos_x, pos_y) = dependent {
            if let Component::Velocity(vel_x, vel_y) = independent {
                *pos_x += vel_x * (delta.as_secs() as f64);
                *pos_y += vel_y * (delta.as_secs() as f64);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn position_is_updated() {
        let mut pos = Component::Position(2.0, 2.0);
        let vel = Component::Velocity(1.0, 1.0);

        let duration = Duration::new(3, 0);
        let system = MovementSystem;
        system.update(&mut pos, &vel, &duration);

        assert_eq!(pos, Component::Position(5.0, 5.0));
    }
}