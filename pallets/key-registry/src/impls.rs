use super::pallet::*;

use crate::types::Payload;

use frame_support::pallet_prelude::*;

use frame_system::offchain::{SignedPayload, SigningTypes};
use sp_runtime::traits::BlockNumberProvider;
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
