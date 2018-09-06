use util::BitVectorStorage;

/// Player-issued commands
pub enum Command {
    Quit,
}

impl Into<BitVectorStorage> for Command {
    fn into(self) -> BitVectorStorage {
        self as BitVectorStorage
    }
}
