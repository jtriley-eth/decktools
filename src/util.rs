//! # Ether Deck Utility Module
//! 
//! Contains General purpose helpers.
use ethers::prelude::{U256, U64};

/// ## U256 to Big Endian Fixed Bytes
///
/// ### Parameters
///
/// - `value`: The value to convert.
///
/// ### Returns
///
/// The value as big endian fixed bytes.
pub fn u256_to_be(value: &U256) -> [u8; 32] {
    let mut bytes = [0u8; 32];
    value.to_big_endian(&mut bytes);
    bytes
}

/// ## U64 to Big Endian Fixed Bytes
///
/// ### Parameters
///
/// - `value`: The value to convert.
///
/// ### Returns
///
/// The value as big endian fixed bytes.
pub fn u64_to_be(value: &U64) -> [u8; 8] {
    let mut bytes = [0u8; 8];
    value.to_big_endian(&mut bytes);
    bytes
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    pub fn test_u256_to_be_zero() {
        assert_eq!(u256_to_be(&U256::zero()), [0u8; 32]);
    }

    #[test]
    pub fn test_u256_to_be_one() {
        let mut fixed = [0u8; 32];
        fixed[31] = 1;
        assert_eq!(u256_to_be(&U256::one()), fixed);
    }

    #[test]
    pub fn test_u256_to_be_max() {
        assert_eq!(u256_to_be(&U256::MAX), [0xffu8; 32]);
    }

    #[test]
    pub fn test_u64_to_be_zero() {
        assert_eq!(u64_to_be(&U64::zero()), [0u8; 8]);
    }

    #[test]
    pub fn test_u64_to_be_one() {
        let mut fixed = [0u8; 8];
        fixed[7] = 1;
        assert_eq!(u64_to_be(&U64::one()), fixed);
    }

    #[test]
    pub fn test_u64_to_be_max() {
        assert_eq!(u64_to_be(&U64::MAX), [0xffu8; 8]);
    }
}
