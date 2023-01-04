#![cfg_attr(not(feature = "std"), no_std)]

pub use pallet::*;

pub mod constants;
pub mod crypto;
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
  use crate::traits::KeyRules;
  use crate::types::Key;
  use crate::types::KeyName;
  use crate::types::KeyType;
  use crate::types::OracleURI;
  use crate::types::Payload;
  use frame_support::pallet_prelude::*;
  use frame_system::offchain::AppCrypto;
  use frame_system::offchain::CreateSignedTransaction;
  use frame_system::offchain::SendUnsignedTransaction;
  use frame_system::offchain::SignedPayload;
  use frame_system::offchain::Signer;
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

  #[pallet::config]
  pub trait Config: CreateSignedTransaction<Call<Self>> + frame_system::Config {
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;

    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;

    #[pallet::constant]
    type WorkerInterval: Get<u64>;

    type KeyRules: KeyRules<Self::AccountId>;
  }

  #[pallet::storage]
  #[pallet::getter(fn oracles)]
  pub type Oracles<T: Config> = StorageValue<_, Vec<OracleURI>, ValueQuery>;

  #[pallet::storage]
  #[pallet::getter(fn chunk_block)]
  pub type KeyTypes<T: Config> = StorageMap<_, Twox64Concat, KeyType, KeyName>;

  #[pallet::storage]
  #[pallet::getter(fn registries)]
  pub type Registries<T: Config> = StorageDoubleMap<_, Blake2_128Concat, T::AccountId, Blake2_128Concat, KeyType, Key>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    KeySynchronized { account_id: T::AccountId, key_type: KeyType },
  }

  #[pallet::error]
  pub enum Error<T> {
    StorageOverflow,
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    fn offchain_worker(block_number: T::BlockNumber) {
      // Locking mechanism
      let mut lock = StorageLock::<BlockAndTime<Self>>::with_block_and_time_deadline(b"offchain-demo::lock", 3, Duration::from_millis(6000));

      log::info!("🤖 Key registry is started. [blocknumber: {:?}]", block_number);

      if let Ok(_guard) = lock.try_lock() {
        let mut is_submitting = false;
        let number: u64 = block_number.try_into().unwrap_or(0);

        match number % T::WorkerInterval::get() {
          0 => {
            let key = Vec::new();
            let key_type = [0u8; 2];
            log::info!("🤖 Key registry is fetching keys. [blocknumber: {:?}]]", block_number);

            // TODO: fetch key from oracle

            log::info!("🤖 Key registry is submitting transaction. [blocknumber: {:?}]", block_number);
            if !key.is_empty() {
              Signer::<T, T::AuthorityId>::any_account().send_unsigned_transaction(
                |account| Payload {
                  block_number,
                  public: account.public.clone(),
                  account_id: account.id.clone(),
                  key: key.clone(),
                  key_type,
                },
                |payload, signature| Call::synchronize_key { signature, payload },
              );
              is_submitting = true;
            }
          },
          _ => (),
        }

        if !is_submitting {
          log::info!("🤖 Meta registry is skipped. [blocknumber: {:?}]", block_number);
        } else {
          log::info!("🤖 Key registry is finished. [blocknumber: {:?}]", block_number);
        }
      };
    }
  }

  #[pallet::validate_unsigned]
  impl<T: Config> ValidateUnsigned for Pallet<T> {
    type Call = Call<T>;

    fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
      if let Call::synchronize_key { signature, payload } = call {
        let signature_valid = SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone());

        if !signature_valid {
          InvalidTransaction::Call.into()
        } else {
          ValidTransaction::with_tag_prefix("kyrg::synchronize_key")
            .priority(3)
            .and_provides(payload.block_number)
            .longevity(5)
            .propagate(true)
            .build()
        }
      } else {
        InvalidTransaction::Call.into()
      }
    }
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::call_index(0)]
    #[pallet::weight(10_000 + T::DbWeight::get().writes(1).ref_time())]
    pub fn synchronize_key(
      origin: OriginFor<T>,
      _signature: T::Signature,
      payload: Payload<T::Public, T::BlockNumber, T::AccountId>,
    ) -> DispatchResult {
      ensure_none(origin)?;

      Self::deposit_event(Event::KeySynchronized {
        account_id: payload.account_id.clone(),
        key_type: payload.key_type,
      });

      Ok(())
    }
  }
}
