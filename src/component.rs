use util::BitVector;

/// Pieces of data that compose an entity
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Component {
    /// The absence of a component
    Empty,

    /// The world position of a component
    Position(f64, f64),

    /// The world velocity of a component
    Velocity(f64, f64),

    /// The state of the keys on the keyboard
    KeysPressed(BitVector),

    /// The current commands being issued
    Commands(BitVector),
}
