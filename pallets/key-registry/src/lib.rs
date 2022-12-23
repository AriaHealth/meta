#![cfg_attr(not(feature = "std"), no_std)]

use frame_support::traits::Get;
use frame_system::{
  self as system,
  offchain::{
    AppCrypto, CreateSignedTransaction, SendSignedTransaction, SendUnsignedTransaction, SignedPayload, Signer, SigningTypes, SubmitTransaction,
  },
};
use parity_scale_codec::{Decode, Encode};
use sp_core::crypto::KeyTypeId;
use sp_core::sr25519::Signature as Sr25519Signature;
use sp_runtime::{
  app_crypto::{app_crypto, sr25519},
  traits::Verify,
  MultiSignature, MultiSigner,
};
use sp_runtime::{
  offchain::{
    http,
    storage::{MutateStorageError, StorageRetrievalError, StorageValueRef},
    Duration,
  },
  traits::Zero,
  transaction_validity::{InvalidTransaction, TransactionValidity, ValidTransaction},
  RuntimeDebug,
};
use sp_std::prelude::*;
use sp_std::vec::Vec;

pub use pallet::*;

pub mod constants;
pub mod types;

mod impls;

pub const KEY_TYPE: KeyTypeId = KeyTypeId(*b"krgs");

pub mod crypto {

  app_crypto!(sr25519, KEY_TYPE);

  pub struct AuthorityId;
  // implemented for runtime
  impl frame_system::offchain::AppCrypto<MultiSigner, MultiSignature> for AuthorityId {
    type RuntimeAppPublic = Public;
    type GenericSignature = sp_core::sr25519::Signature;
    type GenericPublic = sp_core::sr25519::Public;
  }

  // implemented for mock runtime in test
  impl frame_system::offchain::AppCrypto<<Sr25519Signature as Verify>::Signer, Sr25519Signature> for AuthorityId {
    type RuntimeAppPublic = Public;
    type GenericSignature = sp_core::sr25519::Signature;
    type GenericPublic = sp_core::sr25519::Public;
  }
}

#[frame_support::pallet]
pub mod pallet {
  //! A demonstration of an offchain worker that sends onchain callbacks
  use crate::constants::UNSIGNED_TXS_PRIORITY;
  use crate::types::{Payload, TeeOracleURI};
  use core::convert::TryInto;
  use frame_support::pallet_prelude::*;
  use frame_system::{
    offchain::{AppCrypto, CreateSignedTransaction, SignedPayload, SigningTypes},
    pallet_prelude::*,
  };
  use sp_runtime::transaction_validity::{InvalidTransaction, TransactionSource, TransactionValidity, ValidTransaction};
  use sp_std::{collections::vec_deque::VecDeque, prelude::*, str};

  impl<T: SigningTypes> SignedPayload<T> for Payload<T::Public> {
    fn public(&self) -> T::Public {
      self.public.clone()
    }
  }

  #[pallet::config]
  pub trait Config: frame_system::Config + CreateSignedTransaction<Call<Self>> {
    /// The overarching event type.
    type RuntimeEvent: From<Event<Self>> + IsType<<Self as frame_system::Config>::RuntimeEvent>;
    /// The overarching dispatch call type.
    type Call: From<Call<Self>>;
    /// The identifier type for an offchain worker.
    type AuthorityId: AppCrypto<Self::Public, Self::Signature>;
  }

  #[pallet::pallet]
  #[pallet::generate_store(pub(super) trait Store)]
  #[pallet::without_storage_info]
  pub struct Pallet<T>(_);

  #[pallet::storage]
  #[pallet::getter(fn custodians)]
  pub type TeeOracles<T: Config> = StorageValue<_, Vec<TeeOracleURI>, ValueQuery>;

  // The pallet's runtime storage items.
  // https://substrate.dev/docs/en/knowledgebase/runtime/storage
  #[pallet::storage]
  #[pallet::getter(fn numbers)]
  // Learn more about declaring storage items:
  // https://substrate.dev/docs/en/knowledgebase/runtime/storage#declaring-storage-items
  pub type Numbers<T> = StorageValue<_, VecDeque<u64>, ValueQuery>;

  #[pallet::event]
  #[pallet::generate_deposit(pub(super) fn deposit_event)]
  pub enum Event<T: Config> {
    NewNumber(Option<T::AccountId>, u64),
  }

  // Errors inform users that something went wrong.
  #[pallet::error]
  pub enum Error<T> {
    // Error returned when not sure which ocw function to executed
    UnknownOffchainMux,

    // Error returned when making signed transactions in off-chain worker
    NoLocalAcctForSigning,
    OffchainSignedTxError,

