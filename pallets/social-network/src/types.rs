use crate::constants::GROUP_INFO_MAX_LEN;
use crate::constants::GROUP_OID_MAX_LEN;
use crate::constants::USER_INFO_MAX_LEN;
use ap_region::Country;
use ap_region::Region;
use ap_region::SubRegion;
use codec::Decode;
use codec::Encode;
use codec::MaxEncodedLen;
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::cmp::Eq;
use sp_std::cmp::PartialEq;

pub type GroupInfo = BoundedVec<u8, ConstU32<GROUP_INFO_MAX_LEN>>;
pub type AccountInfo = BoundedVec<u8, ConstU32<USER_INFO_MAX_LEN>>;
pub type GroupId = BoundedVec<u8, ConstU32<GROUP_OID_MAX_LEN>>;

#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen, Copy, RuntimeDebug)]
pub enum Permission {
  SuperAdmin,
  Admin,
  CanTransact,
  CanWrite,
  ReadOnly,
  Customer,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen, Copy)]
pub enum Relation {
  Deleted,
  Connected,
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub enum AccountStatus {
  /// The account is active and able to do activity in the network
  Live,
  /// Whether the account is frozen for doing any activity in the network.
  Frozen,
}

impl Default for AccountStatus {
  fn default() -> Self {
    Self::Live
  }
}

#[derive(Clone, Encode, Decode, Eq, PartialEq, RuntimeDebug, MaxEncodedLen, TypeInfo)]
pub struct AccountDetail<AccountId> {
  /// Who first allow this account to join the network
  pub issuer: AccountId,

  /// Who freeze this user
  pub freezer: Option<AccountId>,

  /// The status of this account to do activity in the network
  pub status: AccountStatus,

  /// The externally stored account information
  pub info: AccountInfo,
}

#[derive(Clone, Encode, Decode, PartialEq, RuntimeDebug, TypeInfo, MaxEncodedLen)]
pub struct Group<AccountId> {
  pub admin_id: AccountId,
  pub issuer_id: AccountId,
  pub info: GroupInfo,
  pub country: Country,
  pub region: Region,
  pub sub_region: SubRegion,
  pub admin_count: u32,
  pub member_count: u32,
}
