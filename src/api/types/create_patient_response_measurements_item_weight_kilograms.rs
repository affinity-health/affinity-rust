pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreatePatientResponseMeasurementsItemWeightKilograms {
    Double(f64),

    CreatePatientResponseMeasurementsItemWeightKilogramsOne(
        CreatePatientResponseMeasurementsItemWeightKilogramsOne,
    ),
}

impl CreatePatientResponseMeasurementsItemWeightKilograms {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_patient_response_measurements_item_weight_kilograms_one(&self) -> bool {
        matches!(
            self,
            Self::CreatePatientResponseMeasurementsItemWeightKilogramsOne(_)
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

    pub fn as_create_patient_response_measurements_item_weight_kilograms_one(
        &self,
    ) -> Option<&CreatePatientResponseMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::CreatePatientResponseMeasurementsItemWeightKilogramsOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_patient_response_measurements_item_weight_kilograms_one(
        self,
    ) -> Option<CreatePatientResponseMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::CreatePatientResponseMeasurementsItemWeightKilogramsOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreatePatientResponseMeasurementsItemWeightKilograms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreatePatientResponseMeasurementsItemWeightKilogramsOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
