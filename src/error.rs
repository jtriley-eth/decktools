//! # Ether Deck Error Module

/// ## Error Enum
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Error {
    /// The call value is greater than `2**88-1`
    CallValueOverflow,
    /// The payload length is greater than `2**32-1`.
    PayloadLengthOverfow,
    /// Calldata is too short.
    CalldataLength,
    /// Selector not found.
    SelectorMismatch,
    /// Calldata is malformed.
    MalformedCalldata,
}
