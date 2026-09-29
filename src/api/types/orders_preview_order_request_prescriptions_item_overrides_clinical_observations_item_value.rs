pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
#[serde(untagged)]
pub enum PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemValue {
    Double(f64),

    PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemValueOne(
        PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemValueOne,
    ),
}

impl PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemValue {
    pub fn is_double(&self) -> bool {
        matches!(self, Self::Double(_))
    }

    pub fn is_preview_order_request_prescriptions_item_overrides_clinical_observations_item_value_one(
        &self,
    ) -> bool {
        matches!(
            self,
            Self::PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemValueOne(_)
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

    pub fn as_preview_order_request_prescriptions_item_overrides_clinical_observations_item_value_one(
        &self,
    ) -> Option<&PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemValueOne>
    {
        match self {
            Self::PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemValueOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }

    pub fn into_preview_order_request_prescriptions_item_overrides_clinical_observations_item_value_one(
        self,
    ) -> Option<PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemValueOne> {
        match self {
            Self::PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemValueOne(
                value,
            ) => Some(value),
            _ => None,
        }
    }
}

impl fmt::Display for PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemValue {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::Double(value) => write!(f, "{}", value),
            Self::PreviewOrderRequestPrescriptionsItemOverridesClinicalObservationsItemValueOne(
                value,
            ) => write!(
                f,
                "{}",
                serde_json::to_string(value).unwrap_or_else(|_| format!("{:?}", value))
            ),
        }
    }
}
