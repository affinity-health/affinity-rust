pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseCatalogOrdering {
    #[serde(rename = "requiresPrescription")]
    #[serde(default)]
    pub requires_prescription: bool,
    #[serde(rename = "requiresAccompanyingPrescription")]
    #[serde(default)]
    pub requires_accompanying_prescription: bool,
    pub shipping: RetrievePrescribingOptionsResponseCatalogOrderingShipping,
}

impl RetrievePrescribingOptionsResponseCatalogOrdering {
    pub fn builder() -> RetrievePrescribingOptionsResponseCatalogOrderingBuilder {
        <RetrievePrescribingOptionsResponseCatalogOrderingBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseCatalogOrderingBuilder {
    requires_prescription: Option<bool>,
    requires_accompanying_prescription: Option<bool>,
    shipping: Option<RetrievePrescribingOptionsResponseCatalogOrderingShipping>,
}

impl RetrievePrescribingOptionsResponseCatalogOrderingBuilder {
    pub fn requires_prescription(mut self, value: bool) -> Self {
        self.requires_prescription = Some(value);
        self
    }

    pub fn requires_accompanying_prescription(mut self, value: bool) -> Self {
        self.requires_accompanying_prescription = Some(value);
        self
    }

    pub fn shipping(
        mut self,
        value: RetrievePrescribingOptionsResponseCatalogOrderingShipping,
    ) -> Self {
        self.shipping = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseCatalogOrdering`].
    /// This method will fail if any of the following fields are not set:
    /// - [`requires_prescription`](RetrievePrescribingOptionsResponseCatalogOrderingBuilder::requires_prescription)
    /// - [`requires_accompanying_prescription`](RetrievePrescribingOptionsResponseCatalogOrderingBuilder::requires_accompanying_prescription)
    /// - [`shipping`](RetrievePrescribingOptionsResponseCatalogOrderingBuilder::shipping)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseCatalogOrdering, BuildError> {
        Ok(RetrievePrescribingOptionsResponseCatalogOrdering {
            requires_prescription: self
                .requires_prescription
                .ok_or_else(|| BuildError::missing_field("requires_prescription"))?,
            requires_accompanying_prescription: self
                .requires_accompanying_prescription
                .ok_or_else(|| BuildError::missing_field("requires_accompanying_prescription"))?,
            shipping: self
                .shipping
                .ok_or_else(|| BuildError::missing_field("shipping"))?,
        })
    }
}
