pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UpdatePatientResponseMeasurementsItemHeightCentimeters {
    Double(f64),

    UpdatePatientResponseMeasurementsItemHeightCentimetersOne(
        UpdatePatientResponseMeasurementsItemHeightCentimetersOne,
    ),
}

impl UpdatePatientResponseMeasurementsItemHeightCentimeters {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_update_patient_response_measurements_item_height_centimeters_one(&self) -> bool {
        matches!(
            self,
            Self::UpdatePatientResponseMeasurementsItemHeightCentimetersOne(_)
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

    pub fn as_update_patient_response_measurements_item_height_centimeters_one(
        &self,
    ) -> Option<&UpdatePatientResponseMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::UpdatePatientResponseMeasurementsItemHeightCentimetersOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_update_patient_response_measurements_item_height_centimeters_one(
        self,
    ) -> Option<UpdatePatientResponseMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::UpdatePatientResponseMeasurementsItemHeightCentimetersOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for UpdatePatientResponseMeasurementsItemHeightCentimeters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::UpdatePatientResponseMeasurementsItemHeightCentimetersOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
