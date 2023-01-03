use ap_region::Country;
use ap_region::Region;
use ap_region::SubRegion;
use codec::Decode;
use codec::Encode;
use codec::MaxEncodedLen;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::cmp::Eq;
use sp_std::cmp::PartialEq;
use sp_std::vec::Vec;

use crate::constants::DELIVERY_NETWORK_MAX_URI_LEN;
use crate::constants::REGISTRY_INFO_MAX_LEN;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct Payload<Public, BlockNumber, AcountId> {
  pub public: Public,
  pub block_number: BlockNumber,
  pub account_id: AcountId,
  pub number: u32,
}

pub type RegistryId = [u8; 12];
pub type RegistryInfo = BoundedVec<u8, ConstU32<REGISTRY_INFO_MAX_LEN>>;
pub type RegistryHash = [u8; 32];
pub type ChunkHash = [u8; 32];
pub type ChunkId = [u8; 44]; // RegistryId + ChunkHash

pub type DeliveryNetworkURI = BoundedVec<u8, ConstU32<DELIVERY_NETWORK_MAX_URI_LEN>>;

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub enum Accessibility {
  New,
  Healthy,
  Broken,
  Deleted,
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
  pub delivery_network_id: AccountId,
  pub owner_id: AccountId,
  pub issuer_id: AccountId,
  pub author_id: AccountId,
  pub hash: RegistryHash,
  pub info: RegistryInfo,
  pub status: Accessibility,
  pub salable: bool,
  pub country: Country,
  pub region: Region,
  pub sub_region: SubRegion,
  pub accessor_count: u32,
  pub chunk_hashes: Vec<ChunkHash>,
}

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo)]
pub struct Chunk<BlockNumber> {
  pub last_block: BlockNumber,
  pub status: Accessibility,
}
