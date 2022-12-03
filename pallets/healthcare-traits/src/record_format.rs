#![cfg_attr(not(feature = "std"), no_std)]

use codec::{Decode, Encode};
use scale_info::TypeInfo;
use serde::{Deserialize, Serialize};
use sp_runtime::RuntimeDebug;
use sp_std::cmp::{Eq, PartialEq};

#[derive(Copy, Clone, Encode, Decode, PartialEq, Eq, RuntimeDebug, TypeInfo)]
#[cfg_attr(feature = "std", derive(Serialize, Deserialize))]
pub enum RecordFormat {
    Hl7v2 = 0,
    Dicom = 1,
    FhirDstu1 = 2,
    FhirDstu2_1 = 3,
    FhirDstu3 = 4,
    FhirStu3 = 5,
    FhirR4 = 6,
    FhirR5 = 7,
}
