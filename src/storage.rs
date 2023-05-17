use crate::constants::{AUTH_INDEX, ID_INDEX, SHARD_INDEX, THRESHOLD_INDEX};
use ethers::{
    abi::{encode, Token},
    prelude::{Address, U256},
    utils::keccak256,
};

pub fn id_slot() -> U256 {
    U256::from(ID_INDEX)
}

pub fn threshold_slot() -> U256 {
    U256::from(THRESHOLD_INDEX)
}

pub fn auth_slot(account: &Address) -> [u8; 32] {
    keccak256(&encode(&[
        Token::Address(*account),
        Token::Uint(U256::from(AUTH_INDEX)),
    ]))
}

pub fn shard_slot(selector: &[u8; 4]) -> [u8; 32] {
    keccak256(&encode(&[
        Token::FixedBytes(selector.to_vec()),
        Token::Uint(U256::from(SHARD_INDEX)),
    ]))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_id_slot() {
        assert_eq!(id_slot(), U256::zero());
    }

    #[test]
    fn test_threshold_slot() {
        assert_eq!(threshold_slot(), U256::one());
    }

    #[test]
    fn test_auth_slot() {
        let account = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
            .parse::<Address>()
            .unwrap();
        let slot = auth_slot(&account);

        let hash = [
            0xbc, 0x40, 0xfb, 0xf4, 0x39, 0x4c, 0xd0, 0x0f, 0x78, 0xfa, 0xe9, 0x76, 0x3b, 0x0c,
            0x2c, 0x71, 0xb2, 0x1e, 0xa4, 0x42, 0xc4, 0x2f, 0xda, 0xdc, 0x5b, 0x72, 0x05, 0x37,
            0x24, 0x0e, 0xba, 0xc1,
        ];
        assert_eq!(slot, hash);
    }

    #[test]
    fn test_shard_slot() {
        let shard = [1, 2, 3, 4];
        let slot = shard_slot(&shard);

        let hash = [
            0x42, 0x3a, 0x8c, 0x70, 0xa6, 0x8e, 0xba, 0x50, 0x7f, 0x4a, 0xfd, 0x18, 0x3e, 0xdd,
            0x7b, 0xab, 0x39, 0x47, 0xad, 0x8d, 0xdf, 0xf2, 0xb7, 0xb7, 0x00, 0xb7, 0x6b, 0xe6,
            0xe2, 0x34, 0x79, 0x35,
        ];
        assert_eq!(slot, hash);
    }
}
