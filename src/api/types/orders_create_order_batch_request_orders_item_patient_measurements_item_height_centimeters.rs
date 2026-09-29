pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateOrderBatchRequestOrdersItemPatientMeasurementsItemHeightCentimeters {
    Double(f64),

    CreateOrderBatchRequestOrdersItemPatientMeasurementsItemHeightCentimetersOne(
        CreateOrderBatchRequestOrdersItemPatientMeasurementsItemHeightCentimetersOne,
    ),
}

impl CreateOrderBatchRequestOrdersItemPatientMeasurementsItemHeightCentimeters {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_order_batch_request_orders_item_patient_measurements_item_height_centimeters_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::CreateOrderBatchRequestOrdersItemPatientMeasurementsItemHeightCentimetersOne(_)
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

    pub fn as_create_order_batch_request_orders_item_patient_measurements_item_height_centimeters_one(
        &self,
    ) -> Option<&CreateOrderBatchRequestOrdersItemPatientMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::CreateOrderBatchRequestOrdersItemPatientMeasurementsItemHeightCentimetersOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn into_create_order_batch_request_orders_item_patient_measurements_item_height_centimeters_one(
        self,
    ) -> Option<CreateOrderBatchRequestOrdersItemPatientMeasurementsItemHeightCentimetersOne> {
        match self {
            Self::CreateOrderBatchRequestOrdersItemPatientMeasurementsItemHeightCentimetersOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for CreateOrderBatchRequestOrdersItemPatientMeasurementsItemHeightCentimeters {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreateOrderBatchRequestOrdersItemPatientMeasurementsItemHeightCentimetersOne(
                value,
            ) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
