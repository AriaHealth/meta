use frame_support::pallet_prelude::*;

#[derive(Encode, Decode, Clone, PartialEq, Eq, RuntimeDebug, scale_info::TypeInfo)]
pub struct Payload<Public, BlockNumber, AcountId> {
  pub public: Public,
  pub block_number: BlockNumber,
  pub account_id: AcountId,
  pub number: u32,
}
