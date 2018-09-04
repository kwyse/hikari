/// Player-issued commands
pub enum Command {
    Quit,
}

impl Into<u64> for Command {
    fn into(self) -> u64 {
        self as u64
    }
}
