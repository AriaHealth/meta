use crate::constants::GROUP_INFO_MAX_LEN;

use codec::{Decode, Encode, MaxEncodedLen};
use frame_support::pallet_prelude::*;
use scale_info::TypeInfo;
use sp_std::cmp::{Eq, PartialEq};

pub type GroupInfo = BoundedVec<u8, ConstU32<GROUP_INFO_MAX_LEN>>;
pub type GroupId = [u8; 32];

#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen, Copy)]
pub enum AccessControl {
    SuperAdmin,
    Admin,
    ReadOnly,
    ReadWrite,
    Member,
    PendingSuperAdmin,
    PendingAdmin,
    PendingReadOnly,
    PendingReadWrite,
    PendingMember,
}

#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen, Copy)]
pub enum Relation {
    Pending,
    Connected,
}
