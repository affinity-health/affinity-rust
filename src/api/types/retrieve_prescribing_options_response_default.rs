pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseDefault {
    #[serde(default)]
    pub directions: String,
    pub format: RetrievePrescribingOptionsResponseDefaultFormat,
    pub source: RetrievePrescribingOptionsResponseDefaultSource,
    #[serde(rename = "structuredSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_sig: Option<RetrievePrescribingOptionsResponseDefaultStructuredSig>,
}

impl RetrievePrescribingOptionsResponseDefault {
    pub fn builder() -> RetrievePrescribingOptionsResponseDefaultBuilder {
        <RetrievePrescribingOptionsResponseDefaultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseDefaultBuilder {
    directions: Option<String>,
    format: Option<RetrievePrescribingOptionsResponseDefaultFormat>,
    source: Option<RetrievePrescribingOptionsResponseDefaultSource>,
    structured_sig: Option<RetrievePrescribingOptionsResponseDefaultStructuredSig>,
}

impl RetrievePrescribingOptionsResponseDefaultBuilder {
    pub fn directions(mut self, value: impl Into<String>) -> Self {
        self.directions = Some(value.into());
        self
    }

    pub fn format(mut self, value: RetrievePrescribingOptionsResponseDefaultFormat) -> Self {
        self.format = Some(value);
        self
    }

    pub fn source(mut self, value: RetrievePrescribingOptionsResponseDefaultSource) -> Self {
        self.source = Some(value);
        self
    }

    pub fn structured_sig(
        mut self,
        value: RetrievePrescribingOptionsResponseDefaultStructuredSig,
    ) -> Self {
        self.structured_sig = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseDefault`].
    /// This method will fail if any of the following fields are not set:
    /// - [`directions`](RetrievePrescribingOptionsResponseDefaultBuilder::directions)
    /// - [`format`](RetrievePrescribingOptionsResponseDefaultBuilder::format)
    /// - [`source`](RetrievePrescribingOptionsResponseDefaultBuilder::source)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseDefault, BuildError> {
        Ok(RetrievePrescribingOptionsResponseDefault {
            directions: self
                .directions
                .ok_or_else(|| BuildError::missing_field("directions"))?,
            format: self
                .format
                .ok_or_else(|| BuildError::missing_field("format"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            structured_sig: self.structured_sig,
        })
    }
}
