pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemValue {
    Double(f64),

    ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemValueOne(
        ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemValueOne,
    ),
}

impl ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemValue {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_list_orders_response_data_item_prescriptions_item_clinical_observations_item_value_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemValueOne(_)
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

    pub fn as_list_orders_response_data_item_prescriptions_item_clinical_observations_item_value_one(
        &self,
    ) -> Option<&ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemValueOne> {
        match self {
            Self::ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemValueOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn into_list_orders_response_data_item_prescriptions_item_clinical_observations_item_value_one(
        self,
    ) -> Option<ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemValueOne> {
        match self {
            Self::ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemValueOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::ListOrdersResponseDataItemPrescriptionsItemClinicalObservationsItemValueOne(
                value,
            ) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
