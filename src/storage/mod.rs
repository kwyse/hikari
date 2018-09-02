use component::Component;

pub mod sequence;

pub trait Storage<'a> {
    fn get(&self, index: usize) -> Option<&Component>;
}

pub trait StorageMut<'a> {
    type StorageIterMut: Iterator<Item = &'a mut Component>;

    fn get_mut(&mut self, index: usize) -> Option<&mut Component>;
}
