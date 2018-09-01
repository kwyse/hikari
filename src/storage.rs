use component::Component;

use std::slice::IterMut;

pub trait Storage<'a> {
    fn get(&self, index: usize) -> Option<&Component>;
}

pub trait StorageMut<'a> {
    type StorageIterMut: Iterator<Item = &'a mut Component>;

    fn get_mut(&mut self, index: usize) -> Option<&mut Component>;
}

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
    type StorageIterMut = IterMut<'a, Component>;

    fn get_mut(&mut self, index: usize) -> Option<&mut Component> {
        self.0.get_mut(index)
    }
}

impl<'a> IntoIterator for &'a mut SequenceStorage {
    type Item = &'a mut Component;
    type IntoIter = IterMut<'a, Component>;

    fn into_iter(self) -> Self::IntoIter {
        self.0.iter_mut()
    }
}
