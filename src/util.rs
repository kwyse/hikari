/// A 64-bit unsigned integer-backed bit vector
#[derive(Clone, Copy, Debug, PartialEq)]
pub struct BitVector(u64);

impl BitVector {
    pub fn new() -> Self {
        BitVector(0)
    }

    pub fn is_set(&self, bit: u64) -> bool {
        self.0 & 1 << bit as u64 != 0
    }

    pub fn set(&mut self, bit: u64) {
        self.0 |= 1 << bit as u64
    }

    pub fn unset(&mut self, bit: u64) {
        self.0 &= !(1 << bit as u64)
    }
}

impl From<u64> for BitVector {
    fn from(value: u64) -> Self {
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
        assert!(vec.is_set(0));
        assert!(!vec.is_set(1));
        assert!(vec.is_set(2));
        assert!(vec.is_set(3));
    }

    #[test]
    fn set_bits() {
        let mut vec: BitVector = 0b0.into();
        assert!(!vec.is_set(0));
        assert!(!vec.is_set(1));
        assert!(!vec.is_set(2));

        vec.set(1);
        assert!(!vec.is_set(0));
        assert!(vec.is_set(1));
        assert!(!vec.is_set(2));
    }

    #[test]
    fn unset_bits() {
        let mut vec: BitVector = 0b110.into();
        assert!(!vec.is_set(0));
        assert!(vec.is_set(1));
        assert!(vec.is_set(2));

        vec.unset(1);
        assert!(!vec.is_set(0));
        assert!(!vec.is_set(1));
        assert!(vec.is_set(2));
    }
}
