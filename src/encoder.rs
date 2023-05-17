//! # Ether Deck Encoder Module
//! 
//! Contains functions for encoding each of the Ether Deck contract's methods
use crate::{
    constants::{SELECTOR_SET_AUTH, SELECTOR_SET_SHARD, SELECTOR_SET_THRESHOLD, SELECTOR_SYSCALL},
    error::Error,
    util::{u256_to_be, u64_to_be},
};
use ethers::prelude::{U256, U64, Address, Signature};


/// ## Encode setAuth Call
/// 
/// ### Parameters
/// 
/// - `account`: The account to set authorization for.
/// - `authorized`: Whether the account is authorized.
/// 
/// ### Returns
/// 
/// The encoded calldata.
pub fn encode_set_auth(account: &Address, authorized: bool) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(25);
    encoded.extend_from_slice(&SELECTOR_SET_AUTH);
    encoded.extend_from_slice(&account.to_fixed_bytes());
    encoded.push(authorized as u8);
    encoded
}

/// ## Encode setThreshold Call
/// 
/// ### Parameters
/// 
/// - `threshold`: The new threshold.
/// 
/// ### Returns
/// 
/// The encoded calldata.
pub fn encode_set_threshold(threshold: u8) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(5);
    encoded.extend_from_slice(&SELECTOR_SET_THRESHOLD);
    encoded.push(threshold);
    encoded
}

/// ## Encode setShard Call
/// 
/// ### Parameters
/// 
/// - `selector`: The selector to set the shard for.
/// - `shard`: The new shard.
/// 
/// ### Returns
/// 
/// The encoded calldata.
pub fn encode_set_shard(selector: &[u8; 4], shard: &Address) -> Vec<u8> {
    let mut encoded = Vec::with_capacity(29);
    encoded.extend_from_slice(&SELECTOR_SET_SHARD);
    encoded.extend_from_slice(selector);
    encoded.extend_from_slice(&shard.to_fixed_bytes());
    encoded
}

/// ## Encode syscall Call
/// 
/// ### Parameters
/// 
/// - `id`: The syscall id (nonce).
/// - `target`: The target contract address.
/// - `value`: The value to send.
/// - `deadline`: The deadline for the call.
/// - `payload`: The payload to send.
/// - `signatures`: The signatures to include.
/// 
/// ### Returns
/// 
/// The encoded calldata.
pub fn encode_syscall(
    id: &U256,
    target: &Address,
    value: &U256,
    deadline: &U64,
    payload: &[u8],
    signatures: &[Signature],
) -> Result<Vec<u8>, Error> {
    if value > &U256::from("309485009821345068724781055") {
        return Err(Error::CallValueOverflow);
    }

    if payload.len() > u32::MAX as usize {
        return Err(Error::PayloadLengthOverfow);
    }

    let value_u88 = &u256_to_be(value)[21..32];

    let packed_sigs: Vec<u8> = signatures
        .iter()
        .map(|sig| <[u8; 65]>::from(sig))
        .flatten()
        .collect();

    let mut encoded = Vec::with_capacity(100 + payload.len() + 65 * signatures.len());
    encoded.extend_from_slice(&SELECTOR_SYSCALL);
    encoded.extend_from_slice(&u256_to_be(id));
    encoded.extend_from_slice(&target.to_fixed_bytes());
    encoded.extend_from_slice(&value_u88);
    encoded.extend_from_slice(&u64_to_be(deadline));
    encoded.extend_from_slice((payload.len() as u32).to_be_bytes().as_ref());
    encoded.extend_from_slice(payload);
    encoded.extend_from_slice(&packed_sigs);

    Ok(encoded)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_encode_set_auth() {
        let account = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
            .parse::<Address>()
            .unwrap();
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
        let shard = "0xf39Fd6e51aad88F6F4ce6aB8827279cffFb92266"
            .parse::<Address>()
            .unwrap();
        let encoded = encode_set_shard(selector, &shard);

        assert_eq!(encoded.len(), 28);
        assert_eq!(encoded[0..4], SELECTOR_SET_SHARD);
        assert_eq!(encoded[4..8], *selector);
        assert_eq!(encoded[8..28], shard.to_fixed_bytes());
    }

    // TODO: test encode_syscall
}
