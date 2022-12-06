use codec::{Decode, Encode};
use scale_info::TypeInfo;
use sp_std::cmp::{Eq, PartialEq};

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
