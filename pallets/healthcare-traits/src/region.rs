#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::cmp::{Eq, PartialEq};

#[derive(Copy, Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Region {
    Europe = 0,
    MiddleEast = 1,
    NorthAfrica = 2,
    UAE = 3,
}
