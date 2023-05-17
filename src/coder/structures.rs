//! # Ether Deck Coder Structures
//! 
//! Contains data structures for encoding and decoding each of the Ether Deck contract's methods.
use ethers::prelude::{U256, U64, Address, Signature};

/// ## Set Auth Encoding
pub struct SetAuth {
    account: Address,
    authorized: bool,
}

impl SetAuth {
    /// ### Create New Auth Encoding
    /// 
    /// #### Parameters
    /// 
    /// - `account`: The account to set authorization for.
    /// - `authorized`: Whether the account is authorized.
    pub fn new(account: &Address, authorized: bool) -> Self {
        Self { account: *account, authorized }
    }

    /// ### Get Account
    pub fn account(&self) -> &Address {
        &self.account
    }

    /// ### Get Authorized
    pub fn authorized(&self) -> bool {
        self.authorized
    }
}

/// ## Set Threshold Encoding
pub struct SetThreshold {
    pub threshold: u8,
}

impl SetThreshold {
    /// ### Create New Threshold Encoding
    /// 
    /// #### Parameters
    /// 
    /// - `threshold`: The new threshold.
    pub fn new(threshold: u8) -> Self {
        Self { threshold }
    }

    /// ### Get Threshold
    pub fn threshold(&self) -> u8 {
        self.threshold
    }
}

/// ## Set Shard Encoding
pub struct SetShard {
    selector: [u8; 4],
    shard: Address,
}

impl SetShard {
    /// ### Create New Shard Encoding
    /// 
    /// #### Parameters
    /// 
    /// - `selector`: The selector to set the shard for.
    /// - `shard`: The new shard.
    pub fn new(selector: &[u8; 4], shard: &Address) -> Self {
        Self {
            selector: *selector,
            shard: *shard,
        }
    }

    /// ### Get Selector
    pub fn selector(&self) -> &[u8; 4] {
        &self.selector
    }

    /// ### Get Shard
    pub fn shard(&self) -> &Address {
        &self.shard
    }
}

/// ## Syscall Encoding
pub struct Syscall {
    id: U256,
    target: Address,
    value: U256,
    deadline: U64,
    payload: Vec<u8>,
    signatures: Vec<Signature>,
}

impl Syscall {
    /// ### Create New Syscall Encoding
    /// 
    /// #### Parameters
    /// 
    /// - `id`: The unique ID.
    /// - `target`: The target.
    /// - `value`: The call value.
    /// - `deadline`: The deadline.
    /// - `payload`: The payload.
    /// - `signatures`: The signature list.
    pub fn new(
        id: &U256,
        target: &Address,
        value: &U256,
        deadline: &U64,
        payload: &[u8],
        signatures: &[Signature],
    ) -> Self {
        Self {
            id: *id,
            target: *target,
            value: *value,
            deadline: *deadline,
            payload: payload.to_vec(),
            signatures: signatures.to_vec(),
        }
    }

    /// ### Get the unique ID
    pub fn id(&self) -> &U256 {
        &self.id
    }

    /// ### Get the target
    pub fn target(&self) -> &Address {
        &self.target
    }

    /// ### Get the call value
    pub fn value(&self) -> &U256 {
        &self.value
    }

    /// ### Get the deadline
    pub fn deadline(&self) -> &U64 {
        &self.deadline
    }

    /// ### Get the payload
    pub fn payload(&self) -> &[u8] {
        &self.payload
    }

    /// ### Get the signature list
    pub fn signatures(&self) -> &[Signature] {
        &self.signatures
    }
}

/// ## Encoding Enum
pub enum Encoding {
    /// Set Auth Encoding
    SetAuth(SetAuth),
    /// Set Threshold Encoding
    SetThreshold(SetThreshold),
    /// Set Shard Encoding
    SetShard(SetShard),
    /// Syscall Encoding
    Syscall(Syscall)
}