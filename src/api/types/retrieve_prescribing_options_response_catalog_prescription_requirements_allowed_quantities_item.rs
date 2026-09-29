pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItem {
    #[serde(rename = "daysSupply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_supply: Option<i64>,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub unit: String,
    pub value:
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemValue,
}

impl RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItem {
    pub fn builder(
    ) -> RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemBuilder
    {
        <RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemBuilder
{
    days_supply: Option<i64>,
    label: Option<String>,
    unit: Option<String>,
    value: Option<
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemValue,
    >,
}

impl RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemBuilder {
    pub fn days_supply(mut self, value: i64) -> Self {
        self.days_supply = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn unit(mut self, value: impl Into<String>) -> Self {
        self.unit = Some(value.into());
        self
    }

    pub fn value(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemValue,
    ) -> Self {
        self.value = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemBuilder::label)
    /// - [`unit`](RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemBuilder::unit)
    /// - [`value`](RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<
        RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItem,
        BuildError,
    > {
        Ok(RetrievePrescribingOptionsResponseCatalogPrescriptionRequirementsAllowedQuantitiesItem {
            days_supply: self.days_supply,
            label: self.label.ok_or_else(|| BuildError::missing_field("label"))?,
            unit: self.unit.ok_or_else(|| BuildError::missing_field("unit"))?,
            value: self.value.ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
