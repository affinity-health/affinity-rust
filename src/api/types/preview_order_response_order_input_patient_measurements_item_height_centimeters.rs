pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PreviewOrderResponseOrderInputPatientMeasurementsItemHeightCentimeters {
    Double(f64),

    PreviewOrderResponseOrderInputPatientMeasurementsItemHeightCentimetersOne(
        PreviewOrderResponseOrderInputPatientMeasurementsItemHeightCentimetersOne,
    ),
}

impl PreviewOrderResponseOrderInputPatientMeasurementsItemHeightCentimeters {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_preview_order_response_order_input_patient_measurements_item_height_centimeters_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::PreviewOrderResponseOrderInputPatientMeasurementsItemHeightCentimetersOne(_)
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

    pub fn as_preview_order_response_order_input_patient_measurements_item_height_centimeters_one(
        &self,
    ) -> Option<&PreviewOrderResponseOrderInputPatientMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::PreviewOrderResponseOrderInputPatientMeasurementsItemHeightCentimetersOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn into_preview_order_response_order_input_patient_measurements_item_height_centimeters_one(
        self,
    ) -> Option<PreviewOrderResponseOrderInputPatientMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::PreviewOrderResponseOrderInputPatientMeasurementsItemHeightCentimetersOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for PreviewOrderResponseOrderInputPatientMeasurementsItemHeightCentimeters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::PreviewOrderResponseOrderInputPatientMeasurementsItemHeightCentimetersOne(
                value,
            ) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
