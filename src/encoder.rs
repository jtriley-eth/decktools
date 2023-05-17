use ethers::prelude::*;
use crate::constants::{
    SELECTOR_SET_AUTH,
    SELECTOR_SET_THRESHOLD,
    SELECTOR_SET_SHARD,
    SELECTOR_SYSCALL,
};

pub fn encode_set_auth(account: &Address, authorized: bool) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(25);
    encoded.extend_from_slice(&SELECTOR_SET_AUTH);
    encoded.extend_from_slice(&account.to_fixed_bytes());
    encoded.push(authorized as u8);
    encoded
}

pub fn encode_set_threshold(threshold: u8) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(5);
    encoded.extend_from_slice(&SELECTOR_SET_THRESHOLD);
    encoded.push(threshold);
    encoded
}

pub fn encode_set_shard(selector: &[u8; 4], shard: &Address) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(29);
    encoded.extend_from_slice(&SELECTOR_SET_SHARD);
    encoded.extend_from_slice(selector);
    encoded.extend_from_slice(&shard.to_fixed_bytes());
    encoded
}

pub fn encode_syscall(
    id: &U256,
    target: &Address,
    value: &U256,
    deadline: &U64,
    payload: &[u8],
    signatures: &[Signature],
) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(71 + payload.len() + signatures.len() * 65);
    encoded.extend_from_slice(&SELECTOR_SYSCALL);

    encoded
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_set_auth() {
        let account = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".parse::<Address>().unwrap();
        let authorized = true;
        let encoded = encode_set_auth(&account, authorized);

        assert_eq!(encoded.len(), 25);
        assert_eq!(encoded[0..4], SELECTOR_SET_AUTH);
        assert_eq!(encoded[4..24], account.to_fixed_bytes());
        assert_eq!(encoded[24], authorized as u8);
    }

    #[test]
    fn test_encode_set_threshold() {
        let threshold = 2;
        let encoded = encode_set_threshold(threshold);

        assert_eq!(encoded.len(), 5);
        assert_eq!(encoded[0..4], SELECTOR_SET_THRESHOLD);
    }

    #[test]
    fn test_encode_set_shard() {
        let selector = &[1, 2, 3, 4];
        let shard = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266".parse::<Address>().unwrap();
        let encoded = encode_set_shard(selector, &shard);

        assert_eq!(encoded.len(), 28);
        assert_eq!(encoded[0..4], SELECTOR_SET_SHARD);
        assert_eq!(encoded[4..8], *selector);
        assert_eq!(encoded[8..28], shard.to_fixed_bytes());
    }
}
