use super::pallet::*;
use crate::constants::CHUNK_BLOCK_HEIGHT;
use crate::constants::CLEAR_PREFIX_BATCH_SIZE;
use crate::traits::CombinedIdentifier;
use crate::types::AccessType;
use crate::types::Accessibility;
use crate::types::Chunk;
use crate::types::ChunkHash;
use crate::types::ChunkId;
use crate::types::DeliveryNetwork;
use crate::types::DeliveryNetworkURI;
use crate::types::Payload;
use crate::types::Registry;
use crate::types::RegistryHash;
use crate::types::RegistryId;
use crate::types::RegistryInfo;
use ap_region::Country;
use ap_region::CountryTrait;
use ap_region::Region;
use ap_region::SubRegion;
use frame_support::ensure;
use frame_system::offchain::SignedPayload;
use frame_system::offchain::SigningTypes;
use sp_runtime::traits::BlockNumberProvider;
use sp_std::borrow::ToOwned;
use sp_std::vec;

impl<T: Config> Pallet<T> {
  pub fn do_create_delivery_network(
    delivery_network_id: &T::AccountId,
    uri: &DeliveryNetworkURI,
    country: &Option<Country>,
    region: &Option<Region>,
    sub_region: &Option<SubRegion>,
  ) -> Result<(), Error<T>> {
    ensure!(
      !DeliveryNetworks::<T>::contains_key(delivery_network_id),
      Error::<T>::DeliveryNetworkAlreadyExisted
    );

    ensure!(
      country.is_some() || region.is_some() || sub_region.is_some(),
      Error::<T>::NoLocationSpecified
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

  pub fn do_create_registry(
    registry_id: &RegistryId,
    owner_id: &T::AccountId,
    issuer_id: &T::AccountId,
    author_id: &T::AccountId,
    hash: &RegistryHash,
    info: &RegistryInfo,
    salable: &bool,
    country: &Country,
    delivery_network_id: &T::AccountId,
    chunk_hashes: &[ChunkHash],
  ) -> Result<Registry<T::AccountId>, Error<T>> {
    ensure!(
      DeliveryNetworks::<T>::contains_key(delivery_network_id),
      Error::<T>::DeliveryNetworkNotExisted
    );
    ensure!(!Registries::<T>::contains_key(registry_id), Error::<T>::RegistryAlreadyExisted);

    for chunk_hash in chunk_hashes.to_owned().iter() {
      let chunk_id = ChunkId::compose(&registry_id.clone(), chunk_hash);
      ensure!(!Chunks::<T>::contains_key(chunk_id), Error::<T>::ChunkAlreadyExisted);
      Self::update_chunk_block(&chunk_id)?;
    }

    let now = <frame_system::Pallet<T>>::block_number();

    for chunk_hash in chunk_hashes.to_owned().iter() {
      Chunks::<T>::insert(
        ChunkId::compose(&registry_id.clone(), chunk_hash),
        Chunk {
          last_block: now,
          status: Accessibility::New,
        },
      )
    }

    Accesses::<T>::insert(*registry_id, issuer_id.clone(), AccessType::Issuer);
    Accesses::<T>::insert(*registry_id, owner_id.clone(), AccessType::Owner);

    let registry = Registry {
      delivery_network_id: delivery_network_id.clone(),
      owner_id: owner_id.clone(),
      issuer_id: issuer_id.clone(),
      author_id: author_id.clone(),
      hash: *hash,
      info: info.clone(),
      status: Accessibility::New,
      salable: *salable,
      country: country.clone(),
      region: Region::of_country(country.clone()),
      sub_region: SubRegion::of_country(country.clone()),
      accessor_count: 2,
      chunk_hashes: chunk_hashes.to_owned(),
    };
    Registries::<T>::insert(registry_id, registry.clone());

    Ok(registry)
  }

  pub fn do_set_salable(registry_id: &RegistryId, salable: &bool) -> Result<(), Error<T>> {
    Registries::<T>::try_mutate(registry_id, |maybe_registry| {
      let mut registry = maybe_registry.take().ok_or(Error::<T>::RegistryNotExisted)?;

      registry.salable = *salable;

      *maybe_registry = Some(registry);

      Ok(())
    })?;

    Ok(())
  }

  pub fn do_delete_registry(registry_id: &RegistryId, author_id: &T::AccountId) -> Result<(), Error<T>> {
    let maybe_registry = Registries::<T>::get(registry_id);
    ensure!(maybe_registry.is_some(), Error::<T>::RegistryNotExisted);

    let registry = maybe_registry.unwrap();
    ensure!(
      registry.owner_id == author_id.clone() || registry.issuer_id == author_id.clone(),
      Error::<T>::NonAuthorized
    );
    ensure!(!registry.salable, Error::<T>::RegistrySalable);

    Registries::<T>::try_mutate(registry_id, |maybe_registry| {
      let mut registry = maybe_registry.take().ok_or(Error::<T>::RegistryNotExisted)?;

      registry.status = Accessibility::Deleted;

      *maybe_registry = Some(registry);

      Ok(())
    })?;

    let mut pending_deleted_registries = PendingDeletionRegistries::<T>::get();
    pending_deleted_registries.push(*registry_id);
    PendingDeletionRegistries::<T>::put(pending_deleted_registries);

    // TODO: use offchain worker to remove accesses and chunks

    Ok(())
  }

  pub fn update_chunk(chunk_id: &ChunkId, new_block: &T::BlockNumber, accessibility: &Accessibility) -> Result<(), Error<T>> {
    Chunks::<T>::try_mutate(chunk_id, |maybe_chunk| {
      let mut chunk = maybe_chunk.take().ok_or(Error::<T>::ChunkNotExisted)?;

      chunk.last_block = *new_block;
      chunk.status = accessibility.clone();

      *maybe_chunk = Some(chunk);

      Ok(())
    })

    // TODO: if all chunks are healthy change registry status from new to healthy
  }

  pub fn update_chunk_block(chunk_id: &ChunkId) -> Result<T::BlockNumber, Error<T>> {
    let current_block_number = <frame_system::Pallet<T>>::block_number();
    let current_chunk_block_number = CurrentChunkBlockNumber::<T>::get();
    let next_chunk_block_number = Self::get_nearest_chunk_block(&current_block_number);
    let next_block_number = Self::get_nearest_chunk_block(&current_chunk_block_number);

    let maybe_next_chunk_block = ChunkBlock::<T>::get(next_chunk_block_number);
    let maybe_current_chunk_block = ChunkBlock::<T>::get(current_chunk_block_number);

    ensure!(maybe_current_chunk_block.is_some(), Error::<T>::ChunkBlockNotExisted);

    let current_chunk_block = maybe_current_chunk_block.unwrap();
    if current_chunk_block.len() == 1 {
      ChunkBlock::<T>::remove(current_chunk_block_number);
      CurrentChunkBlockNumber::<T>::put(next_block_number);
    } else {
      ChunkBlock::<T>::try_mutate(current_chunk_block_number, |maybe_current_chunk_block| {
        let mut current_chunk_block = maybe_current_chunk_block.take().ok_or(Error::<T>::ChunkBlockNotExisted)?;

        let index = current_chunk_block.iter().position(|x| x == chunk_id);
        if let Some(index) = index {
          current_chunk_block.remove(index);
        }

        *maybe_current_chunk_block = Some(current_chunk_block);

        Ok(())
      })?;
    }

    if maybe_next_chunk_block.is_none() {
      let new_chunk_block = vec![*chunk_id];
      ChunkBlock::<T>::insert(next_chunk_block_number, new_chunk_block);
    } else {
      ChunkBlock::<T>::mutate(next_chunk_block_number, |maybe_next_chunk_block| {
        let mut next_chunk_block = maybe_next_chunk_block.take().ok_or(Error::<T>::ChunkBlockNotExisted)?;

        next_chunk_block.push(*chunk_id);

        *maybe_next_chunk_block = Some(next_chunk_block);

        Ok(())
      })?;
    }

    Ok(next_chunk_block_number)
  }

  pub fn get_nearest_chunk_block(block_number: &T::BlockNumber) -> T::BlockNumber {
    let curent_block_number: u32 = (*block_number).try_into().unwrap_or(0);
    let mut offset = 0u32;

    if CHUNK_BLOCK_HEIGHT != 0 {
      let remainder = curent_block_number % CHUNK_BLOCK_HEIGHT;

      if remainder != 0 {
        offset = CHUNK_BLOCK_HEIGHT - remainder;
      }
    }

    T::BlockNumber::from(curent_block_number + offset)
  }

  pub fn background_delete_registry(registry_id: &RegistryId, registry: &Registry<T::AccountId>) {
    for chunk_hash in registry.chunk_hashes.iter() {
      let chunk_id = ChunkId::compose(registry_id, chunk_hash);

      Chunks::<T>::remove(chunk_id);
    }

    let mut removal_result = Accesses::<T>::clear_prefix(registry_id, CLEAR_PREFIX_BATCH_SIZE, None);
    while let Some(cursor) = removal_result.maybe_cursor {
      removal_result = Accesses::<T>::clear_prefix(registry_id, CLEAR_PREFIX_BATCH_SIZE, Some(cursor.as_slice()));
    }

    Registries::<T>::remove(registry_id);
    PendingDeletionRegistries::<T>::mutate(|pending_deleted_registries| {
      pending_deleted_registries.retain(|x| x != registry_id);
    });
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

impl CombinedIdentifier<RegistryId, ChunkHash, ChunkId> for ChunkId {
  fn compose(k1: &RegistryId, k2: &ChunkHash) -> ChunkId {
    let mut arr: ChunkId = [0u8; 44];

    let mut i = 0usize;
    for k in k1.iter() {
      arr[i] = *k;
      i = i.saturating_add(1);
    }

    for k in k2.iter() {
      arr[i] = *k;
      i = i.saturating_add(1);
    }

    arr
  }

  fn decompose(&self) -> (RegistryId, ChunkHash) {
    let mut registry_id: RegistryId = [0u8; 12];
    let mut chunk_hash: ChunkHash = [0u8; 32];

    let mut i = 0usize;
    for k in self.iter() {
      if i < 12 {
        registry_id[i] = *k;
      } else {
        chunk_hash[i] = *k;
      }
      i = i.saturating_add(1);
    }

    (registry_id, chunk_hash)
  }
}
