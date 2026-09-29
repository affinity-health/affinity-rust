pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseOptionsFrequenciesItem {
    #[serde(default)]
    pub label: String,
    pub source: RetrievePrescribingOptionsResponseOptionsFrequenciesItemSource,
    #[serde(default)]
    pub value: String,
}

impl RetrievePrescribingOptionsResponseOptionsFrequenciesItem {
    pub fn builder() -> RetrievePrescribingOptionsResponseOptionsFrequenciesItemBuilder {
        <RetrievePrescribingOptionsResponseOptionsFrequenciesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseOptionsFrequenciesItemBuilder {
    label: Option<String>,
    source: Option<RetrievePrescribingOptionsResponseOptionsFrequenciesItemSource>,
    value: Option<String>,
}

impl RetrievePrescribingOptionsResponseOptionsFrequenciesItemBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn source(
        mut self,
        value: RetrievePrescribingOptionsResponseOptionsFrequenciesItemSource,
    ) -> Self {
        self.source = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseOptionsFrequenciesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](RetrievePrescribingOptionsResponseOptionsFrequenciesItemBuilder::label)
    /// - [`source`](RetrievePrescribingOptionsResponseOptionsFrequenciesItemBuilder::source)
    /// - [`value`](RetrievePrescribingOptionsResponseOptionsFrequenciesItemBuilder::value)
    pub fn build(
        self,
    ) -> Result<RetrievePrescribingOptionsResponseOptionsFrequenciesItem, BuildError> {
        Ok(RetrievePrescribingOptionsResponseOptionsFrequenciesItem {
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
