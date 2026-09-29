pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub display: String,
}

impl PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosesItem {
    pub fn builder() -> PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosesItemBuilder
    {
        <PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosesItemBuilder {
    code: Option<String>,
    display: Option<String>,
}

impl PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn display(mut self, value: impl Into<String>) -> Self {
        self.display = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosesItemBuilder::code)
    /// - [`display`](PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosesItemBuilder::display)
    pub fn build(
        self,
    ) -> Result<PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosesItem, BuildError>
    {
        Ok(
            PreviewOrderResponseOrderInputPrescriptionsItemClinicalDiagnosesItem {
                code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
                display: self
                    .display
                    .ok_or_else(|| BuildError::missing_field("display"))?,
            },
        )
    }
}
