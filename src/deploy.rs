//! # Ether Deck Depoyer Module
//! 
//! Contains a deployer function for the deck.
use crate::error::Error;
use ethers::{providers::Middleware, types::transaction::eip2718::TypedTransaction, types::*};
use std::borrow::Borrow;

/// ## Ether Deck Bytecode
pub static ETHER_DECK_BYTECODE: Bytes = Bytes::from_static(__BYTECODE);

/// ## Deploys the Ether Deck.
pub async fn deploy<M: Middleware, B: Borrow<M>>(client: B) -> Result<Address, Error> {
    let tx: TypedTransaction = Eip1559TransactionRequest {
        to: None,
        data: Some(ETHER_DECK_BYTECODE.clone()),
        ..Default::default()
    }
    .into();

    client
        .borrow()
        .send_transaction(tx, None)
        .await
        .map_err(|e| Error::Deployment(e.to_string()))?
        .confirmations(1)
        .await
        .map_err(|e| Error::Deployment(e.to_string()))?
        .ok_or(Error::Deployment("No receipt".to_string()))?
        .contract_address
        .ok_or(Error::Deployment("No contract address".to_string()))
}

const __BYTECODE: &[u8] = &[
    0x60, 0x01, 0x80, 0x60, 0x01, 0x55, 0x33, 0x60, 0x00, 0x52, 0x60, 0x02, 0x60, 0x20, 0x52, 0x60,
    0x40, 0x60, 0x00, 0x20, 0x55, 0x61, 0x03, 0x3d, 0x80, 0x60, 0x1f, 0x3d, 0x39, 0x3d, 0xf3, 0x60,
    0x00, 0x35, 0x60, 0xe0, 0x1c, 0x80, 0x60, 0x04, 0x14, 0x61, 0x00, 0x8a, 0x57, 0x80, 0x60, 0x01,
    0x14, 0x61, 0x01, 0xed, 0x57, 0x80, 0x60, 0x02, 0x14, 0x61, 0x02, 0x62, 0x57, 0x80, 0x60, 0x03,
    0x14, 0x61, 0x02, 0xc5, 0x57, 0x60, 0xe0, 0x1b, 0x60, 0x03, 0x60, 0x20, 0x52, 0x60, 0x00, 0x52,
    0x60, 0x40, 0x60, 0x00, 0x20, 0x54, 0x80, 0x15, 0x61, 0x00, 0x60, 0x57, 0x36, 0x60, 0x00, 0x60,
    0x00, 0x37, 0x60, 0x00, 0x36, 0x60, 0x00, 0x60, 0x00, 0x93, 0x5a, 0xf4, 0x3d, 0x60, 0x00, 0x60,
    0x00, 0x3e, 0x61, 0x00, 0x5b, 0x57, 0x3d, 0x60, 0x00, 0xfd, 0x5b, 0x3d, 0x60, 0x00, 0xf3, 0x5b,
    0x7f, 0xef, 0x16, 0x0e, 0x8e, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x60, 0x00, 0x52, 0x60, 0x04, 0x60, 0x00, 0xfd, 0x5b, 0x60, 0x43, 0x35, 0x60, 0xc0, 0x1c,
    0x42, 0x10, 0x61, 0x00, 0xc0, 0x57, 0x7f, 0xc6, 0xf2, 0x21, 0xaa, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x00, 0x52, 0x60, 0x04, 0x60, 0x00, 0xfd, 0x5b,
    0x60, 0x4b, 0x35, 0x60, 0xe0, 0x1c, 0x80, 0x80, 0x60, 0x4f, 0x01, 0x90, 0x60, 0x4b, 0x01, 0x60,
    0x01, 0x54, 0x90, 0x80, 0x60, 0x04, 0x60, 0x00, 0x37, 0x46, 0x81, 0x52, 0x60, 0x20, 0x01, 0x60,
    0x00, 0x20, 0x60, 0x1c, 0x7f, 0x19, 0x45, 0x74, 0x68, 0x65, 0x72, 0x65, 0x75, 0x6d, 0x20, 0x53,
    0x69, 0x67, 0x6e, 0x65, 0x64, 0x20, 0x4d, 0x65, 0x73, 0x73, 0x61, 0x67, 0x65, 0x3a, 0x0a, 0x33,
    0x32, 0x00, 0x00, 0x00, 0x00, 0x60, 0x00, 0x52, 0x52, 0x60, 0x3c, 0x60, 0x00, 0x20, 0x60, 0x00,
    0x52, 0x60, 0x41, 0x02, 0x81, 0x01, 0x90, 0x60, 0x00, 0x90, 0x60, 0x01, 0x5b, 0x81, 0x60, 0x41,
    0x01, 0x91, 0x80, 0x35, 0x60, 0xf8, 0x1c, 0x60, 0x20, 0x52, 0x60, 0x40, 0x90, 0x60, 0x01, 0x01,
    0x60, 0x40, 0x37, 0x60, 0x80, 0x60, 0x20, 0x81, 0x60, 0x80, 0x60, 0x00, 0x60, 0x01, 0x5a, 0xfa,
    0x50, 0x51, 0x92, 0x83, 0x11, 0x83, 0x60, 0x80, 0x52, 0x60, 0x02, 0x60, 0xa0, 0x52, 0x60, 0x40,
    0x60, 0x80, 0x20, 0x54, 0x16, 0x16, 0x83, 0x82, 0x10, 0x61, 0x01, 0x1d, 0x57, 0x61, 0x01, 0x8b,
    0x57, 0x7f, 0xab, 0x8a, 0x03, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x60, 0x00, 0x52, 0x60, 0x04, 0x60, 0x00, 0xfd, 0x5b, 0x50, 0x50, 0x50, 0x60, 0x00,
    0x54, 0x80, 0x7f, 0xd5, 0x65, 0x24, 0x1e, 0x29, 0x36, 0x34, 0xc5, 0x09, 0xaf, 0x5a, 0x0f, 0x5f,
    0x7b, 0x4c, 0x3f, 0xea, 0x30, 0xfb, 0x14, 0x14, 0x78, 0xd0, 0xc9, 0xf9, 0xa2, 0xe2, 0xd2, 0xe6,
    0xa3, 0xcf, 0x7c, 0x60, 0x00, 0x60, 0x00, 0xa2, 0x60, 0x01, 0x01, 0x60, 0x00, 0x55, 0x80, 0x60,
    0x4f, 0x60, 0x00, 0x37, 0x60, 0x00, 0x60, 0x00, 0x91, 0x60, 0x00, 0x60, 0x38, 0x35, 0x60, 0xa8,
    0x1c, 0x60, 0x24, 0x35, 0x60, 0x60, 0x1c, 0x5a, 0xf1, 0x3d, 0x60, 0x00, 0x60, 0x00, 0x3e, 0x61,
    0x01, 0xe8, 0x57, 0x3d, 0x60, 0x00, 0xfd, 0x5b, 0x3d, 0x60, 0x00, 0xf3, 0x5b, 0x33, 0x30, 0x14,
    0x61, 0x02, 0x1e, 0x57, 0x7f, 0xab, 0x8a, 0x03, 0x60, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x00, 0x52, 0x60, 0x04, 0x60, 0x00, 0xfd, 0x5b, 0x60, 0x18,
    0x35, 0x60, 0xf8, 0x1c, 0x60, 0x04, 0x35, 0x60, 0x60, 0x1c, 0x81, 0x81, 0x60, 0x00, 0x52, 0x60,
    0x02, 0x60, 0x20, 0x52, 0x60, 0x40, 0x60, 0x00, 0x20, 0x55, 0x7f, 0x5b, 0x11, 0x78, 0x62, 0x68,
    0x98, 0x64, 0xea, 0xda, 0x25, 0x61, 0x17, 0x27, 0x62, 0xc0, 0xae, 0x6d, 0x3e, 0x90, 0x16, 0xf3,
    0xe6, 0x3d, 0x39, 0x28, 0xc0, 0x20, 0xee, 0xdc, 0x26, 0xe7, 0xc1, 0x60, 0x00, 0x60, 0x00, 0xa3,
    0x00, 0x5b, 0x33, 0x30, 0x14, 0x61, 0x02, 0x93, 0x57, 0x7f, 0xab, 0x8a, 0x03, 0x60, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x00, 0x52, 0x60, 0x04, 0x60,
    0x00, 0xfd, 0x5b, 0x60, 0x04, 0x35, 0x60, 0xf8, 0x1c, 0x80, 0x60, 0x01, 0x55, 0x7f, 0x19, 0xbd,
    0xbd, 0x56, 0xc4, 0xc0, 0x49, 0xb2, 0xd3, 0xa1, 0x18, 0x69, 0x73, 0xe7, 0x69, 0x05, 0xfb, 0x68,
    0x43, 0xc0, 0xed, 0xe2, 0x9b, 0x88, 0x1e, 0x1e, 0x2a, 0x34, 0xe3, 0xbd, 0xbd, 0x9a, 0x60, 0x00,
    0x60, 0x00, 0xa2, 0x00, 0x5b, 0x33, 0x30, 0x14, 0x61, 0x02, 0xf6, 0x57, 0x7f, 0xab, 0x8a, 0x03,
    0x60, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00,
    0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x00, 0x60, 0x00, 0x52,
    0x60, 0x04, 0x60, 0x00, 0xfd, 0x5b, 0x60, 0x08, 0x35, 0x60, 0x60, 0x1c, 0x60, 0x04, 0x35, 0x60,
    0xe0, 0x1c, 0x60, 0xe0, 0x1b, 0x81, 0x81, 0x60, 0x03, 0x60, 0x20, 0x52, 0x60, 0x00, 0x52, 0x60,
    0x40, 0x60, 0x00, 0x20, 0x55, 0x7f, 0x92, 0x65, 0x51, 0x64, 0x11, 0x85, 0x43, 0x1a, 0x7a, 0xd6,
    0x58, 0x51, 0x2f, 0xae, 0x1a, 0x08, 0x8f, 0x1c, 0xef, 0xda, 0x6f, 0xf1, 0xb7, 0xe4, 0x80, 0xeb,
    0x55, 0x9e, 0x9c, 0x47, 0x00, 0xb8, 0x60, 0x00, 0x60, 0x00, 0xa3, 0x00,
];
