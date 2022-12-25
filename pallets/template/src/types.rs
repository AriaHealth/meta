// Only for template pallet. Delete the warning surpressors if you are not using template pallet.
#![allow(unused_variables)]
#![allow(unused_imports)]

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

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct Payload<Public, BlockNumber, AcountId> {
  pub public: Public,
  pub block_number: BlockNumber,
  pub account_id: AcountId,
  pub number: u32,
}
