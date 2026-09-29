pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreatePatientRequestMeasurementsItemHeightCentimeters {
    Double(f64),

    CreatePatientRequestMeasurementsItemHeightCentimetersOne(
        CreatePatientRequestMeasurementsItemHeightCentimetersOne,
    ),
}

impl CreatePatientRequestMeasurementsItemHeightCentimeters {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_patient_request_measurements_item_height_centimeters_one(&self) -> bool {
        matches!(
            self,
            Self::CreatePatientRequestMeasurementsItemHeightCentimetersOne(_)
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

    pub fn as_create_patient_request_measurements_item_height_centimeters_one(
        &self,
    ) -> Option<&CreatePatientRequestMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::CreatePatientRequestMeasurementsItemHeightCentimetersOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_patient_request_measurements_item_height_centimeters_one(
        self,
    ) -> Option<CreatePatientRequestMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::CreatePatientRequestMeasurementsItemHeightCentimetersOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreatePatientRequestMeasurementsItemHeightCentimeters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreatePatientRequestMeasurementsItemHeightCentimetersOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
