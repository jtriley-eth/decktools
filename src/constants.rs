//! Ether Deck Constants
//! 
//! Contains reserved function selectors and storage indices.
//! 
//! > Note: Storage indices are not necessarily the final storage slot. Storage mappings require
//! > additional processing. See the [storage module](crate::storage) for storage slot computation.

/// ## SetAuth Selector
pub const SELECTOR_SET_AUTH: [u8; 4] = [0, 0, 0, 1];

/// ## SetThreshold Selector
pub const SELECTOR_SET_THRESHOLD: [u8; 4] = [0, 0, 0, 2];

/// ## SetShard Selector
pub const SELECTOR_SET_SHARD: [u8; 4] = [0, 0, 0, 3];

/// ## Syscall Selector
pub const SELECTOR_SYSCALL: [u8; 4] = [0, 0, 0, 4];

/// ## ID Storage Index
pub const ID_INDEX: usize = 0;

/// ## Threshold Storage Index
pub const THRESHOLD_INDEX: usize = 1;

/// ## Authorization Storage Index
pub const AUTH_INDEX: usize = 2;

/// ## Shard Storage Index
pub const SHARD_INDEX: usize = 3;
