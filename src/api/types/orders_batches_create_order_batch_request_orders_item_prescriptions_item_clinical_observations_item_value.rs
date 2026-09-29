pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemValue {
    Double(f64),

    CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemValueOne(
        CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemValueOne,
    ),
}

impl CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemValue {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_create_order_batch_request_orders_item_prescriptions_item_clinical_observations_item_value_one(
        &self,
    ) -> bool {
        matches!(self, Self::CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemValueOne(_))
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

    pub fn as_create_order_batch_request_orders_item_prescriptions_item_clinical_observations_item_value_one(
        &self,
    ) -> Option<&CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemValueOne>
    {
        match self {
                    Self::CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemValueOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_create_order_batch_request_orders_item_prescriptions_item_clinical_observations_item_value_one(
        self,
    ) -> Option<CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemValueOne>
    {
        match self {
                    Self::CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemValueOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display
    for CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemValue
{
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::CreateOrderBatchRequestOrdersItemPrescriptionsItemClinicalObservationsItemValueOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
