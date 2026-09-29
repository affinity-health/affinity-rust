pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantity {
    #[serde(default)]
    pub unit: String,
    pub value:
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityValue,
}

impl RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantity {
    pub fn builder(
    ) -> RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityBuilder
    {
        <RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityBuilder {
    unit: Option<String>,
    value: Option<
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityValue,
    >,
}

impl RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityBuilder {
    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn value(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantity`].
    /// This method will fail if any of the following fields are not set:
    /// - [`unit`](RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityBuilder::unit)
    /// - [`value`](RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantityBuilder::value)
    pub fn build(
        self,
    ) -> Result<
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantity,
        BuildError,
    > {
        Ok(
            RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsDefaultQuantity {
                unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
                value: self
                    .value
                    .ok_or_else(|| BuildError::missing_field("value"))?,
            },
        )
    }
}
