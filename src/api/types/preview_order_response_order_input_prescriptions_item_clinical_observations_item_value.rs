pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemValue {
    Double(f64),

    PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemValueOne(
        PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemValueOne,
    ),
}

impl PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemValue {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_preview_order_response_order_input_prescriptions_item_clinical_observations_item_value_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemValueOne(
                _
            )
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

    pub fn as_preview_order_response_order_input_prescriptions_item_clinical_observations_item_value_one(
        &self,
    ) -> Option<&PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemValueOne>
    {
        match self {
                    Self::PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemValueOne(value) => Some(value),
                    _ => None,
                }
    }

    pub fn into_preview_order_response_order_input_prescriptions_item_clinical_observations_item_value_one(
        self,
    ) -> Option<PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemValueOne>
    {
        match self {
                    Self::PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemValueOne(value) => Some(value),
                    _ => None,
                }
    }
}

impl fmt::Display for PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::PreviewOrderResponseOrderInputPrescriptionsItemClinicalObservationsItemValueOne(value) => write!(f, "{}", serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))),
        }
    }
}
