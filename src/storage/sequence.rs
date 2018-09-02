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
        self.1.next().map(|item| {
            let ret = (self.0, item);
            self.0 += 1;

            ret
        })
    }
}
