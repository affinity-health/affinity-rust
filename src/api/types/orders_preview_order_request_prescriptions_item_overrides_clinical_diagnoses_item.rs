pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub display: String,
}

impl PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosesItem {
    pub fn builder() -> PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosesItemBuilder {
        <PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosesItemBuilder {
    code: Option<String>,
    display: Option<String>,
}

impl PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn display(mut self, value: impl Into<String>) -> Self {
        self.display = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosesItemBuilder::code)
    /// - [`display`](PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosesItemBuilder::display)
    pub fn build(
        self,
    ) -> Result<PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosesItem, BuildError>
    {
        Ok(
            PreviewOrderRequestPrescriptionsItemOverridesClinicalDiagnosesItem {
                code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
                display: self
                    .display
                    .ok_or_else(|| BuildError::missing_field("display"))?,
            },
        )
    }
}
