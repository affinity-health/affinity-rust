pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrement {
    #[serde(skip_serializing_if = "Option::is_none")]
    pub max: Option<
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMax,
    >,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub min: Option<
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMin,
    >,
    #[serde(default)]
    pub unit: String,
    pub value:
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementValue,
}

impl RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrement {
    pub fn builder(
    ) -> RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementBuilder
    {
        <RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementBuilder
{
    max: Option<
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMax,
    >,
    min: Option<
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMin,
    >,
    unit: Option<String>,
    value: Option<
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementValue,
    >,
}

impl RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementBuilder {
    pub fn max(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMax,
    ) -> Self {
        self.max = Some(value);
        self
    }

    pub fn min(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementMin,
    ) -> Self {
        self.min = Some(value);
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn value(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrement`].
    /// This method will fail if any of the following fields are not set:
    /// - [`unit`](RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementBuilder::unit)
    /// - [`value`](RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrementBuilder::value)
    pub fn build(
        self,
    ) -> Result<
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrement,
        BuildError,
    > {
        Ok(
            RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsQuantityIncrement {
                max: self.max,
                min: self.min,
                unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
            },
        )
    }
}
