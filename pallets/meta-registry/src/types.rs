#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::cmp::{Eq, PartialEq};
use sp_std::vec::Vec;

use common::types::{Country, Region, SubRegion};

use crate::constants::REGISTRY_INFO_MAX_LEN;

pub type RegistryId = BoundedVec<u8, ConstU32<64>>;
pub type RegistryInfo = BoundedVec<u8, ConstU32<REGISTRY_INFO_MAX_LEN>>;
pub type RegistryHash = [u8; 32];
pub type ChunkHash = [u8; 32];

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub enum Accessibility {
    Healthy,
    Broken,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen, Copy)]
pub enum AccessType {
    Issuer,
    Owner,
    Accessor,
    Buyer,
    Aggregator,
}

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Registry<AccountId> {
    pub owner: AccountId,
    pub issuer: AccountId,
    pub hash: RegistryHash,
    pub info: RegistryInfo,
    pub salable: bool,
    pub country: Country,
    pub region: Region,
    pub sub_region: SubRegion,
    pub accessors: u32,
    pub status: Accessibility,
    pub chunk_hashes: Vec<ChunkHash>,
}

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Chunk<BlockNumber> {
    pub last_block: BlockNumber,
    pub status: Accessibility,
}
