use util::BitVectorStorage;

/// Keys on a keyboard
pub enum Keys {
    Escape,
    W,
    S,
    A,
    D,
}

impl Into<BitVectorStorage> for Keys {
    fn into(self) -> BitVectorStorage {
        self as BitVectorStorage
    }
}
