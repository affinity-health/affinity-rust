pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PreviewOrderRequestPatientMeasurementsItemHeightCentimeters {
    Double(f64),

    PreviewOrderRequestPatientMeasurementsItemHeightCentimetersOne(
        PreviewOrderRequestPatientMeasurementsItemHeightCentimetersOne,
    ),
}

impl PreviewOrderRequestPatientMeasurementsItemHeightCentimeters {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_preview_order_request_patient_measurements_item_height_centimeters_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::PreviewOrderRequestPatientMeasurementsItemHeightCentimetersOne(_)
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

    pub fn as_preview_order_request_patient_measurements_item_height_centimeters_one(
        &self,
    ) -> Option<&PreviewOrderRequestPatientMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::PreviewOrderRequestPatientMeasurementsItemHeightCentimetersOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }

    pub fn into_preview_order_request_patient_measurements_item_height_centimeters_one(
        self,
    ) -> Option<PreviewOrderRequestPatientMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::PreviewOrderRequestPatientMeasurementsItemHeightCentimetersOne(value) => {
                Some(value)
            }
            _ => None,
        }
    }
}

impl fmt::Display for PreviewOrderRequestPatientMeasurementsItemHeightCentimeters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::PreviewOrderRequestPatientMeasurementsItemHeightCentimetersOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
