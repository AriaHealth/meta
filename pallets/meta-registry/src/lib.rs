#![cfg_attr(not(feature = "std"), no_std)]

pub mod crypto;

pub mod constants;
mod impls;
pub mod traits;
pub mod types;

pub use pallet::*;

#[cfg(test)]
mod mock;

#[cfg(test)]
mod tests;

#[cfg(feature = "runtime-benchmarks")]
mod benchmarking;

#[frame_support::pallet]
pub mod pallet {
  use crate::constants::OFFCHAIN_WORKER_TASKS;
  use crate::traits::CustodianRules;
  use crate::traits::IssuerRules;
  use crate::types::AccessType;
  use crate::types::Accessibility;
  use crate::types::Chunk;
  use crate::types::ChunkHash;
  use crate::types::ChunkId;
  use crate::types::DeliveryNetwork;
  use crate::types::DeliveryNetworkURI;
  use crate::types::Registry;
  use crate::types::RegistryHash;
  use crate::types::RegistryId;
  use crate::types::RegistryInfo;
  use ap_region::Country;
  use ap_region::Region;
  use ap_region::SubRegion;
  use frame_support::pallet_prelude::*;
  use frame_system::offchain::AppCrypto;
  use frame_system::offchain::CreateSignedTransaction;
  use frame_system::offchain::SubmitTransaction;
  use frame_system::pallet_prelude::*;
  use sp_runtime::offchain::storage_lock::BlockAndTime;
  use sp_runtime::offchain::storage_lock::StorageLock;
  use sp_runtime::offchain::Duration;
  use sp_runtime::transaction_validity::InvalidTransaction;
  use sp_runtime::transaction_validity::TransactionValidity;
  use sp_runtime::transaction_validity::ValidTransaction;
  use sp_std::vec::Vec;

