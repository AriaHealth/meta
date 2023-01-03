#![cfg_attr(not(feature = "std"), no_std)]

use codec::Decode;
use codec::Encode;
use scale_info::TypeInfo;
use sp_std::cmp::Eq;
use sp_std::cmp::PartialEq;

// Enum declaration for User role.
#[derive(Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
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

#[derive(Copy, Clone, Encode, Decode, PartialEq, Eq, TypeInfo)]
pub enum TherapeuticArea {
  Ageing,
  BoneJointConnectiveTissue,
  Cardiovascular,
  Dermatology,
  Endocrinology,
  Gastroenterology,
  Haematology,
  Immunology,
  InfectiousDiseaseOrVaccines,
  Inflammation,
  LiverDisease,
  MentalHealth,
  Metabolism,
  Neurology,
  Oncology,
  Ophthalmology,
  OralHealth,
  Orthopaedics,
  Otolaryngology,
  Paediatrics,
  Pulmonology,
  Reproduction,
  RareDiseases,
  RegenerativeMedicine,
  Respiratory,
  UrologyRenalUrogenital,
  WomensHealth,
}
