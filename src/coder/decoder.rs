//! # Ether Deck Decoding Module
//!
//! Contains functions for decoding each of the Ether Deck contract's methods.
use crate::{
    coder::structures::{Encoding, SetAuth, SetShard, SetThreshold, Syscall},
    constants::{SELECTOR_SET_AUTH, SELECTOR_SET_SHARD, SELECTOR_SET_THRESHOLD, SELECTOR_SYSCALL},
    error::Error,
};
use ethers::prelude::{Address, Signature, U256, U64};

/// ## Decode Calldata
///
/// ### Parameters
///
/// - `calldata`: The calldata to decode.
///
/// ### Returns
///
/// Either an [Encoding](crate::coder::structures::Encoding) field or an
/// [Error](crate::error::Error).
pub fn decode(calldata: &[u8]) -> Result<Encoding, Error> {
    let encoding = match calldata.try_into().map_err(|_| Error::CalldataLength)? {
        SELECTOR_SET_AUTH => Encoding::SetAuth(decode_set_auth(calldata)?),
        SELECTOR_SET_THRESHOLD => Encoding::SetThreshold(decode_set_threshold(calldata)?),
        SELECTOR_SET_SHARD => Encoding::SetShard(decode_set_shard(calldata)?),
        SELECTOR_SYSCALL => Encoding::Syscall(decode_syscall(calldata)?),
        _ => return Err(Error::SelectorMismatch),
    };
    Ok(encoding)
}

/// ## Decode setAuth Call
/// 
/// ### Parameters
/// 
/// - `calldata`: The calldata to decode.
/// 
/// ### Returns
/// 
/// Either a [SetAuth](crate::coder::structures::SetAuth) field or an [Error](crate::error::Error).
pub fn decode_set_auth(calldata: &[u8]) -> Result<SetAuth, Error> {
    if calldata.len() != 25 {
        return Err(Error::CalldataLength);
    }

    Ok(SetAuth::new(
        &Address::from_slice(&calldata[4..24]),
        calldata[24] == 1,
    ))
}

/// ## Decode setThreshold Call
/// 
/// ### Parameters
/// 
/// - `calldata`: The calldata to decode.
/// 
/// ### Returns
/// 
/// Either a [SetThreshold](crate::coder::structures::SetThreshold) structure or an
/// [Error](crate::error::Error).
pub fn decode_set_threshold(calldata: &[u8]) -> Result<SetThreshold, Error> {
    if calldata.len() != 5 {
        return Err(Error::CalldataLength);
    }

    Ok(SetThreshold::new(calldata[4]))
}

/// ## Decode setShard Call
/// 
/// ### Parameters
/// 
/// - `calldata`: The calldata to decode.
/// 
/// ### Returns
/// 
/// Either a [SetShard](crate::coder::structures::SetShard) structure or an
/// [Error](crate::error::Error).
pub fn decode_set_shard(calldata: &[u8]) -> Result<SetShard, Error> {
    if calldata.len() != 29 {
        return Err(Error::CalldataLength);
    }

    let selector: [u8; 4] = calldata[4..8]
        .try_into()
        .map_err(|_| Error::CalldataLength)?;

    Ok(SetShard::new(
        &selector,
        &Address::from_slice(&calldata[8..]),
    ))
}

/// ## Decode syscall Call
/// 
/// ### Parameters
/// 
/// - `calldata`: The calldata to decode.
/// 
/// ### Returns
/// 
/// Either a [Syscall](crate::coder::structures::Syscall) structure or an
/// [Error](crate::error::Error).
pub fn decode_syscall(calldata: &[u8]) -> Result<Syscall, Error> {
    if calldata.len() < 100 {
        return Err(Error::CalldataLength);
    }

    let payload_len = u32::from_be_bytes(calldata[96..100].try_into().unwrap()) as usize;

    if calldata.len() != 100 + payload_len {
        return Err(Error::CalldataLength);
    }

    let id = U256::from_big_endian(&calldata[4..36]);
    let target = Address::from_slice(&calldata[36..56]);
    let value = U256::from_big_endian(&calldata[56..67]); // TODO: verify this does not panic.
    let deadline = U64::from_big_endian(&calldata[67..75]);
    let payload = &calldata[79..(79 + payload_len)];

    let sigs_len = calldata.len() - (79 + payload_len);
    if sigs_len % 65 != 0 {
        return Err(Error::MalformedCalldata)
    }

    let mut sigs = Vec::with_capacity(sigs_len / 65);

    for i in 0..sigs_len {
        let sig_start = 79 + payload_len + (i * 65);
        let sig_stop = 79 + payload_len + ((i + 1) * 65);
        // we unwrap here, as the only fail case is `bytes.len() != 65`, which we have already
        // validated.
        let sig = Signature::try_from(&calldata[sig_start..sig_stop]).unwrap();
        sigs.push(sig);
    }

    Ok(Syscall::new(&id, &target, &value, &deadline, payload, &sigs))
}

#[cfg(test)]
mod tests {
    // TODO: test these lol
}
