// Only for template pallet. Delete the warning surpressors if you are not using template pallet.
#![allow(unused_variables)]
#![allow(unused_imports)]

use frame_support::pallet_prelude::*;
use frame_support::traits::Randomness;
use frame_system::offchain::AppCrypto;
use frame_system::offchain::CreateSignedTransaction;
use frame_system::offchain::SendSignedTransaction;
use frame_system::offchain::SendUnsignedTransaction;
use frame_system::offchain::SignedPayload;
use frame_system::offchain::Signer;
use frame_system::offchain::SigningTypes;
use frame_system::offchain::SubmitTransaction;
use frame_system::pallet_prelude::*;
use sp_runtime::offchain::storage::StorageValueRef;
use sp_runtime::offchain::storage_lock::BlockAndTime;
use sp_runtime::offchain::storage_lock::StorageLock;
use sp_runtime::offchain::Duration;
use sp_runtime::traits::BlockNumberProvider;
use sp_runtime::transaction_validity::InvalidTransaction;
use sp_runtime::transaction_validity::TransactionValidity;
use sp_runtime::transaction_validity::ValidTransaction;
use sp_std::vec::Vec;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct Payload<Public, BlockNumber, AcountId> {
  pub public: Public,
  pub block_number: BlockNumber,
  pub account_id: AcountId,
  pub number: u32,
}
