pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum GetPatientResponseMeasurementsItemHeightCentimeters {
    Double(f64),

    GetPatientResponseMeasurementsItemHeightCentimetersOne(
        GetPatientResponseMeasurementsItemHeightCentimetersOne,
    ),
}

impl GetPatientResponseMeasurementsItemHeightCentimeters {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_get_patient_response_measurements_item_height_centimeters_one(&self) -> bool {
        matches!(
            self,
            Self::GetPatientResponseMeasurementsItemHeightCentimetersOne(_)
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

    pub fn as_get_patient_response_measurements_item_height_centimeters_one(
        &self,
    ) -> Option<&GetPatientResponseMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::GetPatientResponseMeasurementsItemHeightCentimetersOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_get_patient_response_measurements_item_height_centimeters_one(
        self,
    ) -> Option<GetPatientResponseMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::GetPatientResponseMeasurementsItemHeightCentimetersOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for GetPatientResponseMeasurementsItemHeightCentimeters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::GetPatientResponseMeasurementsItemHeightCentimetersOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
