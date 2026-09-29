pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseCatalogQuantityConstraintChoicesQuantitiesItem {
    #[serde(default)]
    pub value: String,
    #[serde(default)]
    pub unit: String,
}

impl RetrievePrescribingOptionsResponseCatalogQuantityConstraintChoicesQuantitiesItem {
    pub fn builder(
    ) -> RetrievePrescribingOptionsResponseCatalogQuantityConstraintChoicesQuantitiesItemBuilder
    {
        <RetrievePrescribingOptionsResponseCatalogQuantityConstraintChoicesQuantitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogQuantityConstraintChoicesQuantitiesItemBuilder {
    value: Option<String>,
    unit: Option<String>,
}

impl RetrievePrescribingOptionsResponseCatalogQuantityConstraintChoicesQuantitiesItemBuilder {
    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogQuantityConstraintChoicesQuantitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`value`](RetrievePrescribingOptionsResponseCatalogQuantityConstraintChoicesQuantitiesItemBuilder::value)
    /// - [`unit`](RetrievePrescribingOptionsResponseCatalogQuantityConstraintChoicesQuantitiesItemBuilder::unit)
    pub fn build(
        self,
    ) -> Result<
        RetrievePrescribingOptionsResponseCatalogQuantityConstraintChoicesQuantitiesItem,
        BuildError,
    > {
        Ok(
            RetrievePrescribingOptionsResponseCatalogQuantityConstraintChoicesQuantitiesItem {
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
                unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            },
        )
    }
}
