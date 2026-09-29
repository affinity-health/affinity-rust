pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum UpdatePatientRequestMeasurementsItemHeightCentimeters {
    Double(f64),

    UpdatePatientRequestMeasurementsItemHeightCentimetersOne(
        UpdatePatientRequestMeasurementsItemHeightCentimetersOne,
    ),
}

impl UpdatePatientRequestMeasurementsItemHeightCentimeters {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_update_patient_request_measurements_item_height_centimeters_one(&self) -> bool {
        matches!(
            self,
            Self::UpdatePatientRequestMeasurementsItemHeightCentimetersOne(_)
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

    pub fn as_update_patient_request_measurements_item_height_centimeters_one(
        &self,
    ) -> Option<&UpdatePatientRequestMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::UpdatePatientRequestMeasurementsItemHeightCentimetersOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_update_patient_request_measurements_item_height_centimeters_one(
        self,
    ) -> Option<UpdatePatientRequestMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::UpdatePatientRequestMeasurementsItemHeightCentimetersOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for UpdatePatientRequestMeasurementsItemHeightCentimeters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::UpdatePatientRequestMeasurementsItemHeightCentimetersOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
