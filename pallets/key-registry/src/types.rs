use crate::constants::ORACLE_MAX_URI_LENGTH;
use codec::Decode;
use codec::Encode;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_runtime::RuntimeDebug;
use sp_std::vec::Vec;

pub type OracleURI = BoundedVec<u8, ConstU32<ORACLE_MAX_URI_LENGTH>>;
pub type KeyType = [u8; 2];
pub type Key = Vec<u8>;
pub type KeyName = Vec<u8>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Payload<Public, BlockNumber, AccountId> {
  pub public: Public,
  pub block_number: BlockNumber,
  pub account_id: AccountId,
  pub key: Key,
  pub key_type: KeyType,
}

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, TypeInfo)]
pub struct Oracle<AccountId> {
  pub account_id: AccountId,
  pub uri: OracleURI,
}
