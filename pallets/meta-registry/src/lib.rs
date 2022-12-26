#![cfg_attr(not(feature = "std"), no_std)]

pub mod crypto;

mod constants;
mod impls;
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
  use crate::types::{AccessType, Chunk, ChunkHash, DeliveryNetwork, DeliveryNetworkId, Registry, RegistryId};
  use frame_support::pallet_prelude::*;
  use frame_system::{
    offchain::{AppCrypto, CreateSignedTransaction, SubmitTransaction},
    pallet_prelude::*,
  };
  use sp_runtime::{
    offchain::{
      storage_lock::{BlockAndTime, StorageLock},
      Duration,
    },
    transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
  };
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
  }

  #[pallet::storage]
  #[pallet::getter(fn something)]
  pub type Something<T> = StorageValue<_, u32>;

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
  // https://docs.substrate.io/main-docs/build/events-errors/
  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    /// Event documentation should end with an array that provides descriptive names for event
    /// parameters. [something, who]
    SomethingStored { something: u32 },
    /// Event documentation should end with an array that provides descriptive names for event
    /// parameters. [something, who]
    SomethingStoredSigned { something: u32, account_id: T::AccountId },
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

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn offchain_worker(block_number: T::BlockNumber) {
      // Locking mechanism
      let mut lock = StorageLock::<BlockAndTime<Self>>::with_block_and_time_deadline(b"offchain-demo::lock", 3, Duration::from_millis(6000));

      if let Ok(_guard) = lock.try_lock() {
        // Unsigned transaction with unsigned payload
        let number: u64 = block_number.try_into().unwrap_or(0);

        log::info!("Hello from pallet-meta-registry.");

        let call = Call::do_something {
          block_number,
          something: number.try_into().unwrap_or(0),
        };

        SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
          .map_err(|()| "Unable to submit unsigned transaction.")
          .ok();
      };
    }
  }

  #[pallet::validate_unsigned]
  impl<T: Config> ValidateUnsigned for Pallet<T> {
    type Call = Call<T>;

    fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
      if let Call::do_something { block_number, something: _ } = call {
        ValidTransaction::with_tag_prefix("ExampleOffchainWorker")
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

  // Dispatchable functions allows users to interact with the pallet and invoke state changes.
  // These functions materialize as "extrinsics", which are often compared to transactions.
  // Dispatchable functions must be annotated with a weight and must return a DispatchResult.
  #[pallet::call]
  impl<T: Config> Pallet<T> {
    /// An example dispatchable that takes a singles value as a parameter, writes the value to
    /// storage and emits an event. This function must be dispatched by a signed extrinsic.
    #[pallet::call_index(0)]
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn do_something(origin: OriginFor<T>, _block_number: T::BlockNumber, something: u32) -> DispatchResult {
      // Check that the extrinsic was signed and get the signer.
      // This function will return an error if the extrinsic is not signed.
      // https://docs.substrate.io/main-docs/build/origins/
      ensure_none(origin)?;

      // Update storage.
      <Something<T>>::put(something);

      // Emit an event.
      Self::deposit_event(Event::SomethingStored { something });
      // Return a successful DispatchResultWithPostInfo
      Ok(())
    }
  }
}
