#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

mod constants;
mod impls;
mod traits;
mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
    use crate::traits::ConnectionRuler;
    use crate::types::{AccessControl, AccountDetail, GroupInfo, Relation};

    use frame_support::pallet_prelude::*;
    use frame_support::traits::Get;

    /// Configure the pallet by specifying the parameters and types on which it depends.
    #[pallet::config]
    pub trait Config: frame_system::Config {
        type GroupId: Member + Parameter + Copy + MaybeSerializeDeserialize + MaxEncodedLen;
        type GroupIdParameter: Parameter
            + Copy
            + From<Self::GroupId>
            + Into<Self::GroupId>
            + MaxEncodedLen;

        type Event: From<Event<Self>> + IsType<<Self as frame_system::Config>::Event>;

        #[pallet::constant]
        type RemovalLimit: Get<u32>;

        type ConnectionRuler: ConnectionRuler<Self>;
    }

    #[pallet::pallet]
    #[pallet::generate_store(pub(super) trait Store)]
    pub struct Pallet<T>(_);

    #[pallet::storage]
    #[pallet::getter(fn accounts)]
    pub type Accounts<T: Config> =
        StorageMap<_, Blake2_128Concat, T::AccountId, AccountDetail<T::AccountId>>;

    #[pallet::storage]
    #[pallet::getter(fn connections)]
    pub type Connections<T: Config> =
        StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, T::AccountId, Relation>;

    #[pallet::storage]
    #[pallet::getter(fn groups)]
    pub type Groups<T: Config> = StorageMap<_, Twox64Concat, T::GroupId, Option<GroupInfo>>;

    #[pallet::storage]
    #[pallet::getter(fn group_members)]
    pub type GroupMembers<T: Config> =
        StorageDoubleMap<_, Twox64Concat, T::GroupId, Twox64Concat, T::AccountId, AccessControl>;

    #[pallet::event]
    #[pallet::generate_deposit(pub(super) fn deposit_event)]
    pub enum Event<T: Config> {
        SomethingStored(u32, T::AccountId),
    }

    // Errors inform users that something went wrong.
    #[pallet::error]
    pub enum Error<T> {
        OnlyPendingAllowed,
        AlreadyJoined,
        AlreadyJoining,
        AlreadyConnected,
        AlreadyConnecting,
        AccountNotLive,
        AccountNotExisted,
        NeverConnecting,
        NeverJoining,
        GroupAlreadyExisted,
        GroupNotExisted,
    }
}
