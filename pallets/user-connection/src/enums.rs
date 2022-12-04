use codec::{Decode, Encode, MaxEncodedLen};
use scale_info::TypeInfo;
use sp_std::cmp::{Eq, PartialEq};

#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo, MaxEncodedLen, Copy)]
pub enum Relation {
    ConnectedInclusive,
    ConnectedExclusive,
    Blocked,
}
