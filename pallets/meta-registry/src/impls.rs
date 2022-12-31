use super::pallet::*;

use ap_region::{Country, CountryTrait, Region, SubRegion};
use frame_support::ensure;
use frame_system::offchain::{SignedPayload, SigningTypes};
use sp_runtime::traits::BlockNumberProvider;
use sp_std::vec::Vec;

use crate::types::{
  AccessType, Accessibility, Chunk, ChunkHash, DeliveryNetwork, DeliveryNetworkURI, Payload, Registry, RegistryHash, RegistryId, RegistryInfo,
};

impl<T: Config> Pallet<T> {
  pub fn create_delivery_network(
    delivery_network_id: &T::AccountId,
    uri: &DeliveryNetworkURI,
    country: &Country,
    region: &Region,
    sub_region: &SubRegion,
  ) -> Result<(), Error<T>> {
    ensure!(
      !DeliveryNetworks::<T>::contains_key(delivery_network_id),
      Error::<T>::DeliveryNetworkAlreadyExisted
    );

    DeliveryNetworks::<T>::insert(
      delivery_network_id,
      DeliveryNetwork {
        uri: uri.clone(),
        country: country.clone(),
        region: region.clone(),
        sub_region: sub_region.clone(),
      },
    );

    Ok(())
  }

  pub fn create_registry(
    registry_id: &RegistryId,
    owner_id: &T::AccountId,
    issuer_id: &T::AccountId,
    author_id: &T::AccountId,
    hash: &RegistryHash,
    info: &RegistryInfo,
    salable: &bool,
    country: &Country,
    delivery_network_id: &T::AccountId,
    chunk_hashes: &Vec<ChunkHash>,
  ) -> Result<(), Error<T>> {
    ensure!(
      DeliveryNetworks::<T>::contains_key(delivery_network_id),
      Error::<T>::DeliveryNetworkNotExisted
    );
    ensure!(!Registries::<T>::contains_key(registry_id), Error::<T>::RegistryAlreadyExisted);
    for chunk_hash in chunk_hashes.clone().iter() {
      ensure!(!Chunks::<T>::contains_key(chunk_hash), Error::<T>::ChunkAlreadyExisted);
    }

    let now = <frame_system::Pallet<T>>::block_number();

    for chunk_hash in chunk_hashes.clone().iter() {
      Chunks::<T>::insert(
        chunk_hash,
        Chunk {
          registry_id: registry_id.clone(),
          last_block: now,
          status: Accessibility::New,
        },
      )
    }

    Accesses::<T>::insert(registry_id.clone(), issuer_id.clone(), AccessType::Issuer);
    Accesses::<T>::insert(registry_id.clone(), owner_id.clone(), AccessType::Owner);

    Registries::<T>::insert(
      registry_id,
      Registry {
        delivery_network_id: delivery_network_id.clone(),
        owner_id: owner_id.clone(),
        issuer_id: issuer_id.clone(),
        author_id: author_id.clone(),
        hash: hash.clone(),
        info: info.clone(),
        status: Accessibility::New,
        salable: salable.clone(),
        country: country.clone(),
        region: Region::of_country(country.clone()),
        sub_region: SubRegion::of_country(country.clone()),
        accessors: 2,
        chunk_hashes: chunk_hashes.clone(),
      },
    );

    Ok(())
  }

  pub fn delete_registry(registry_id: &RegistryId, actor_id: &T::AccountId) -> Result<(), Error<T>> {
    let maybe_registry = Registries::<T>::get(registry_id);
    ensure!(maybe_registry.is_some(), Error::<T>::RegistryNotExisted);

    let registry = maybe_registry.unwrap();
    ensure!(
      registry.owner_id == actor_id.clone() || registry.issuer_id == actor_id.clone(),
      Error::<T>::NonAuthorized
    );
    ensure!(!registry.salable, Error::<T>::RegistrySalable);

    Registries::<T>::try_mutate(registry_id, |maybe_registry| {
      let mut registry = maybe_registry.take().ok_or(Error::<T>::RegistryNotExisted)?;

      registry.status = Accessibility::Deleted;

      *maybe_registry = Some(registry);

      Ok(())
    })?;
    // TODO: use offchain worker to remove accesses and chunks

    Ok(())
  }

  pub fn update_chunk(chunk_hash: &ChunkHash, new_block: &T::BlockNumber, accessibility: &Accessibility) -> Result<(), Error<T>> {
    Chunks::<T>::try_mutate(chunk_hash, |maybe_chunk| {
      let mut chunk = maybe_chunk.take().ok_or(Error::<T>::ChunkNotExisted)?;

      chunk.last_block = new_block.clone();
      chunk.status = accessibility.clone();

      *maybe_chunk = Some(chunk);

      Ok(())
    })
  }
}

impl<T: Config> BlockNumberProvider for Pallet<T> {
  type BlockNumber = T::BlockNumber;

  fn current_block_number() -> Self::BlockNumber {
    <frame_system::Pallet<T>>::block_number()
  }
}

impl<T: SigningTypes> SignedPayload<T> for Payload<T::Public, T::BlockNumber, T::AccountId> {
  fn public(&self) -> T::Public {
    self.public.clone()
  }
}
