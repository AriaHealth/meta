#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::cmp::{Eq, PartialEq};

// Enum declaration for User role.
#[derive(Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum Role {
    Custodian = 1,
    Virtual = 2,
    Patient = 3,
    FamilyHealth = 4,
    MedicalProfessional = 5,
    NonMedicalPersonnel = 6,
    HealthFacility = 7,
    Aggregator = 8,
    Researcher = 9,
    Enterprise = 10,
    EnterpriseEmployee = 11,
}
