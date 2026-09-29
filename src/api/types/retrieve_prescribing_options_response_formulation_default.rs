pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseFormulationDefault {
    #[serde(default)]
    pub directions: String,
    pub format: RetrievePrescribingOptionsResponseFormulationDefaultFormat,
    #[serde(rename = "structuredSig")]
    #[serde(skip_serializing_if = "Option::is_none")]
    pub structured_sig: Option<RetrievePrescribingOptionsResponseFormulationDefaultStructuredSig>,
}

impl RetrievePrescribingOptionsResponseFormulationDefault {
    pub fn builder() -> RetrievePrescribingOptionsResponseFormulationDefaultBuilder {
        <RetrievePrescribingOptionsResponseFormulationDefaultBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseFormulationDefaultBuilder {
    directions: Option<String>,
    format: Option<RetrievePrescribingOptionsResponseFormulationDefaultFormat>,
    structured_sig: Option<RetrievePrescribingOptionsResponseFormulationDefaultStructuredSig>,
}

impl RetrievePrescribingOptionsResponseFormulationDefaultBuilder {
    pub fn directions(mut self, value: impl Into<String>) -> Self {
        self.directions = Some(value.into());
        self
    }

    pub fn format(
        mut self,
        value: RetrievePrescribingOptionsResponseFormulationDefaultFormat,
    ) -> Self {
        self.format = Some(value);
        self
    }

    pub fn structured_sig(
        mut self,
        value: RetrievePrescribingOptionsResponseFormulationDefaultStructuredSig,
    ) -> Self {
        self.structured_sig = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseFormulationDefault`].
    /// This method will fail if any of the following fields are not set:
    /// - [`directions`](RetrievePrescribingOptionsResponseFormulationDefaultBuilder::directions)
    /// - [`format`](RetrievePrescribingOptionsResponseFormulationDefaultBuilder::format)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseFormulationDefault, BuildError> {
        Ok(RetrievePrescribingOptionsResponseFormulationDefault {
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
