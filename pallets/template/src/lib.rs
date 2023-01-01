// Only for template pallet. Delete the warning surpressors if you are not using template pallet.
#![allow(unused_variables)]
#![allow(unused_imports)]
#![cfg_attr(not(feature = "std"), no_std)]

/// Edit this file to define custom logic or remove it if it is not needed.
/// Learn more about FRAME and the core library of Substrate FRAME pallets:
/// <https://docs.substrate.io/reference/frame-pallets/>
pub mod crypto;
mod impls;
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
  use crate::types::Payload;
  use frame_support::{pallet_prelude::*, traits::Randomness};
  use frame_system::{
    offchain::{
      AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction, SignedPayload, Signer, SigningTypes, SubmitTransaction,
    },
    pallet_prelude::*,
  };
  use sp_runtime::{
    offchain::{
      storage::StorageValueRef,
      storage_lock::{BlockAndTime, StorageLock},
      Duration,
    },
    traits::BlockNumberProvider,
    transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
  };
  use sp_std::vec::Vec;

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  pub struct Pallet<T>(_);

  /// Configure the pallet by specifying the parameters and types on which it depends.
  #[pallet::config]
  pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
    /// Because this pallet emits events, it depends on the runtime's definition of an event.
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
    type MyRandomness: Randomness<Self::Hash, Self::BlockNumber>;
  }

  // The pallet's runtime storage items.
  // https://docs.substrate.io/main-docs/build/runtime-storage/
  #[pallet::storage]
  #[pallet::getter(fn something)]
  // Learn more about declaring storage items:
  // https://docs.substrate.io/main-docs/build/runtime-storage/#declaring-storage-items
  pub type Something<T> = StorageValue<_, u32>;

  #[pallet::storage]
  #[pallet::getter(fn nonce)]
  pub type Nonce<T> = StorageValue<_, u32>;

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
    /// Error names should be descriptive.
    NoneValue,
    /// Errors should have helpful documentation associated with them.
    StorageOverflow,
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn offchain_worker(block_number: T::BlockNumber) {
      // Locking mechanism
      let mut lock = StorageLock::<BlockAndTime<Self>>::with_block_and_time_deadline(b"offchain-demo::lock", 3, Duration::from_millis(6000));

      if let Ok(_guard) = lock.try_lock() {
        // Unsigned transaction with unsigned payload
        // let number: u64 = block_number.try_into().unwrap_or(0);

        // log::info!("Hello from pallet-ocw.");

        // let call =
        // 	Call::do_something { block_number, something: number.try_into().unwrap_or(0) };

        // SubmitTransaction::<T, Call<T>>::submit_unsigned_transaction(call.into())
        // 	.map_err(|()| "Unable to submit unsigned transaction.")
        // 	.ok();

        // Unsigned transaction with signed payload
        // let number: u32 = block_number.try_into().unwrap_or(0);

        // Signer::<T, T::AuthorityId>::any_account().send_unsigned_transaction(
        //   |account| Payload {
        //     number,
        //     block_number,
        //     public: account.public.clone(),
        //     account_id: account.id.clone(),
        //   },
        //   |payload, signature| Call::do_something_with_signature { signature, payload },
        // );
      };
    }
  }

  #[pallet::validate_unsigned]
  impl<T: Config> ValidateUnsigned for Pallet<T> {
    type Call = Call<T>;

    fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
      let nonce = Self::get_and_increment_nonce();
      let (random_value, _) = T::MyRandomness::random(&nonce);

      if let Call::do_something_with_signature { signature, payload } = call {
        let signature_valid = SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone());

        if !signature_valid {
          InvalidTransaction::Call.into()
        } else {
          ValidTransaction::with_tag_prefix("ExampleOffchainWorker")
            .priority(random_value.encode()[0].into())
            .and_provides(&payload.block_number)
            .longevity(1)
            .propagate(true)
            .build()
        }
      } else if let Call::do_something { block_number, something } = call {
        ValidTransaction::with_tag_prefix("ExampleOffchainWorker")
          .priority(random_value.encode()[0].into())
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

    /// An example dispatchable that may throw a custom error.
    #[pallet::call_index(1)]
    #[pallet::weight(10_000 + T::DbWeight::get().reads_writes(1,1).ref_time())]
    pub fn cause_error(origin: OriginFor<T>) -> DispatchResult {
      let _who = ensure_signed(origin)?;

      // Read a value from storage.
      match <Something<T>>::get() {
        // Return an error if the value has not been set.
        None => return Err(Error::<T>::NoneValue.into()),
        Some(old) => {
          // Increment the value read from storage; will error in the event of overflow.
          let new = old.checked_add(1).ok_or(Error::<T>::StorageOverflow)?;
          // Update the value in storage with the incremented result.
          <Something<T>>::put(new);
          Ok(())
        },
      }
    }

    #[pallet::call_index(2)]
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn do_something_with_signature(
      origin: OriginFor<T>,
      _signature: T::Signature,
      payload: Payload<T::Public, T::BlockNumber, T::AccountId>,
    ) -> DispatchResult {
      // Check that the extrinsic was signed and get the signer.
      // This function will return an error if the extrinsic is not signed.
      // https://docs.substrate.io/main-docs/build/origins/
      ensure_none(origin)?;

      // Update storage.
      <Something<T>>::put(payload.number.clone());

      // Emit an event.
      Self::deposit_event(Event::SomethingStoredSigned {
        something: payload.number.clone(),
        account_id: payload.account_id.clone(),
      });
      // Return a successful DispatchResultWithPostInfo
      Ok(())
    }
  }
}
