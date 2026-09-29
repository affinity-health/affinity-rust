pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateOrderBatchRequestOrdersItemPatientMeasurementsItemWeightKilograms {
    Double(f64),

    CreateOrderBatchRequestOrdersItemPatientMeasurementsItemWeightKilogramsOne(
        CreateOrderBatchRequestOrdersItemPatientMeasurementsItemWeightKilogramsOne,
    ),
}

impl CreateOrderBatchRequestOrdersItemPatientMeasurementsItemWeightKilograms {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_order_batch_request_orders_item_patient_measurements_item_weight_kilograms_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::CreateOrderBatchRequestOrdersItemPatientMeasurementsItemWeightKilogramsOne(_)
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

    pub fn as_create_order_batch_request_orders_item_patient_measurements_item_weight_kilograms_one(
        &self,
    ) -> Option<&CreateOrderBatchRequestOrdersItemPatientMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::CreateOrderBatchRequestOrdersItemPatientMeasurementsItemWeightKilogramsOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_order_batch_request_orders_item_patient_measurements_item_weight_kilograms_one(
        self,
    ) -> Option<CreateOrderBatchRequestOrdersItemPatientMeasurementsItemWeightKilogramsOne> {
        match self {
            Self::CreateOrderBatchRequestOrdersItemPatientMeasurementsItemWeightKilogramsOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateOrderBatchRequestOrdersItemPatientMeasurementsItemWeightKilograms {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreateOrderBatchRequestOrdersItemPatientMeasurementsItemWeightKilogramsOne(
                value,
            ) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