    // Error returned when making unsigned transactions in off-chain worker
    OffchainUnsignedTxError,

    // Error returned when making unsigned transactions with signed payloads in off-chain worker
    OffchainUnsignedTxSignedPayloadError,

    // Error returned when fetching github info
    HttpFetchingError,
    DeserializeToObjError,
    DeserializeToStrError,
  }

  #[pallet::hooks]
  impl<T: Config> Hooks<BlockNumberFor<T>> for Pallet<T> {
    /// Offchain Worker entry point.
    ///
    /// By implementing `fn offchain_worker` you declare a new offchain worker.
    /// This function will be called when the node is fully synced and a new best block is
    /// succesfuly imported.
    /// Note that it's not guaranteed for offchain workers to run on EVERY block, there might
    /// be cases where some blocks are skipped, or for some the worker runs twice (re-orgs),
    /// so the code should be able to handle that.
    /// You can use `Local Storage` API to coordinate runs of the worker.
    fn offchain_worker(block_number: T::BlockNumber) {
      log::info!("Hello from pallet-ocw.");

      // Here we are showcasing various techniques used when running off-chain workers (ocw)
      // 1. Sending signed transaction from ocw
      // 2. Sending unsigned transaction from ocw
      // 3. Sending unsigned transactions with signed payloads from ocw
      // 4. Fetching JSON via http requests in ocw
      const TX_TYPES: u32 = 4;
      let modu = block_number.try_into().map_or(TX_TYPES, |bn: usize| (bn as u32) % TX_TYPES);
      let result = match modu {
        0 => Self::offchain_signed_tx(block_number),
        1 => Self::offchain_unsigned_tx(block_number),
        2 => Self::offchain_unsigned_tx_signed_payload(block_number),
        3 => Self::fetch_remote_info(),
        _ => Err(Error::<T>::UnknownOffchainMux),
      };

      if let Err(e) = result {
        log::error!("offchain_worker error: {:?}", e);
      }
    }
  }

  #[pallet::validate_unsigned]
  impl<T: Config> ValidateUnsigned for Pallet<T> {
    type Call = Call<T>;

    /// Validate unsigned call to this module.
    ///
    /// By default unsigned transactions are disallowed, but implementing the validator
    /// here we make sure that some particular calls (the ones produced by offchain worker)
    /// are being whitelisted and marked as valid.
    fn validate_unsigned(_source: TransactionSource, call: &Self::Call) -> TransactionValidity {
      let valid_tx = |provide| {
        ValidTransaction::with_tag_prefix("ocw-demo")
          .priority(UNSIGNED_TXS_PRIORITY)
          .and_provides([&provide])
          .longevity(3)
          .propagate(true)
          .build()
      };

      match call {
        Call::submit_number_unsigned { number: _number } => valid_tx(b"submit_number_unsigned".to_vec()),
        Call::submit_number_unsigned_with_signed_payload { ref payload, ref signature } => {
          if !SignedPayload::<T>::verify::<T::AuthorityId>(payload, signature.clone()) {
            return InvalidTransaction::BadProof.into();
          }
          valid_tx(b"submit_number_unsigned_with_signed_payload".to_vec())
        },
        _ => InvalidTransaction::Call.into(),
      }
    }
  }

  #[pallet::call]
  impl<T: Config> Pallet<T> {
    #[pallet::weight(10000)]
    pub fn submit_number_signed(origin: OriginFor<T>, number: u64) -> DispatchResult {
      let who = ensure_signed(origin)?;
      log::info!("submit_number_signed: ({}, {:?})", number, who);
      Self::append_or_replace_number(number);

      Self::deposit_event(Event::NewNumber(Some(who), number));
      Ok(())
    }

    #[pallet::weight(10000)]
    pub fn submit_number_unsigned(origin: OriginFor<T>, number: u64) -> DispatchResult {
      let _ = ensure_none(origin)?;
      log::info!("submit_number_unsigned: {}", number);
      Self::append_or_replace_number(number);

      Self::deposit_event(Event::NewNumber(None, number));
      Ok(())
    }

    #[pallet::weight(10000)]
    #[allow(unused_variables)]
    pub fn submit_number_unsigned_with_signed_payload(origin: OriginFor<T>, payload: Payload<T::Public>, signature: T::Signature) -> DispatchResult {
      let _ = ensure_none(origin)?;
      // we don't need to verify the signature here because it has been verified in
      //   `validate_unsigned` function when sending out the unsigned tx.
      let Payload { number, public } = payload;
      log::info!("submit_number_unsigned_with_signed_payload: ({}, {:?})", number, public);
      Self::append_or_replace_number(number);

      Self::deposit_event(Event::NewNumber(None, number));
      Ok(())
    }
  }
}
