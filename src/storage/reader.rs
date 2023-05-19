//! # Ether Deck Storage Reader Module
//! 
//! Contains functoins for reading the deck's storage.
use crate::{
    error::Error,
    storage::slot::{auth_slot, id_slot, shard_slot, threshold_slot},
    util::u256_to_be,
};
use ethers::prelude::*;

/// ## Read ID
pub async fn read_id<M>(
    provider: M,
    deck: &Address,
) -> Result<U256, Error>
where
    M: Middleware + 'static,
{
    provider
        .get_storage_at(*deck, H256::from_slice(&u256_to_be(&id_slot())), None)
        .await
        .map(|res| U256::from(res.as_ref()))
        .map_err(|_| Error::Middleware("Failed to read ID".to_string()))
}

/// ## Read Threshold
pub async fn read_threshold<M>(
    provider: M,
    deck: &Address,
) -> Result<u8, Error>
where
    M: Middleware + 'static,
{
    provider
        .get_storage_at(
            *deck,
            H256::from_slice(&u256_to_be(&threshold_slot())),
            None,
        )
        .await
        .map(|res| res.as_ref()[31])
        .map_err(|_| Error::Middleware("Failed to read threshold".to_string()))
}

/// ## Read Authorization
pub async fn read_auth<M>(
    provider: M,
    deck: &Address,
    account: &Address,
) -> Result<bool, Error>
where
    M: Middleware + 'static,
{
    provider
        .get_storage_at(
            *deck,
            H256::from_slice(&auth_slot(account)),
            None,
        )
        .await
        .map(|res| res.as_ref()[31] == 1)
        .map_err(|_| Error::Middleware("Failed to read authorization".to_string()))
}

/// ## Read Shard
pub async fn read_shard<M>(
    provider: M,
    deck: &Address,
    selector: &[u8; 4],
) -> Result<Address, Error>
where
    M: Middleware + 'static,
{
    provider
        .get_storage_at(
            *deck,
            H256::from_slice(&shard_slot(selector)),
            None,
        )
        .await
        .map(|res| Address::from_slice(res.as_ref()))
        .map_err(|_| Error::Middleware("Failed to read shard".to_string()))
}
