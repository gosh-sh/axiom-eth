#![warn(clippy::useless_conversion)]

use serde::{de::DeserializeOwned, Serialize};

pub use halo2_base;
pub use halo2_base::halo2_proofs;
pub use halo2_base::halo2_proofs::halo2curves;
pub use snark_verifier;
pub use snark_verifier_sdk;
pub use zkevm_hashes;

pub mod block_header;
pub mod keccak;
pub mod mpt;
pub mod receipt;
pub mod rlc;
pub mod rlp;
pub mod solidity;
pub mod storage;
pub mod transaction;
pub mod utils;

#[cfg(feature = "providers")]
pub mod providers;

// stable-rust equivalents of the former `trait_alias` declarations:
//   pub trait RawField = zkevm_hashes::util::eth_types::Field;
//   pub trait Field    = RawField + Serialize + DeserializeOwned;
pub trait RawField: zkevm_hashes::util::eth_types::Field {}
impl<T: zkevm_hashes::util::eth_types::Field> RawField for T {}
pub trait Field: RawField + Serialize + DeserializeOwned {}
impl<T: RawField + Serialize + DeserializeOwned> Field for T {}
