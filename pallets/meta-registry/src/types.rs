#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::cmp::{Eq, PartialEq};
use sp_std::vec::Vec;

use region::{Country, Region, SubRegion};

use crate::constants::{DELIVERY_NETWORK_MAX_URI_LEN, REGISTRY_INFO_MAX_LEN};

pub type DeliveryNetworkId = BoundedVec<u8, ConstU32<64>>;
pub type RegistryId = BoundedVec<u8, ConstU32<64>>;
pub type RegistryInfo = BoundedVec<u8, ConstU32<REGISTRY_INFO_MAX_LEN>>;
pub type RegistryHash = [u8; 32];
pub type ChunkHash = [u8; 32];

pub type DeliveryNetworkURI = BoundedVec<u8, ConstU32<DELIVERY_NETWORK_MAX_URI_LEN>>;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub enum Accessibility {
    New,
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
pub struct DeliveryNetwork {
    pub uri: DeliveryNetworkURI,
    pub country: Option<Country>,
    pub region: Option<Region>,
    pub sub_region: Option<SubRegion>,
}

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Registry<AccountId> {
    pub delivery_network_id: DeliveryNetworkId,
    pub owner_id: AccountId,
    pub issuer_id: AccountId,
    pub hash: RegistryHash,
    pub info: RegistryInfo,
    pub salable: bool,
    pub country: Country,
    pub region: Region,
    pub sub_region: SubRegion,
    pub accessors: u32,
    pub chunk_hashes: Vec<ChunkHash>,
}

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Chunk<BlockNumber> {
    pub registry_id: RegistryId,
    pub last_block: BlockNumber,
    pub status: Accessibility,
}
