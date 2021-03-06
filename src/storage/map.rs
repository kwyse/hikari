use super::{Storage, StorageMut};
use component::Component;

use std::collections::HashMap;
use std::collections::hash_map::IterMut;

pub struct MapStorage(HashMap<usize, Component>);

impl MapStorage {
    pub fn new() -> Self {
        MapStorage(HashMap::new())
    }

    pub fn add(&mut self, index: usize, component: Component) {
        self.0.insert(index, component);
    }
}

impl<'a> Storage<'a> for &'a MapStorage {
    fn get(&self, index: usize) -> Option<&Component> {
        self.0.get(&index)
    }
}

impl<'a> StorageMut<'a> for &'a mut MapStorage {
    fn get_mut(&mut self, index: usize) -> Option<&mut Component> {
        self.0.get_mut(&index)
    }
}

impl<'a> IntoIterator for &'a mut MapStorage {
    type Item = (usize, &'a mut Component);
    type IntoIter = MapStorageIter<'a>;

    fn into_iter(self) -> Self::IntoIter {
        MapStorageIter(self.0.iter_mut())
    }
}

pub struct MapStorageIter<'a>(IterMut<'a, usize, Component>);

impl<'a> Iterator for MapStorageIter<'a> {
    type Item = (usize, &'a mut Component);

    fn next(&mut self) -> Option<Self::Item> {
        self.0.next().map(|(index, component)| (*index, component))
    }
}
