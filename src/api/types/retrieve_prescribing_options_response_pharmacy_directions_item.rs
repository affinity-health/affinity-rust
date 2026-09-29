pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponsePharmacyDirectionsItem {
    #[serde(default)]
    pub directions: String,
    pub format: RetrievePrescribingOptionsResponsePharmacyDirectionsItemFormat,
    #[serde(rename = "structuredSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_sig:
        Option<RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSig>,
}

impl RetrievePrescribingOptionsResponsePharmacyDirectionsItem {
    pub fn builder() -> RetrievePrescribingOptionsResponsePharmacyDirectionsItemBuilder {
        <RetrievePrescribingOptionsResponsePharmacyDirectionsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponsePharmacyDirectionsItemBuilder {
    directions: Option<String>,
    format: Option<RetrievePrescribingOptionsResponsePharmacyDirectionsItemFormat>,
    structured_sig: Option<RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSig>,
}

impl RetrievePrescribingOptionsResponsePharmacyDirectionsItemBuilder {
    pub fn directions(mut self, value: impl Into<String>) -> Self {
        self.directions = Some(value.into());
        self
    }

    pub fn format(
        mut self,
        value: RetrievePrescribingOptionsResponsePharmacyDirectionsItemFormat,
    ) -> Self {
        self.format = Some(value);
        self
    }

    pub fn structured_sig(
        mut self,
        value: RetrievePrescribingOptionsResponsePharmacyDirectionsItemStructuredSig,
    ) -> Self {
        self.structured_sig = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponsePharmacyDirectionsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`directions`](RetrievePrescribingOptionsResponsePharmacyDirectionsItemBuilder::directions)
    /// - [`format`](RetrievePrescribingOptionsResponsePharmacyDirectionsItemBuilder::format)
    pub fn build(
        self,
    ) -> Result<RetrievePrescribingOptionsResponsePharmacyDirectionsItem, BuildError> {
        Ok(RetrievePrescribingOptionsResponsePharmacyDirectionsItem {
            directions: self
                .directions
                .ok_or_else(|| BuildError::missing_field("directions"))?,
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            structured_sig: self.structured_sig,
        })
    }
}
