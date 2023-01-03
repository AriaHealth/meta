use codec::Decode;
use codec::Encode;
use frame_support::pallet_prelude::*;
use sp_runtime::RuntimeDebug;

use crate::constants::TEE_ORACLE_MAX_URI_LENGTH;

pub type TeeOracleURI = BoundedVec<u8, ConstU32<TEE_ORACLE_MAX_URI_LENGTH>>;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct Payload<Public, BlockNumber, AcountId> {
  pub public: Public,
  pub block_number: BlockNumber,
  pub account_id: AcountId,
  pub number: u32,
}
