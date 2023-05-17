//! # Ether Deck Error Module

/// ## Error Enumeration
/// 
/// ### Information
/// 
/// - `CallValueOverflow`: The call value is greater than `2**88-1`.
/// - `PayloadLengthOverflow`: The payload length is greater than `2**32-1`.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    CallValueOverflow,
    PayloadLengthOverfow,
}
