pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GetPatientResponseMeasurementsItemWeightKilograms {
    Double(f64),

    GetPatientResponseMeasurementsItemWeightKilogramsOne(
        GetPatientResponseMeasurementsItemWeightKilogramsOne,
    ),
}

impl GetPatientResponseMeasurementsItemWeightKilograms {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_get_patient_response_measurements_item_weight_kilograms_one(&self) -> bool {
        matches!(
            self,
            Self::GetPatientResponseMeasurementsItemWeightKilogramsOne(_)
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

    pub fn as_get_patient_response_measurements_item_weight_kilograms_one(
        &self,
    ) -> Option<&GetPatientResponseMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::GetPatientResponseMeasurementsItemWeightKilogramsOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_get_patient_response_measurements_item_weight_kilograms_one(
        self,
    ) -> Option<GetPatientResponseMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::GetPatientResponseMeasurementsItemWeightKilogramsOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for GetPatientResponseMeasurementsItemWeightKilograms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::GetPatientResponseMeasurementsItemWeightKilogramsOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
