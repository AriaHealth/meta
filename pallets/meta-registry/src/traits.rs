use crate::types::{ChunkHash, RegistryHash, RegistryId, RegistryInfo};
use crate::Config;
use frame_support::dispatch::DispatchResultWithPostInfo;
use region::Country;
use sp_std::vec::Vec;

pub trait IssuerRules<T: Config> {
  fn can_create(
    &self,
    registry_id: &RegistryId,
    owner: &T::AccountId,
    issuer: &T::AccountId,
    hash: &RegistryHash,
    info: &RegistryInfo,
    salable: &bool,
    country: &Country,
    chunk_hashes: &Vec<ChunkHash>,
  ) -> bool;

  fn on_create(
    &self,
    registry_id: &RegistryId,
    owner: &T::AccountId,
    issuer: &T::AccountId,
    hash: &RegistryHash,
    info: &RegistryInfo,
    salable: &bool,
    country: &Country,
    chunk_hashes: &Vec<ChunkHash>,
  ) -> DispatchResultWithPostInfo;

  fn can_delete(
    &self,
    registry_id: &RegistryId,
    owner: &T::AccountId,
    issuer: &T::AccountId,
    hash: &RegistryHash,
    info: &RegistryInfo,
    salable: &bool,
    country: &Country,
    chunk_hashes: &Vec<ChunkHash>,
  ) -> bool;

  fn on_delete(
    &self,
    registry_id: &RegistryId,
    owner: &T::AccountId,
    issuer: &T::AccountId,
    hash: &RegistryHash,
    info: &RegistryInfo,
    salable: &bool,
    country: &Country,
    chunk_hashes: &Vec<ChunkHash>,
  ) -> DispatchResultWithPostInfo;
}

impl<T: Config> IssuerRules<T> for () {
  #[allow(unused_variables)]
  fn can_create(
    &self,
    registry_id: &RegistryId,
    owner: &T::AccountId,
    issuer: &T::AccountId,
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
    &self,
    registry_id: &RegistryId,
    owner: &T::AccountId,
    issuer: &T::AccountId,
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
    &self,
    registry_id: &RegistryId,
    owner: &T::AccountId,
    issuer: &T::AccountId,
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
    &self,
    registry_id: &RegistryId,
    owner: &T::AccountId,
    issuer: &T::AccountId,
    hash: &RegistryHash,
    info: &RegistryInfo,
    salable: &bool,
    country: &Country,
    chunk_hashes: &Vec<ChunkHash>,
  ) -> DispatchResultWithPostInfo {
    Ok(().into())
  }
}
