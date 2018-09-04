/// Keys on a keyboard
pub enum Keys {
    Escape,
    W,
    S,
    A,
    D,
}

impl Into<u64> for Keys {
    fn into(self) -> u64 {
        self as u64
    }
}
