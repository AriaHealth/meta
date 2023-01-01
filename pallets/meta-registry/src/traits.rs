use ap_region::Country;
use frame_support::dispatch::DispatchResultWithPostInfo;
use sp_std::vec::Vec;

use crate::types::{ChunkHash, RegistryHash, RegistryId, RegistryInfo};

// traits
pub trait CustodianRules<AccountId> {
  fn is_authorized(account: &AccountId) -> bool;
}

pub trait IssuerRules<AccountId> {
  fn can_create(
    registry_id: &RegistryId,
    owner: &AccountId,
    issuer: &AccountId,
    hash: &RegistryHash,
    info: &RegistryInfo,
    salable: &bool,
    country: &Country,
    chunk_hashes: &Vec<ChunkHash>,
  ) -> bool;

  fn on_create(
    registry_id: &RegistryId,
    owner: &AccountId,
    issuer: &AccountId,
    hash: &RegistryHash,
    info: &RegistryInfo,
    salable: &bool,
    country: &Country,
    chunk_hashes: &Vec<ChunkHash>,
  ) -> DispatchResultWithPostInfo;

  fn can_delete(
    registry_id: &RegistryId,
    owner: &AccountId,
    issuer: &AccountId,
    hash: &RegistryHash,
    info: &RegistryInfo,
    salable: &bool,
    country: &Country,
    chunk_hashes: &Vec<ChunkHash>,
  ) -> bool;

  fn on_delete(
    registry_id: &RegistryId,
    owner: &AccountId,
    issuer: &AccountId,
    hash: &RegistryHash,
    info: &RegistryInfo,
    salable: &bool,
    country: &Country,
    chunk_hashes: &Vec<ChunkHash>,
  ) -> DispatchResultWithPostInfo;
}

pub trait URLTrait {
  fn is_valid(&self) -> bool;
}

pub trait CombinedKey<K1, K2, R> {
  fn generate(k1: &K1, k2: &K2) -> R;
  fn decompose(&self) -> (K1, K2);
}

// blank implementation

impl<AccountId> CustodianRules<AccountId> for () {
  fn is_authorized(account: &AccountId) -> bool {
    true
  }
}

impl<AccountId> IssuerRules<AccountId> for () {
  #[allow(unused_variables)]
  fn can_create(
    registry_id: &RegistryId,
    owner: &AccountId,
    issuer: &AccountId,
    hash: &RegistryHash,
    info: &RegistryInfo,
    salable: &bool,
    country: &Country,
    chunk_hashes: &Vec<ChunkHash>,
  ) -> bool {
    true
  }

  #[allow(unused_variables)]
  fn on_create(
    registry_id: &RegistryId,
    owner: &AccountId,
    issuer: &AccountId,
    hash: &RegistryHash,
    info: &RegistryInfo,
    salable: &bool,
    country: &Country,
    chunk_hashes: &Vec<ChunkHash>,
  ) -> DispatchResultWithPostInfo {
    Ok(().into())
  }

  #[allow(unused_variables)]
  fn can_delete(
    registry_id: &RegistryId,
    owner: &AccountId,
    issuer: &AccountId,
    hash: &RegistryHash,
    info: &RegistryInfo,
    salable: &bool,
    country: &Country,
    chunk_hashes: &Vec<ChunkHash>,
  ) -> bool {
    true
  }

  #[allow(unused_variables)]
  fn on_delete(
    registry_id: &RegistryId,
    owner: &AccountId,
    issuer: &AccountId,
    hash: &RegistryHash,
    info: &RegistryInfo,
    salable: &bool,
    country: &Country,
    chunk_hashes: &Vec<ChunkHash>,
  ) -> DispatchResultWithPostInfo {
    Ok(().into())
  }
}
