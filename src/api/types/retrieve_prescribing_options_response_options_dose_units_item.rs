pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseOptionsDoseUnitsItem {
    #[serde(default)]
    pub label: String,
    pub source: RetrievePrescribingOptionsResponseOptionsDoseUnitsItemSource,
    #[serde(default)]
    pub value: String,
}

impl RetrievePrescribingOptionsResponseOptionsDoseUnitsItem {
    pub fn builder() -> RetrievePrescribingOptionsResponseOptionsDoseUnitsItemBuilder {
        <RetrievePrescribingOptionsResponseOptionsDoseUnitsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseOptionsDoseUnitsItemBuilder {
    label: Option<String>,
    source: Option<RetrievePrescribingOptionsResponseOptionsDoseUnitsItemSource>,
    value: Option<String>,
}

impl RetrievePrescribingOptionsResponseOptionsDoseUnitsItemBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn source(
        mut self,
        value: RetrievePrescribingOptionsResponseOptionsDoseUnitsItemSource,
    ) -> Self {
        self.source = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseOptionsDoseUnitsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](RetrievePrescribingOptionsResponseOptionsDoseUnitsItemBuilder::label)
    /// - [`source`](RetrievePrescribingOptionsResponseOptionsDoseUnitsItemBuilder::source)
    /// - [`value`](RetrievePrescribingOptionsResponseOptionsDoseUnitsItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<RetrievePrescribingOptionsResponseOptionsDoseUnitsItem, BuildError> {
        Ok(RetrievePrescribingOptionsResponseOptionsDoseUnitsItem {
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            source: self
                .source
                .ok_or_else(|| BuildError::missing_field("source"))?,
            value: self
                .value
                .ok_or_else(|| BuildError::missing_field("value"))?,
        })
    }
}
