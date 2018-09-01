use component::Component;
use storage::{Storage, StorageMut};

use std::time::Duration;

mod movement;

pub trait System {
    fn update(&self, dependent: &mut Component, independent: &Component, delta: &Duration);

    fn run<'a, A, B>(&self, dependents: A, independents: B, delta: &Duration)
    where
        A: StorageMut<'a> + IntoIterator<Item = &'a mut Component>,
        B: Storage<'a>,
    {
        for (index, mut dependent) in dependents.into_iter().enumerate() {
            if let Some(independent) = independents.get(index) {
                self.update(&mut dependent, independent, delta);
            }
        }
    }
}
