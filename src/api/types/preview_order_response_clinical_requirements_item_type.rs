pub use crate::prelude::*;

#[non_exhaustive]
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum PreviewOrderResponseClinicalRequirementsItemType {
    AllergyReview,
    MedicationReview,
    DiagnosisReview,
    Diagnosis,
    /// This variant is used for forward compatibility.
    /// If the server sends a value not recognized by the current SDK version,
    /// it will be captured here with the raw string value.
    __Unknown(String),
}
impl Serialize for PreviewOrderResponseClinicalRequirementsItemType {
    fn serialize<S: serde::Serializer>(&self, serializer: S) -> Result<S::Ok, S::Error> {
        match self {
            Self::AllergyReview => serializer.serialize_str("allergy_review"),
            Self::MedicationReview => serializer.serialize_str("medication_review"),
            Self::DiagnosisReview => serializer.serialize_str("diagnosis_review"),
            Self::Diagnosis => serializer.serialize_str("diagnosis"),
            Self::__Unknown(val) => serializer.serialize_str(val),
        }
    }
}

impl<'de> Deserialize<'de> for PreviewOrderResponseClinicalRequirementsItemType {
    fn deserialize<D: serde::Deserializer<'de>>(deserializer: D) -> Result<Self, D::Error> {
        let value = String::deserialize(deserializer)?;
        match value.as_str() {
            "allergy_review" => Ok(Self::AllergyReview),
            "medication_review" => Ok(Self::MedicationReview),
            "diagnosis_review" => Ok(Self::DiagnosisReview),
            "diagnosis" => Ok(Self::Diagnosis),
            _ => Ok(Self::__Unknown(value)),
        }
    }
}

impl fmt::Display for PreviewOrderResponseClinicalRequirementsItemType {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::AllergyReview => write!(f, "allergy_review"),
            Self::MedicationReview => write!(f, "medication_review"),
            Self::DiagnosisReview => write!(f, "diagnosis_review"),
            Self::Diagnosis => write!(f, "diagnosis"),
            Self::__Unknown(val) => write!(f, "{}", val),
        }
    }
}
