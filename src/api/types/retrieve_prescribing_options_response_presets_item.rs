pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct RetrievePrescribingOptionsResponsePresetsItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub revision: String,
    pub source: RetrievePrescribingOptionsResponsePresetsItemSource,
    #[serde(default)]
    pub directions: String,
    pub format: RetrievePrescribingOptionsResponsePresetsItemFormat,
    #[serde(rename = "structuredSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_sig: Option<RetrievePrescribingOptionsResponsePresetsItemStructuredSig>,
    #[serde(skip_serializing_if = "Option::is_none")]
    pub quantity: Option<RetrievePrescribingOptionsResponsePresetsItemQuantity>,
    #[serde(rename = "daysSupply")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub days_supply: Option<i64>,
    #[serde(default)]
    pub refills: i64,
}

impl RetrievePrescribingOptionsResponsePresetsItem {
    pub fn builder() -> RetrievePrescribingOptionsResponsePresetsItemBuilder {
        <RetrievePrescribingOptionsResponsePresetsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponsePresetsItemBuilder {
    id: Option<String>,
    revision: Option<String>,
    source: Option<RetrievePrescribingOptionsResponsePresetsItemSource>,
    directions: Option<String>,
    format: Option<RetrievePrescribingOptionsResponsePresetsItemFormat>,
    structured_sig: Option<RetrievePrescribingOptionsResponsePresetsItemStructuredSig>,
    quantity: Option<RetrievePrescribingOptionsResponsePresetsItemQuantity>,
    days_supply: Option<i64>,
    refills: Option<i64>,
}

impl RetrievePrescribingOptionsResponsePresetsItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    pub fn source(mut self, value: RetrievePrescribingOptionsResponsePresetsItemSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn directions(mut self, value: impl Into<String>) -> Self {
        self.directions = Some(value.into());
        self
    }

    pub fn format(mut self, value: RetrievePrescribingOptionsResponsePresetsItemFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn structured_sig(
        mut self,
        value: RetrievePrescribingOptionsResponsePresetsItemStructuredSig,
    ) -> Self {
        self.structured_sig = Some(value);
        self
    }

    pub fn quantity(
        mut self,
        value: RetrievePrescribingOptionsResponsePresetsItemQuantity,
    ) -> Self {
        self.quantity = Some(value);
        self
    }

    pub fn days_supply(mut self, value: i64) -> Self {
        self.days_supply = Some(value);
        self
    }

    pub fn refills(mut self, value: i64) -> Self {
        self.refills = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponsePresetsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RetrievePrescribingOptionsResponsePresetsItemBuilder::id)
    /// - [`revision`](RetrievePrescribingOptionsResponsePresetsItemBuilder::revision)
    /// - [`source`](RetrievePrescribingOptionsResponsePresetsItemBuilder::source)
    /// - [`directions`](RetrievePrescribingOptionsResponsePresetsItemBuilder::directions)
    /// - [`format`](RetrievePrescribingOptionsResponsePresetsItemBuilder::format)
    /// - [`refills`](RetrievePrescribingOptionsResponsePresetsItemBuilder::refills)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponsePresetsItem, BuildError> {
        Ok(RetrievePrescribingOptionsResponsePresetsItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            revision: self
                .revision
                .ok_or_else(|| BuildError::missing_field("revision"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            directions: self
                .directions
                .ok_or_else(|| BuildError::missing_field("directions"))?,
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            structured_sig: self.structured_sig,
            quantity: self.quantity,
            days_supply: self.days_supply,
            refills: self
                .refills
                .ok_or_else(|| BuildError::missing_field("refills"))?,
        })
    }
}
