pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseOptionsRoutesItem {
    #[serde(default)]
    pub label: String,
    pub source: RetrievePrescribingOptionsResponseOptionsRoutesItemSource,
    #[serde(default)]
    pub value: String,
}

impl RetrievePrescribingOptionsResponseOptionsRoutesItem {
    pub fn builder() -> RetrievePrescribingOptionsResponseOptionsRoutesItemBuilder {
        <RetrievePrescribingOptionsResponseOptionsRoutesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseOptionsRoutesItemBuilder {
    label: Option<String>,
    source: Option<RetrievePrescribingOptionsResponseOptionsRoutesItemSource>,
    value: Option<String>,
}

impl RetrievePrescribingOptionsResponseOptionsRoutesItemBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn source(
        mut self,
        value: RetrievePrescribingOptionsResponseOptionsRoutesItemSource,
    ) -> Self {
        self.source = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseOptionsRoutesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](RetrievePrescribingOptionsResponseOptionsRoutesItemBuilder::label)
    /// - [`source`](RetrievePrescribingOptionsResponseOptionsRoutesItemBuilder::source)
    /// - [`value`](RetrievePrescribingOptionsResponseOptionsRoutesItemBuilder::value)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseOptionsRoutesItem, BuildError> {
        Ok(RetrievePrescribingOptionsResponseOptionsRoutesItem {
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
