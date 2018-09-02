use super::{Storage, StorageMut};
use component::Component;

use std::slice::IterMut;

pub struct SequenceStorage(Vec<Component>);

impl SequenceStorage {
    pub fn new() -> Self {
        SequenceStorage(Vec::new())
    }

    pub fn add(&mut self, component: Component) {
        self.0.push(component)
    }
}

impl<'a> Storage<'a> for &'a SequenceStorage {
    fn get(&self, index: usize) -> Option<&Component> {
        self.0.get(index)
    }
}

impl<'a> StorageMut<'a> for &'a mut SequenceStorage {
    fn get_mut(&mut self, index: usize) -> Option<&mut Component> {
        self.0.get_mut(index)
    }
}

impl<'a> IntoIterator for &'a mut SequenceStorage {
    type Item = (usize, &'a mut Component);
    type IntoIter = SequenceStorageIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        SequenceStorageIter(0, self.0.iter_mut())
    }
}

pub struct SequenceStorageIter<'a>(usize, IterMut<'a, Component>);

impl<'a> Iterator for SequenceStorageIter<'a> {
    type Item = (usize, &'a mut Component);

    fn next(&mut self) -> Option<Self::Item> {
        self.1.next().map(|component| {
            let index = self.0;
            self.0 += 1;

            (index, component)
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn iterator_yields_index_and_component() {
        let mut seq = SequenceStorage::new();
        seq.add(Component::Empty);
        seq.add(Component::Empty);

        let mut iter = seq.into_iter();
        assert_eq!(iter.next(), Some((0, &mut Component::Empty)));
        assert_eq!(iter.next(), Some((1, &mut Component::Empty)));
        assert_eq!(iter.next(), None);
    }
}