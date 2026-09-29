pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreatePatientResponseMeasurementsItemHeightCentimeters {
    Double(f64),

    CreatePatientResponseMeasurementsItemHeightCentimetersOne(
        CreatePatientResponseMeasurementsItemHeightCentimetersOne,
    ),
}

impl CreatePatientResponseMeasurementsItemHeightCentimeters {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_patient_response_measurements_item_height_centimeters_one(&self) -> bool {
        matches!(
            self,
            Self::CreatePatientResponseMeasurementsItemHeightCentimetersOne(_)
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

    pub fn as_create_patient_response_measurements_item_height_centimeters_one(
        &self,
    ) -> Option<&CreatePatientResponseMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::CreatePatientResponseMeasurementsItemHeightCentimetersOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_patient_response_measurements_item_height_centimeters_one(
        self,
    ) -> Option<CreatePatientResponseMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::CreatePatientResponseMeasurementsItemHeightCentimetersOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreatePatientResponseMeasurementsItemHeightCentimeters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreatePatientResponseMeasurementsItemHeightCentimetersOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
