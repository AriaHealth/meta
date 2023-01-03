// Only for template pallet. Delete the warning surpressors if you are not using template pallet.
#![allow(unused_variables)]
#![allow(unused_imports)]
#![cfg_attr(not(feature = "std"), no_std)]
use super::pallet::*;

use crate::types::Payload;
use frame_support::ensure;
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

impl<T: Config> Pallet<T> {
  pub fn get_and_increment_nonce() -> Vec<u8> {
    let nonce = Nonce::<T>::get().unwrap_or(0);
    Nonce::<T>::put(nonce.wrapping_add(1));
    nonce.encode()
  }
}

impl<T: Config> BlockNumberProvider for Pallet<T> {
  type BlockNumber = T::BlockNumber;

  fn current_block_number() -> Self::BlockNumber {
    <frame_system::Pallet<T>>::block_number()
  }
}

impl<T: SigningTypes> SignedPayload<T> for Payload<T::Public, T::BlockNumber, T::AccountId> {
  fn public(&self) -> T::Public {
    self.public.clone()
  }
}
