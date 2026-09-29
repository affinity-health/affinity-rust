pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseOptionsDosesItem {
    #[serde(default)]
    pub label: String,
    pub source: RetrievePrescribingOptionsResponseOptionsDosesItemSource,
    #[serde(default)]
    pub value: String,
}

impl RetrievePrescribingOptionsResponseOptionsDosesItem {
    pub fn builder() -> RetrievePrescribingOptionsResponseOptionsDosesItemBuilder {
        <RetrievePrescribingOptionsResponseOptionsDosesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseOptionsDosesItemBuilder {
    label: Option<String>,
    source: Option<RetrievePrescribingOptionsResponseOptionsDosesItemSource>,
    value: Option<String>,
}

impl RetrievePrescribingOptionsResponseOptionsDosesItemBuilder {
    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn source(
        mut self,
        value: RetrievePrescribingOptionsResponseOptionsDosesItemSource,
    ) -> Self {
        self.source = Some(value);
        self
    }

    pub fn value(mut self, value: impl Into<String>) -> Self {
        self.value = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseOptionsDosesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`label`](RetrievePrescribingOptionsResponseOptionsDosesItemBuilder::label)
    /// - [`source`](RetrievePrescribingOptionsResponseOptionsDosesItemBuilder::source)
    /// - [`value`](RetrievePrescribingOptionsResponseOptionsDosesItemBuilder::value)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseOptionsDosesItem, BuildError> {
        Ok(RetrievePrescribingOptionsResponseOptionsDosesItem {
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
