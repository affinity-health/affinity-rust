pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct PreviewOrderResponsePrescriptionsItemQuantity {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub value: f64,
    #[serde(default)]
    pub unit: String,
}

impl PreviewOrderResponsePrescriptionsItemQuantity {
    pub fn builder() -> PreviewOrderResponsePrescriptionsItemQuantityBuilder {
        <PreviewOrderResponsePrescriptionsItemQuantityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponsePrescriptionsItemQuantityBuilder {
    value: Option<f64>,
    unit: Option<String>,
}

impl PreviewOrderResponsePrescriptionsItemQuantityBuilder {
    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponsePrescriptionsItemQuantity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](PreviewOrderResponsePrescriptionsItemQuantityBuilder::value)
    /// - [`unit`](PreviewOrderResponsePrescriptionsItemQuantityBuilder::unit)
    pub fn build(self) -> Result<PreviewOrderResponsePrescriptionsItemQuantity, BuildError> {
        Ok(PreviewOrderResponsePrescriptionsItemQuantity {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
        })
    }
}
