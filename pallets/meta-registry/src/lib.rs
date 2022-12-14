#![cfg_attr(not(feature = "std"), no_std)]

mod constants;
mod traits;
mod types;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
  use crate::traits::IssuerRules;
  use crate::types::{AccessType, Chunk, ChunkHash, DeliveryNetwork, DeliveryNetworkId, Registry, RegistryId};
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;
  use sp_std::vec::Vec;

  /// Configure the pallet by specifying the parameters and types on which it depends.
  #[pallet::config]
  pub trait Config: frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

    type IssuerRules: IssuerRules<Self::AccountId>;
  }

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  #[pallet::storage]
  #[pallet::getter(fn delivery_networks)]
  pub type DeliveryNetworks<T: Config> = StorageMap<_, Twox64Concat, DeliveryNetworkId, DeliveryNetwork>;

  #[pallet::storage]
  #[pallet::getter(fn registries)]
  pub type Registries<T: Config> = StorageMap<_, Blake2_128Concat, RegistryId, Registry<T::AccountId>>;

  #[pallet::storage]
  #[pallet::getter(fn chunks)]
  pub type Chunks<T: Config> = StorageMap<_, Blake2_128Concat, ChunkHash, Chunk<T::BlockNumber>>;

  #[pallet::storage]
  #[pallet::getter(fn chunk_block)]
  pub type ChunkBlock<T: Config> = StorageMap<_, Twox64Concat, T::BlockNumber, Vec<ChunkHash>>;

  #[pallet::storage]
  #[pallet::getter(fn accesses)]
  pub type Accesses<T: Config> = StorageDoubleMap<_, Blake2_128Concat, RegistryId, Blake2_128Concat, T::AccountId, AccessType>;

  // Pallets use events to inform users when important changes are made.
  // https://substrate.dev/docs/en/knowledgebase/runtime/events
  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    /// Event documentation should end with an array that provides descriptive names for event
    /// parameters. [something, who]
    SomethingStored(u32, T::AccountId),
  }

  // Errors inform users that something went wrong.
  #[pallet::error]
  pub enum Error<T> {
    ChunkAlreadyExisted,
    ChunkNotExisted,
    DeliveryNetworkAlreadyExisted,
    DeliveryNetworkNotExisted,
    NoneValue,
    Overflow,
    RegistryAlreadyExisted,
    RegistryNotExisted,
    RegistrySalable,
    StorageOverflow,
    NonAuthorized,
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(10_000)]
    pub fn do_something(origin: OriginFor<T>, id: T::AccountId) -> DispatchResult {
      let creator_id = ensure_signed(origin)?;

      Ok(())
    }
  }
}
