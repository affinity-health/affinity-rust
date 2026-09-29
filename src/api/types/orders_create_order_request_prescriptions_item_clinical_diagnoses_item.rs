pub use crate::prelude::*;

#[derive(Debug, Clone, Serialize, Deserialize, Default, PartialEq, Eq, Hash)]
pub struct CreateOrderRequestPrescriptionsItemClinicalDiagnosesItem {
    #[serde(default)]
    pub code: String,
    #[serde(default)]
    pub display: String,
}

impl CreateOrderRequestPrescriptionsItemClinicalDiagnosesItem {
    pub fn builder() -> CreateOrderRequestPrescriptionsItemClinicalDiagnosesItemBuilder {
        <CreateOrderRequestPrescriptionsItemClinicalDiagnosesItemBuilder as Default>::default()
    }
}

#[derive(Clone, PartialEq, Default, Debug)]
#[non_exhaustive]
pub struct CreateOrderRequestPrescriptionsItemClinicalDiagnosesItemBuilder {
    code: Option<String>,
    display: Option<String>,
}

impl CreateOrderRequestPrescriptionsItemClinicalDiagnosesItemBuilder {
    pub fn code(mut self, value: impl Into<String>) -> Self {
        self.code = Some(value.into());
        self
    }

    pub fn display(mut self, value: impl Into<String>) -> Self {
        self.display = Some(value.into());
        self
    }

    /// Consumes the builder and constructs a [`CreateOrderRequestPrescriptionsItemClinicalDiagnosesItem`].
    /// This method will fail if any of the following fields are not set:
    /// - [`code`](CreateOrderRequestPrescriptionsItemClinicalDiagnosesItemBuilder::code)
    /// - [`display`](CreateOrderRequestPrescriptionsItemClinicalDiagnosesItemBuilder::display)
    pub fn build(
        self,
    ) -> Result<CreateOrderRequestPrescriptionsItemClinicalDiagnosesItem, BuildError> {
        Ok(CreateOrderRequestPrescriptionsItemClinicalDiagnosesItem {
            code: self.code.ok_or_else(|| BuildError::missing_field("code"))?,
            display: self
                .display
                .ok_or_else(|| BuildError::missing_field("display"))?,
        })
    }
}
