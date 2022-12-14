use super::pallet::*;

use frame_support::ensure;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

use region::{region_of, sub_region_of};

use crate::types::{Accessibility, ChunkHash};

impl<T: Config> Pallet<T> {
    pub fn create_delivery_network(
        delivery_network_id: &DeliveryNetworkId,
        uri: &DeliveryNetworkURI,
        country: &Option<Country>,
        region: &Option<Region>,
        sub_region: &Option<SubRegion>,
    ) -> Result<(), Error<T>> {
        ensure!(
            !DeliveryNetworks::contains_key(delivery_network_id),
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
        owner_id: &AccountId,
        issuer_id: &AccountId,
        hash: &RegistryHash,
        info: &RegistryInfo,
        salable: &bool,
        country: &Country,
        delivery_network_id: &DeliveryNetworkId,
        chunk_hashes: &Vec<ChunkHash>,
    ) -> Result<(), Error<T>> {
        ensure!(
            DeliveryNetworks::contains_key(delivery_network_id),
            Error::<T>::DeliveryNetworkNotExisted
        );
        ensure!(
            !Registries::<T>::contains_key(registry_id),
            Error::<T>::RegistryAlreadyExisted
        );
        for chunk_hash in chunk_hashes.clone().iter() {
            ensure!(
                !maybe_chunk.contains_key(chunk_hash),
                Error::<T>::ChunkAlreadyExisted
            );
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
                hash: hash.clone(),
                info: info.clone(),
                salable: false,
                country: country.clone(),
                region: region_of(country.clone()),
                sub_region: sub_region_of(country.clone()),
                accessors: 2,
                chunk_hashes: chunk_hashes.clone(),
            },
        );

        Ok(())
    }

    pub fn delete_registry(registry_id: &RegistryId, actor_id: &AccountId) {
        let maybe_registry = Registries::<T>::get(registry_id);
        ensure!(maybe_registry.is_some(), Error::<T>::RegistryNotExisted);

        let registry = maybe_registry.unwrap();
        ensure!(
            registry.owner_id == actor_id || registry.issuer_id == actor_id,
            Error::<T>::NonAuthorized
        );
        ensure!(!registry.salable, Error::<T>::RegistrySalable);

        for chunk_hash in registry.chunk_hashes.iter() {
            Chunks::<T>::remove(chunk_hash);
        }

        Accesses::<T>::clear_prefix(registry_id);
        Registries::<T>::remove(registry_id);

        Ok(())
    }

    pub fn update_chunk(
        chunk_hash: &ChunkHash,
        new_block: &BlockNumber,
        accessibility: &Accessibility,
    ) -> Result<(), Error<T>> {
        Chunks::<T>::try_mutate(chunk_hash, |maybe_chunk| {
            let mut chunk = maybe_chunk.take().ok_or(Error::<T>::ChunkNotExisted)?;

            chunk.last_block = new_block;
            chunk.accessibility = accessibility;

            *maybe_chunk = Some(chunk);

            Ok(())
        })
    }
}
