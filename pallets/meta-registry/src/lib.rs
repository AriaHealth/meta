#![cfg_attr(not(feature = "std"), no_std)]

mod constants;
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
    use frame_support::pallet_prelude::*;
    use scale_info::TypeInfo;

    use crate::types::{AccessType, RegistryHash, RegistryId, RegistryInfo};
    use common::types::{Country, Region, SubRegion};

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;
    }

    #[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
    #[scale_info(skip_type_params(T))]
    #[codec(mel_bound())]
    pub struct Registry {
        pub hash: RegistryHash,
        pub info: RegistryInfo,
        pub salable: bool,
        pub country: Country,
        pub region: Region,
        pub sub_region: SubRegion,
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn registries)]
    pub type Registries<T: Config> = StorageMap<_, Blake2_128Concat, RegistryId, Registry>;
    #[pallet::storage]
    #[pallet::getter(fn accesses)]
    pub type Accesses<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        RegistryId,
        Blake2_128Concat,
        T::AccountId,
        AccessType,
    >;

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
        /// Error names should be descriptive.
        NoneValue,
        /// Errors should have helpful documentation associated with them.
        StorageOverflow,
    }
}
