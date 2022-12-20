use frame_support::pallet_prelude::*;
use parity_scale_codec::{Decode, Encode};
use serde::{Deserialize, Deserializer};
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

use crate::constants::TEE_ORACLE_MAX_URI_LENGTH;

pub type TeeOracleURI = BoundedVec<u8, ConstU32<TEE_ORACLE_MAX_URI_LENGTH>>;

// ref: https://serde.rs/container-attrs.html#crate
#[derive(Deserialize, Encode, Decode, Default, RuntimeDebug, scale_info::TypeInfo)]
pub struct HackerNewsInfo {
    // Specify our own deserializing function to convert JSON string to vector of bytes
    #[serde(deserialize_with = "de_string_to_bytes")]
    pub by: Vec<u8>,
    #[serde(deserialize_with = "de_string_to_bytes")]
    pub title: Vec<u8>,
    #[serde(deserialize_with = "de_string_to_bytes")]
    pub url: Vec<u8>,
    pub descendants: u32,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct Payload<Public> {
    pub number: u64,
    pub public: Public,
}

#[derive(Debug, Deserialize, Encode, Decode, Default)]
pub struct IndexingData(Vec<u8>, u64);

pub fn de_string_to_bytes<'de, D>(de: D) -> Result<Vec<u8>, D::Error>
where
    D: Deserializer<'de>,
{
    let s: &str = Deserialize::deserialize(de)?;
    Ok(s.as_bytes().to_vec())
}
