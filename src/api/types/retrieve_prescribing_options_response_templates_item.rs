pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct RetrievePrescribingOptionsResponseTemplatesItem {
    #[serde(default)]
    pub id: String,
    #[serde(default)]
    pub initial: RetrievePrescribingOptionsResponseTemplatesItemInitial,
    #[serde(default)]
    pub label: String,
    #[serde(default)]
    pub preview: String,
    #[serde(default)]
    pub revision: String,
}

impl RetrievePrescribingOptionsResponseTemplatesItem {
    pub fn builder() -> RetrievePrescribingOptionsResponseTemplatesItemBuilder {
        <RetrievePrescribingOptionsResponseTemplatesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct RetrievePrescribingOptionsResponseTemplatesItemBuilder {
    id: Option<String>,
    initial: Option<RetrievePrescribingOptionsResponseTemplatesItemInitial>,
    label: Option<String>,
    preview: Option<String>,
    revision: Option<String>,
}

impl RetrievePrescribingOptionsResponseTemplatesItemBuilder {
    pub fn id(mut self, value: impl Into<String>) -> Self {
        self.id = Some(value.into());
        self
    }

    pub fn initial(
        mut self,
        value: RetrievePrescribingOptionsResponseTemplatesItemInitial,
    ) -> Self {
        self.initial = Some(value);
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn preview(mut self, value: impl Into<String>) -> Self {
        self.preview = Some(value.into());
        self
    }

    pub fn revision(mut self, value: impl Into<String>) -> Self {
        self.revision = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`RetrievePrescribingOptionsResponseTemplatesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`id`](RetrievePrescribingOptionsResponseTemplatesItemBuilder::id)
    /// - [`initial`](RetrievePrescribingOptionsResponseTemplatesItemBuilder::initial)
    /// - [`label`](RetrievePrescribingOptionsResponseTemplatesItemBuilder::label)
    /// - [`preview`](RetrievePrescribingOptionsResponseTemplatesItemBuilder::preview)
    /// - [`revision`](RetrievePrescribingOptionsResponseTemplatesItemBuilder::revision)
    pub fn build(self) -> Result<RetrievePrescribingOptionsResponseTemplatesItem, BuildError> {
        Ok(RetrievePrescribingOptionsResponseTemplatesItem {
            id: self.id.ok_or_else(|| BuildError::missing_field("id"))?,
            initial: self
                .initial
                .ok_or_else(|| BuildError::missing_field("initial"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            preview: self
                .preview
                .ok_or_else(|| BuildError::missing_field("preview"))?,
            revision: self
                .revision
                .ok_or_else(|| BuildError::missing_field("revision"))?,
        })
    }
}
