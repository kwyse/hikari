/// The underlying storage type for `BitVector`
pub type BitVectorStorage = u128;

/// A 128-bit unsigned integer-backed bit vector
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitVector(BitVectorStorage);

impl BitVector {
    pub fn new() -> Self {
        BitVector(0)
    }

    pub fn is_set<T: Into<BitVectorStorage>>(&self, bit: T) -> bool {
        self.0 & 1 << bit.into() != 0
    }

    pub fn set<T: Into<BitVectorStorage>>(&mut self, bit: T) {
        self.0 |= 1 << bit.into()
    }

    pub fn unset<T: Into<BitVectorStorage>>(&mut self, bit: T) {
        self.0 &= !(1 << bit.into())
    }
}

impl From<BitVectorStorage> for BitVector {
    fn from(value: BitVectorStorage) -> Self {
        BitVector(value)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn initialize_to_empty() {
        assert_eq!(BitVector::new(), 0.into());
    }

    #[test]
    fn check_set_bits() {
        let vec: BitVector = 0b1101.into();
        assert!(vec.is_set(0_u8));
        assert!(!vec.is_set(1_u8));
        assert!(vec.is_set(2_u8));
        assert!(vec.is_set(3_u8));
    }

    #[test]
    fn set_bits() {
        let mut vec: BitVector = 0b0.into();
        assert!(!vec.is_set(0_u8));
        assert!(!vec.is_set(1_u8));
        assert!(!vec.is_set(2_u8));

        vec.set(1_u8);
        assert!(!vec.is_set(0_u8));
        assert!(vec.is_set(1_u8));
        assert!(!vec.is_set(2_u8));
    }

    #[test]
    fn unset_bits() {
        let mut vec: BitVector = 0b110.into();
        assert!(!vec.is_set(0_u8));
        assert!(vec.is_set(1_u8));
        assert!(vec.is_set(2_u8));

        vec.unset(1_u8);
        assert!(!vec.is_set(0_u8));
        assert!(!vec.is_set(1_u8));
        assert!(vec.is_set(2_u8));
    }
}