  #[pallet::pallet]
  #[pallet::without_storage_info]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  /// Configure the pallet by specifying the parameters and types on which it depends.
  #[pallet::config]
  pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    type IssuerRules: IssuerRules<Self::AccountId>;
    type CustodianRules: CustodianRules<Self::AccountId>;
  }

  #[pallet::genesis_config]
  pub struct GenesisConfig<T: Config> {
    pub delivery_network_uri: Option<DeliveryNetworkURI>,
    pub delivery_network_id: Option<T::AccountId>,
  }

  #[cfg(feature = "std")]
  impl<T: Config> Default for GenesisConfig<T> {
    fn default() -> Self {
      Self {
        delivery_network_uri: None,
        delivery_network_id: None,
      }
    }
  }

  #[pallet::genesis_build]
  impl<T: Config> GenesisBuild<T> for GenesisConfig<T> {
    fn build(&self) {
      if self.delivery_network_uri.is_some() && self.delivery_network_id.is_some() {
        let delivery_network_uri = self.delivery_network_uri.clone().unwrap();
        let delivery_network_id = self.delivery_network_id.clone().unwrap();

        DeliveryNetworks::<T>::insert(
          delivery_network_id,
          DeliveryNetwork {
            uri: delivery_network_uri,
            country: None,
            region: Some(Region::Europe),
            sub_region: None,
          },
        );
        CurrentChunkBlockNumber::<T>::put(T::BlockNumber::from(0u32));
      }
    }
  }

  #[pallet::storage]
  #[pallet::getter(fn delivery_networks)]
  pub type DeliveryNetworks<T: Config> = StorageMap<_, Twox64Concat, T::AccountId, DeliveryNetwork>;

  #[pallet::storage]
  #[pallet::getter(fn registries)]
  pub type Registries<T: Config> = StorageMap<_, Blake2_128Concat, RegistryId, Registry<T::AccountId>>;

  #[pallet::storage]
  #[pallet::getter(fn chunks)]
  pub type Chunks<T: Config> = StorageMap<_, Blake2_128Concat, ChunkId, Chunk<T::BlockNumber>>;

  #[pallet::storage]
  #[pallet::getter(fn chunk_block)]
  pub type ChunkBlock<T: Config> = StorageMap<_, Twox64Concat, T::BlockNumber, Vec<ChunkId>>;

  #[pallet::storage]
  #[pallet::getter(fn current_chunk_block)]
  pub type CurrentChunkBlockNumber<T: Config> = StorageValue<_, T::BlockNumber, ValueQuery>;

  #[pallet::storage]
  #[pallet::getter(fn accesses)]
  pub type Accesses<T: Config> = StorageDoubleMap<_, Blake2_128Concat, RegistryId, Blake2_128Concat, T::AccountId, AccessType>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    /// Chunk has been inspected
    ChunkInspected {
      chunk_id: ChunkId,
      status: Accessibility,
      next_block_number: T::BlockNumber,
    },

    /// New delivery network registered
    DeliveryNetworkRegistered {
      delivery_network_id: T::AccountId,
      delivery_network_uri: DeliveryNetworkURI,
    },

    /// New registry created
    RegistryCreated { registry_id: RegistryId, issuer_id: T::AccountId },

    /// A registry updated
    RegistryUpdated { registry_id: RegistryId, author_id: T::AccountId },
  }

  // Errors inform users that something went wrong.
  #[pallet::error]
  pub enum Error<T> {
    ChunkAlreadyExisted,
    ChunkNotExisted,
    DeliveryNetworkAlreadyExisted,
    DeliveryNetworkNotExisted,
    NoneValue,
    NoChanges,
    Overflow,
    RegistryAlreadyExisted,
    RegistryNotExisted,
    RegistrySalable,
    StorageOverflow,
    NonAuthorized,
    NoLocationSpecified,
    ChunkBlockNotExisted,
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn offchain_worker(block_number: T::BlockNumber) {
      // Locking mechanism
      let mut lock = StorageLock::<BlockAndTime<Self>>::with_block_and_time_deadline(b"mtrg::lock", 3, Duration::from_millis(6000));

      if let Ok(_guard) = lock.try_lock() {
        // Unsigned transaction with unsigned payload
        let number: u64 = block_number.try_into().unwrap_or(0);

        log::info!("{:?}::{:?}::starting...", crate::crypto::KEY_NAME, block_number);

        let call: Call<T>;
        match number % OFFCHAIN_WORKER_TASKS {
          1 | _ => {
            call = Call::inspect_chunk {
              block_number,
              maybe_chunk_id: None,
              maybe_status: None,
            };
          },
        }

        SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
          .map_err(|()| "Unable to submit unsigned transaction.")
          .ok();

        log::info!("{:?}::{:?}::finished...", crate::crypto::KEY_NAME, block_number);
      };
    }
  }

  #[pallet::validate_unsigned]
  impl<T: Config> ValidateUnsigned for Pallet<T> {
    type Call = Call<T>;

    fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
      if let Call::inspect_chunk { block_number, .. } = call {
        ValidTransaction::with_tag_prefix("mtrg::inspect_chunk")
          .priority(3)
          .and_provides(block_number)
          .longevity(5)
          .propagate(true)
          .build()
      } else {
        InvalidTransaction::Call.into()
      }
    }
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::call_index(0)]
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn inspect_chunk(
      origin: OriginFor<T>,
      block_number: T::BlockNumber,
      maybe_chunk_id: Option<ChunkId>,
      maybe_status: Option<Accessibility>,
    ) -> DispatchResult {
      ensure_none(origin)?;
      ensure!(maybe_chunk_id.is_some() && maybe_status.is_some(), Error::<T>::NoneValue);

      let chunk_id = maybe_chunk_id.unwrap();
      let status = maybe_status.unwrap();
      ensure!(Self::chunks(chunk_id.clone()).is_some(), Error::<T>::ChunkNotExisted);

      Self::update_chunk(&chunk_id, &block_number, &status)?;
      let next_block_number = Self::update_chunk_block(&chunk_id)?;

      Self::deposit_event(Event::ChunkInspected {
        chunk_id,
        status,
        next_block_number,
      });

      Ok(())
    }

    #[pallet::call_index(1)]
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn create_delivery_network(
      origin: OriginFor<T>,
      delivery_network_id: T::AccountId,
      delivery_network_uri: DeliveryNetworkURI,
      country: Option<Country>,
      region: Option<Region>,
      sub_region: Option<SubRegion>,
    ) -> DispatchResult {
      let author_id = ensure_signed(origin)?;

      ensure!(T::CustodianRules::is_authorized(&author_id), Error::<T>::NonAuthorized);

      Self::do_create_delivery_network(&delivery_network_id, &delivery_network_uri, &country, &region, &sub_region)?;

      Self::deposit_event(Event::DeliveryNetworkRegistered {
        delivery_network_id,
        delivery_network_uri,
      });

      Ok(())
    }

    #[pallet::call_index(2)]
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn create_registry(
      origin: OriginFor<T>,
      registry_id: RegistryId,
      owner_id: T::AccountId,
      issuer_id: T::AccountId,
      hash: RegistryHash,
      info: RegistryInfo,
      country: Country,
      delivery_network_id: T::AccountId,
      chunk_hashes: Vec<ChunkHash>,
    ) -> DispatchResult {
      let author_id = ensure_signed(origin)?;

      ensure!(T::IssuerRules::can_create(&owner_id, &issuer_id, &author_id,), Error::<T>::NonAuthorized);

      let registry = Self::do_create_registry(
        &registry_id,
        &owner_id,
        &issuer_id,
        &author_id,
        &hash,
        &info,
        &false,
        &country,
        &delivery_network_id,
        &chunk_hashes,
      )?;

      T::IssuerRules::on_create(&registry);
      Self::deposit_event(Event::RegistryCreated { registry_id, issuer_id });

      Ok(())
    }

    #[pallet::call_index(3)]
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn set_salable(origin: OriginFor<T>, registry_id: RegistryId, salable: bool) -> DispatchResult {
      let author_id = ensure_signed(origin)?;

      let maybe_registry = Registries::<T>::get(registry_id.clone());
      ensure!(maybe_registry.is_some(), Error::<T>::RegistryNotExisted);

      let old_registry = maybe_registry.unwrap();

      let mut new_registry = old_registry.clone();
      new_registry.salable = salable.clone();

      ensure!(old_registry.salable != new_registry.salable, Error::<T>::NoChanges);
      ensure!(
        T::IssuerRules::can_update(&old_registry, &new_registry, &author_id),
        Error::<T>::NonAuthorized
      );

      Self::do_set_salable(&registry_id, &salable)?;

      T::IssuerRules::on_update(&new_registry, &author_id);
      Self::deposit_event(Event::RegistryUpdated { registry_id, author_id });

      Ok(())
    }
  }
}
