pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrievePrescribingOptionsResponseCatalogMedicationGroup {
    #[serde(rename = "offerCount")]
    pub offer_count: RetrievePrescribingOptionsResponseCatalogMedicationGroupOfferCount,
    #[serde(rename = "pharmacyCount")]
    pub pharmacy_count: RetrievePrescribingOptionsResponseCatalogMedicationGroupPharmacyCount,
    #[serde(default)]
    pub strengths: Vec<String>,
}

impl RetrievePrescribingOptionsResponseCatalogMedicationGroup {
    pub fn builder() -> RetrievePrescribingOptionsResponseCatalogMedicationGroupBuilder {
        <RetrievePrescribingOptionsResponseCatalogMedicationGroupBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogMedicationGroupBuilder {
    offer_count: Option<RetrievePrescribingOptionsResponseCatalogMedicationGroupOfferCount>,
    pharmacy_count: Option<RetrievePrescribingOptionsResponseCatalogMedicationGroupPharmacyCount>,
    strengths: Option<Vec<String>>,
}

impl RetrievePrescribingOptionsResponseCatalogMedicationGroupBuilder {
    pub fn offer_count(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogMedicationGroupOfferCount,
    ) -> Self {
        self.offer_count = Some(value);
        self
    }

    pub fn pharmacy_count(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogMedicationGroupPharmacyCount,
    ) -> Self {
        self.pharmacy_count = Some(value);
        self
    }

    pub fn strengths(mut self, value: Vec<String>) -> Self {
        self.strengths = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogMedicationGroup`].
    /// This method will fail if any of the following fields are not set:
    /// - [`offer_count`](RetrievePrescribingOptionsResponseCatalogMedicationGroupBuilder::offer_count)
    /// - [`pharmacy_count`](RetrievePrescribingOptionsResponseCatalogMedicationGroupBuilder::pharmacy_count)
    /// - [`strengths`](RetrievePrescribingOptionsResponseCatalogMedicationGroupBuilder::strengths)
    pub fn build(
        self,
    ) -> Result<RetrievePrescribingOptionsResponseCatalogMedicationGroup, BuildError> {
        Ok(RetrievePrescribingOptionsResponseCatalogMedicationGroup {
            offer_count: self
                .offer_count
                .ok_or_else(|| BuildError::missing_field("offer_count"))?,
            pharmacy_count: self
                .pharmacy_count
                .ok_or_else(|| BuildError::missing_field("pharmacy_count"))?,
            strengths: self
                .strengths
                .ok_or_else(|| BuildError::missing_field("strengths"))?,
        })
    }
}
