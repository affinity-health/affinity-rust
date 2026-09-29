pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UpdatePatientResponseMeasurementsItemWeightKilograms {
    Double(f64),

    UpdatePatientResponseMeasurementsItemWeightKilogramsOne(
        UpdatePatientResponseMeasurementsItemWeightKilogramsOne,
    ),
}

impl UpdatePatientResponseMeasurementsItemWeightKilograms {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_update_patient_response_measurements_item_weight_kilograms_one(&self) -> bool {
        matches!(
            self,
            Self::UpdatePatientResponseMeasurementsItemWeightKilogramsOne(_)
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

    pub fn as_update_patient_response_measurements_item_weight_kilograms_one(
        &self,
    ) -> Option<&UpdatePatientResponseMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::UpdatePatientResponseMeasurementsItemWeightKilogramsOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_update_patient_response_measurements_item_weight_kilograms_one(
        self,
    ) -> Option<UpdatePatientResponseMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::UpdatePatientResponseMeasurementsItemWeightKilogramsOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for UpdatePatientResponseMeasurementsItemWeightKilograms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::UpdatePatientResponseMeasurementsItemWeightKilogramsOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
