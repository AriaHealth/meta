use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::cmp::{Eq, PartialEq};

#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
pub enum Relation {
    ConnectedInclusive,
    ConnectedExclusive,
    Blocker,
}
