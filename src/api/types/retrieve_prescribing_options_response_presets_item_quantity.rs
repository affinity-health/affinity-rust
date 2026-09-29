pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq)]
pub struct RetrievePrescribingOptionsResponsePresetsItemQuantity {
    #[serde(default)]
    #[serde(with = "crate::core::number_serializers")]
    pub value: f64,
    #[serde(default)]
    pub unit: String,
}

impl RetrievePrescribingOptionsResponsePresetsItemQuantity {
    pub fn builder() -> RetrievePrescribingOptionsResponsePresetsItemQuantityBuilder {
        <RetrievePrescribingOptionsResponsePresetsItemQuantityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponsePresetsItemQuantityBuilder {
    value: Option<f64>,
    unit: Option<String>,
}

impl RetrievePrescribingOptionsResponsePresetsItemQuantityBuilder {
    pub fn value(mut self, value: f64) -> Self {
        self.value = Some(value);
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponsePresetsItemQuantity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](RetrievePrescribingOptionsResponsePresetsItemQuantityBuilder::value)
    /// - [`unit`](RetrievePrescribingOptionsResponsePresetsItemQuantityBuilder::unit)
    pub fn build(
        self,
    ) -> Result<RetrievePrescribingOptionsResponsePresetsItemQuantity, BuildError> {
        Ok(RetrievePrescribingOptionsResponsePresetsItemQuantity {
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
        })
    }
}
