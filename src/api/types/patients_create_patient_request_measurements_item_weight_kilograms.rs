pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreatePatientRequestMeasurementsItemWeightKilograms {
    Double(f64),

    CreatePatientRequestMeasurementsItemWeightKilogramsOne(
        CreatePatientRequestMeasurementsItemWeightKilogramsOne,
    ),
}

impl CreatePatientRequestMeasurementsItemWeightKilograms {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_patient_request_measurements_item_weight_kilograms_one(&self) -> bool {
        matches!(
            self,
            Self::CreatePatientRequestMeasurementsItemWeightKilogramsOne(_)
        )
    }

    pub fn as_double(&self) -> Option<&f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_double(self) -> Option<f64> {
        match self {
            Self::Double(value) => Some(value),
            _ => None,
        }
    }

    pub fn as_create_patient_request_measurements_item_weight_kilograms_one(
        &self,
    ) -> Option<&CreatePatientRequestMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::CreatePatientRequestMeasurementsItemWeightKilogramsOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_patient_request_measurements_item_weight_kilograms_one(
        self,
    ) -> Option<CreatePatientRequestMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::CreatePatientRequestMeasurementsItemWeightKilogramsOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreatePatientRequestMeasurementsItemWeightKilograms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreatePatientRequestMeasurementsItemWeightKilogramsOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
