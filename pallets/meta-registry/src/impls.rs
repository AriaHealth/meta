use super::pallet::*;

use frame_support::ensure;
use frame_support::pallet_prelude::*;
use frame_system::pallet_prelude::*;

use region::{region_of, sub_region_of};

use crate::types::{Accessibility, ChunkHash};

impl<T: Config> Pallet<T> {
    pub fn create_registry(
        registry_id: &RegistryId,
        owner: &AccountId,
        issuer: &AccountId,
        hash: &RegistryHash,
        info: &RegistryInfo,
        salable: &bool,
        country: &Country,
        chunk_hashes: &Vec<ChunkHash>,
    ) -> Result<(), Error<T>> {
        ensure!(
            !Registries::<T>::contains_key(registry_id),
            Error::<T>::RegistryAlreadyExisted
        );
        for chunk_hash in chunk_hashes.clone().iter() {
            let maybe_chunk = Chunks::get(chunk_hash);
            ensure!(maybe_chunk.is_none(), Error::<T>::ChunkAlreadyExisted);
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

        Registries::<T>::insert(
            registry_id,
            Registry {
                owner: owner.clone(),
                issuer: issuer.clone(),
                hash: hash.clone(),
                info: info.clone(),
                salable: false,
                country: country.clone(),
                region: region_of(country.clone()),
                sub_region: sub_region_of(country.clone()),
                accessors: accessors.clone(),
                status: Accessibility::New,
                accessors: 2,
                chunk_hashes: chunk_hashes.clone(),
            },
        );

        Ok(())
    }

    pub fn delete_registry(registry_id: &RegistryId, owner: &AccountId, issuer: &AccountId) {
        let maybe_registry = Registries::<T>::get(registry_id);

        ensure!(maybe_registry.is_some(), Error::<T>::Registry);

        // TODO
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
