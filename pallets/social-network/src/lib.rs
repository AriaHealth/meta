#![cfg_attr(not(feature = "std"), no_std)]
pub use pallet::*;

pub mod constants;
mod impls;
pub mod traits;
pub mod types;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
  use crate::traits::ConnectionRules;
  use crate::types::AccountDetail;
  use crate::types::Group;
  use crate::types::GroupId;
  use crate::types::GroupInfo;
  use crate::types::Permission;
  use crate::types::Relation;
  use ap_region::Country;
  use ap_region::Region;
  use ap_region::SubRegion;
  use frame_support::pallet_prelude::*;
  use frame_system::pallet_prelude::*;
  use sp_std::vec::Vec;

  #[pallet::pallet]
  #[pallet::without_storage_info]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  /// Configure the pallet by specifying the parameters and types on which it depends.
  #[pallet::config]
  pub trait Config: frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    type ConnectionRules: ConnectionRules<Self::AccountId>;
  }

  #[pallet::genesis_config]
  pub struct GenesisConfig<T: Config> {
    pub main_custodian_id: Option<T::AccountId>,
  }

  #[cfg(feature = "std")]
  impl<T: Config> Default for GenesisConfig<T> {
    fn default() -> Self {
      Self { main_custodian_id: None }
    }
  }

  #[pallet::genesis_build]
  impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
    fn build(&self) {
      if let Some(main_custodian_id) = self.main_custodian_id.clone() {
        Custodians::<T>::put(vec![main_custodian_id.clone()]);
        Accounts::<T>::insert(
          main_custodian_id.clone(),
          AccountDetail::<T::AccountId> {
            issuer: main_custodian_id,
            freezer: Default::default(),
            status: Default::default(),
            info: Default::default(),
          },
        );
      }
    }
  }

  #[pallet::storage]
  #[pallet::getter(fn custodians)]
  pub type Custodians<T: Config> = StorageValue<_, Vec<T::AccountId>, ValueQuery>;

  #[pallet::storage]
  #[pallet::getter(fn accounts)]
  pub type Accounts<T: Config> = StorageMap<_, Blake2_128Concat, T::AccountId, AccountDetail<T::AccountId>>;

  #[pallet::storage]
  #[pallet::getter(fn connections)]
  pub type Connections<T: Config> = StorageDoubleMap<_, Twox64Concat, T::AccountId, Twox64Concat, T::AccountId, Relation>;

  #[pallet::storage]
  #[pallet::getter(fn groups)]
  pub type Groups<T: Config> = StorageMap<_, Twox64Concat, GroupId, Group<T::AccountId>>;

  #[pallet::storage]
  #[pallet::getter(fn group_members)]
  pub type GroupPermissions<T: Config> = StorageDoubleMap<_, Twox64Concat, GroupId, Twox64Concat, T::AccountId, Permission>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    GroupCreated {
      group_id: GroupId,
      admin_id: T::AccountId,
    },

    JoinedGroup {
      joiner_id: T::AccountId,
      group_id: GroupId,
    },

    DisjoinedGroup {
      disjoiner_id: T::AccountId,
      group_id: GroupId,
    },

    CustodianAdded {
      custodian_id: T::AccountId,
      issuer_id: T::AccountId,
    },

    CustodianRemoved {
      custodian_id: T::AccountId,
      issuer_id: T::AccountId,
    },

    PermissionUpdated {
      group_id: GroupId,
      account_id: T::AccountId,
      permission: Permission,
    },
  }

  // Errors inform users that something went wrong.
  #[pallet::error]
  pub enum Error<T> {
    AccountNotExisted,
    AccountNotLive,
    AlreadyJoined,
    GroupAlreadyExisted,
    GroupNotExisted,
    NeverJoined,
    OnlyAdminAllowed,
    NoPermission,
    ValueOverflow,
    TooManyCustodians,
    TooFewCustodians,
    CustodianAlreadyRegistered,
    CustodianNotRegistered,
    Unauthorized,
    SelfOperationNotAllowed,
    OnlySuperAdminAllowed,
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::call_index(0)]
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn create_group(
      origin: OriginFor<T>,
      admin_id: T::AccountId,
      group_id: GroupId,
      group_info: GroupInfo,
      country: Country,
      region: Region,
      sub_region: SubRegion,
    ) -> DispatchResult {
      let custodian_id = ensure_signed(origin)?;

      Self::do_create_group(&custodian_id, &admin_id, &group_id, &group_info, &country, &region, &sub_region)?;

      Self::deposit_event(Event::GroupCreated { group_id, admin_id });

      Ok(())
    }

    #[pallet::call_index(1)]
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn join_group(origin: OriginFor<T>, joiner_id: T::AccountId, group_id: GroupId, permission: Permission) -> DispatchResult {
      let issuer_id = ensure_signed(origin)?;

      Self::do_join_group(&issuer_id, &joiner_id, &group_id, &permission)?;

      Self::deposit_event(Event::JoinedGroup { joiner_id, group_id });

      Ok(())
    }

    #[pallet::call_index(2)]
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn disjoin_group(origin: OriginFor<T>, disjoiner_id: T::AccountId, group_id: GroupId) -> DispatchResult {
      let issuer_id = ensure_signed(origin)?;

      Self::do_disjoin_group(&issuer_id, &disjoiner_id, &group_id)?;

      Self::deposit_event(Event::DisjoinedGroup { disjoiner_id, group_id });

      Ok(())
    }

    #[pallet::call_index(3)]
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn add_custodian(origin: OriginFor<T>, custodian_id: T::AccountId) -> DispatchResult {
      let issuer_id = ensure_signed(origin)?;

      Self::do_add_custodian(&issuer_id, &custodian_id)?;

      Self::deposit_event(Event::CustodianAdded { custodian_id, issuer_id });

      Ok(())
    }

    #[pallet::call_index(4)]
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn remove_custodian(origin: OriginFor<T>, custodian_id: T::AccountId) -> DispatchResult {
      let issuer_id = ensure_signed(origin)?;

      Self::do_remove_custodian(&issuer_id, &custodian_id)?;

      Self::deposit_event(Event::CustodianRemoved { custodian_id, issuer_id });

      Ok(())
    }

    #[pallet::call_index(5)]
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn update_permission(origin: OriginFor<T>, member_id: T::AccountId, group_id: GroupId, permission: Permission) -> DispatchResult {
      let issuer_id = ensure_signed(origin)?;

      Self::do_update_permission(&issuer_id, &member_id, &group_id, &permission)?;

      Self::deposit_event(Event::PermissionUpdated {
        group_id,
        account_id: member_id,
        permission,
      });

      Ok(())
    }
  }
}
