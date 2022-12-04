use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::cmp::{Eq, PartialEq};

#[derive(Copy, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
pub enum Region {
    Europe = 0,
    MiddleEast = 1,
    NorthAfrica = 2,
    UAE = 3,
}
