pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PreviewOrderResponseOrderInputPatientMeasurementsItemWeightKilograms {
    Double(f64),

    PreviewOrderResponseOrderInputPatientMeasurementsItemWeightKilogramsOne(
        PreviewOrderResponseOrderInputPatientMeasurementsItemWeightKilogramsOne,
    ),
}

impl PreviewOrderResponseOrderInputPatientMeasurementsItemWeightKilograms {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_preview_order_response_order_input_patient_measurements_item_weight_kilograms_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::PreviewOrderResponseOrderInputPatientMeasurementsItemWeightKilogramsOne(_)
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

    pub fn as_preview_order_response_order_input_patient_measurements_item_weight_kilograms_one(
        &self,
    ) -> Option<&PreviewOrderResponseOrderInputPatientMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::PreviewOrderResponseOrderInputPatientMeasurementsItemWeightKilogramsOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn into_preview_order_response_order_input_patient_measurements_item_weight_kilograms_one(
        self,
    ) -> Option<PreviewOrderResponseOrderInputPatientMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::PreviewOrderResponseOrderInputPatientMeasurementsItemWeightKilogramsOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for PreviewOrderResponseOrderInputPatientMeasurementsItemWeightKilograms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::PreviewOrderResponseOrderInputPatientMeasurementsItemWeightKilogramsOne(
                value,
            ) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
