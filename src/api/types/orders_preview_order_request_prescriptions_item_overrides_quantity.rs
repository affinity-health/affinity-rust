pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PreviewOrderRequestPrescriptionsItemOverridesQuantity {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub value: f64,
    #[serde(default)]
    pub unit: String,
}

impl PreviewOrderRequestPrescriptionsItemOverridesQuantity {
    pub fn builder() -> PreviewOrderRequestPrescriptionsItemOverridesQuantityBuilder {
        <PreviewOrderRequestPrescriptionsItemOverridesQuantityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderRequestPrescriptionsItemOverridesQuantityBuilder {
    value: Option<f64>,
    unit: Option<String>,
}

impl PreviewOrderRequestPrescriptionsItemOverridesQuantityBuilder {
    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderRequestPrescriptionsItemOverridesQuantity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](PreviewOrderRequestPrescriptionsItemOverridesQuantityBuilder::value)
    /// - [`unit`](PreviewOrderRequestPrescriptionsItemOverridesQuantityBuilder::unit)
    pub fn build(
        self,
    ) -> Result<PreviewOrderRequestPrescriptionsItemOverridesQuantity, BuildError> {
        Ok(PreviewOrderRequestPrescriptionsItemOverridesQuantity {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
        })
    }
}
