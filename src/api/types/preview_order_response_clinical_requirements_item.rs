pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub struct PreviewOrderResponseClinicalRequirementsItem {
    #[serde(default)]
    pub field: String,
    #[serde(default)]
    pub label: String,
    pub r#type: PreviewOrderResponseClinicalRequirementsItemType,
    #[serde(default)]
    pub required: bool,
    pub status: PreviewOrderResponseClinicalRequirementsItemStatus,
}

impl PreviewOrderResponseClinicalRequirementsItem {
    pub fn builder() -> PreviewOrderResponseClinicalRequirementsItemBuilder {
        <PreviewOrderResponseClinicalRequirementsItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseClinicalRequirementsItemBuilder {
    field: Option<String>,
    label: Option<String>,
    r#type: Option<PreviewOrderResponseClinicalRequirementsItemType>,
    required: Option<bool>,
    status: Option<PreviewOrderResponseClinicalRequirementsItemStatus>,
}

impl PreviewOrderResponseClinicalRequirementsItemBuilder {
    pub fn field(mut self, value: impl Into<String>) -> Self {
        self.field = Some(value.into());
        self
    }

    pub fn label(mut self, value: impl Into<String>) -> Self {
        self.label = Some(value.into());
        self
    }

    pub fn r#type(mut self, value: PreviewOrderResponseClinicalRequirementsItemType) -> Self {
        self.r#type = Some(value);
        self
    }

    pub fn required(mut self, value: bool) -> Self {
        self.required = Some(value);
        self
    }

    pub fn status(mut self, value: PreviewOrderResponseClinicalRequirementsItemStatus) -> Self {
        self.status = Some(value);
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponseClinicalRequirementsItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`field`](PreviewOrderResponseClinicalRequirementsItemBuilder::field)
    /// - [`label`](PreviewOrderResponseClinicalRequirementsItemBuilder::label)
    /// - [`r#type`](PreviewOrderResponseClinicalRequirementsItemBuilder::r#type)
    /// - [`required`](PreviewOrderResponseClinicalRequirementsItemBuilder::required)
    /// - [`status`](PreviewOrderResponseClinicalRequirementsItemBuilder::status)
    pub fn build(self) -> Result<PreviewOrderResponseClinicalRequirementsItem, BuildError> {
        Ok(PreviewOrderResponseClinicalRequirementsItem {
            field: self
                .field
                .ok_or_else(|| BuildError::missing_field("field"))?,
            label: self
                .label
                .ok_or_else(|| BuildError::missing_field("label"))?,
            r#type: self
                .r#type
                .ok_or_else(|| BuildError::missing_field("r#type"))?,
            required: self
                .required
                .ok_or_else(|| BuildError::missing_field("required"))?,
            status: self
                .status
                .ok_or_else(|| BuildError::missing_field("status"))?,
        })
    }
}
