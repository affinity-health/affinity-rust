pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateOrderRequestPatientMeasurementsItemWeightKilograms {
    Double(f64),

    CreateOrderRequestPatientMeasurementsItemWeightKilogramsOne(
        CreateOrderRequestPatientMeasurementsItemWeightKilogramsOne,
    ),
}

impl CreateOrderRequestPatientMeasurementsItemWeightKilograms {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_order_request_patient_measurements_item_weight_kilograms_one(&self) -> bool {
        matches!(
            self,
            Self::CreateOrderRequestPatientMeasurementsItemWeightKilogramsOne(_)
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

    pub fn as_create_order_request_patient_measurements_item_weight_kilograms_one(
        &self,
    ) -> Option<&CreateOrderRequestPatientMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::CreateOrderRequestPatientMeasurementsItemWeightKilogramsOne(value) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_order_request_patient_measurements_item_weight_kilograms_one(
        self,
    ) -> Option<CreateOrderRequestPatientMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::CreateOrderRequestPatientMeasurementsItemWeightKilogramsOne(value) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateOrderRequestPatientMeasurementsItemWeightKilograms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreateOrderRequestPatientMeasurementsItemWeightKilogramsOne(value) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
