use component::Component;

pub mod map;
pub mod sequence;

pub trait Storage<'a> {
    fn get(&self, index: usize) -> Option<&Component>;
}

pub trait StorageMut<'a> {
    fn get_mut(&mut self, index: usize) -> Option<&mut Component>;
}
