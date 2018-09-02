use component::Component;
use storage::{Storage, StorageMut};

use std::time::Duration;

pub mod keys;
pub mod movement;

pub trait System {
    fn update(&self, dependent: &mut Component, independent: &Component, delta: &Duration);

    fn run<'a, A, B>(&self, dependents: A, independents: B, delta: &Duration)
    where
        A: StorageMut<'a> + IntoIterator<Item = (usize, &'a mut Component)>,
        B: Storage<'a>,
    {
        for (index, mut dependent) in dependents.into_iter().filter(not_empty) {
            if let Some(independent) = independents.get(index).and_then(exists) {
                self.update(&mut dependent, independent, delta);
            }
        }
    }
}

fn not_empty((_, component): &(usize, &mut Component)) -> bool {
    **component != Component::Empty
}

fn exists(component: &Component) -> Option<&Component> {
    if *component != Component::Empty { Some(component) } else { None }
}

#[cfg(test)]
mod tests {
    use super::*;
    use component::Component::{Empty, Position, Velocity};
    use storage::sequence::SequenceStorage;

    use std::time::Duration;

    struct StubSystem;

    impl System for StubSystem {
        fn update(&self, dependent: &mut Component, independent: &Component, _: &Duration) {
            if let (Position(pos_x, pos_y), Velocity(vel_x, vel_y)) = (dependent, independent) {
                *pos_x += vel_x;
                *pos_y += vel_y;
            }
        }
    }

    #[test]
    fn dependent_is_empty() {
        let mut positions = SequenceStorage::new();
        let mut velocities = SequenceStorage::new();

        positions.add(0, Empty);
        velocities.add(0, Velocity(2.0, 2.0));

        let delta = Duration::new(0, 0);
        StubSystem.run(&mut positions, &velocities, &delta);

        assert_eq!(positions.as_slice(), [Empty]);
        assert_eq!(velocities.as_slice(), [Velocity(2.0, 2.0)]);
    }

    #[test]
    fn independent_is_empty() {
        let mut positions = SequenceStorage::new();
        let mut velocities = SequenceStorage::new();

        positions.add(0, Position(1.0, 1.0));
        velocities.add(0, Empty);

        let delta = Duration::new(0, 0);
        StubSystem.run(&mut positions, &velocities, &delta);

        assert_eq!(positions.as_slice(), [Position(1.0, 1.0)]);
        assert_eq!(velocities.as_slice(), [Empty]);
    }

    #[test]
    fn dependent_is_aligned_with_independent() {
        let mut positions = SequenceStorage::new();
        let mut velocities = SequenceStorage::new();

        positions.add(0, Position(1.0, 1.0));
        velocities.add(0, Velocity(2.0, 2.0));

        let delta = Duration::new(0, 0);
        StubSystem.run(&mut positions, &velocities, &delta);

        assert_eq!(positions.as_slice(), [Position(3.0, 3.0)]);
        assert_eq!(velocities.as_slice(), [Velocity(2.0, 2.0)]);
    }
}
