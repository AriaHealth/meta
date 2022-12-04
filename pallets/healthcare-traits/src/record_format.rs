use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::cmp::{Eq, PartialEq};

#[derive(Copy, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
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
