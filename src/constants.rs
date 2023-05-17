use ethers::prelude::*;

pub const SELECTOR_SET_AUTH: [u8; 4] = [0, 0, 0, 1];
pub const SELECTOR_SET_THRESHOLD: [u8; 4] = [0, 0, 0, 2];
pub const SELECTOR_SET_SHARD: [u8; 4] = [0, 0, 0, 3];
pub const SELECTOR_SYSCALL: [u8; 4] = [0, 0, 0, 4];

pub const ID_INDEX: usize = 0;
pub const THRESHOLD_INDEX: usize = 1;
pub const AUTH_INDEX: usize = 2;
pub const SHARD_INDEX: usize = 3;
