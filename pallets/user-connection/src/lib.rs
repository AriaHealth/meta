#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;
mod constants;
mod impls;
#[cfg(test)]
mod mock;
mod traits;
mod types;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use crate::traits::ConnectionRuler;
    use crate::types::{GroupId, GroupInfo, Relation};
    use frame_support::pallet_prelude::*;
    use frame_support::traits::Get;
    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        /// Because this pallet emits events, it depends on the runtime's definition of an event.
        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        #[pallet::constant]
        type RemovalLimit: Get<u32>;

        type ConnectionRuler: ConnectionRuler<Self>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn connections)]
    pub type Connections<T: Config> = StorageDoubleMap<
        _,
        Blake2_128Concat,
        T::AccountId,
        Blake2_128Concat,
        T::AccountId,
        Relation,
    >;

    #[pallet::storage]
    #[pallet::getter(fn groups)]
    pub type Groups<T: Config> = StorageMap<_, Blake2_128Concat, GroupId, Option<GroupInfo>>;

    #[pallet::storage]
    #[pallet::getter(fn group_admins)]
    pub type GroupAdmins<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, GroupId, Blake2_128Concat, T::AccountId, bool>;

    #[pallet::storage]
    #[pallet::getter(fn group_members)]
    pub type GroupMembers<T: Config> =
        StorageDoubleMap<_, Blake2_128Concat, GroupId, Blake2_128Concat, T::AccountId, bool>;

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
