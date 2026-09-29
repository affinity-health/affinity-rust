pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseCatalogQuantityConstraintFixedQuantity {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub unit: String,
}

impl RetrievePrescribingOptionsResponseCatalogQuantityConstraintFixedQuantity {
    pub fn builder(
    ) -> RetrievePrescribingOptionsResponseCatalogQuantityConstraintFixedQuantityBuilder {
        <RetrievePrescribingOptionsResponseCatalogQuantityConstraintFixedQuantityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogQuantityConstraintFixedQuantityBuilder {
    value: Option<String>,
    unit: Option<String>,
}

impl RetrievePrescribingOptionsResponseCatalogQuantityConstraintFixedQuantityBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogQuantityConstraintFixedQuantity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](RetrievePrescribingOptionsResponseCatalogQuantityConstraintFixedQuantityBuilder::value)
    /// - [`unit`](RetrievePrescribingOptionsResponseCatalogQuantityConstraintFixedQuantityBuilder::unit)
    pub fn build(
        self,
    ) -> Result<RetrievePrescribingOptionsResponseCatalogQuantityConstraintFixedQuantity, BuildError>
    {
        Ok(
            RetrievePrescribingOptionsResponseCatalogQuantityConstraintFixedQuantity {
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
                unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            },
        )
    }
}
